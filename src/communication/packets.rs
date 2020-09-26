use crate::communication::Antenna;
use crate::communication::Caster;
use crate::service::Event;
use crate::service::IServiceOwned;
use crate::service::Service;
use std::io::Read;
use std::io::Write;
use uuid::Uuid;

const BUFFER_SIZE: usize = 64;

pub struct Packets<'t> {
    message_id: Uuid,
    caster: &'t Caster<'t>,
    event: &'t Event,
}

impl<'t> IServiceOwned<'t> for Packets<'t> {
    fn get_service(&self) -> &'t Service {
        self.caster.get_service()
    }
}

impl<'t> Packets<'t> {
    fn send_buffers(
        &'t self,
        mut st: impl Read + Send + 'static,
    ) -> std::thread::JoinHandle<Result<i32, ()>> {
        use crate::rd_tools::IRedisClient;
        let mut conn = self.get_service().get_provider().get_conn();
        let id = format!("{}", self.message_id);
        std::thread::spawn(move || {
            let mut buffer = &mut [0u8; BUFFER_SIZE];
            loop {
                if let Ok(i) = st.read(buffer) {
                    if let Ok(rx) = redis::cmd("rpush")
                        .arg(id.clone())
                        .arg::<&[u8]>(&mut buffer[0..i])
                        .query::<i32>(&mut conn)
                    {
                        if i < buffer.len() {
                            return Ok(rx);
                        }
                    }
                }
            }
        })
    }

    pub fn send(&'t self, st: impl Read + Send + 'static) -> Result<i32, ()> {
        if let Ok(_) = self
            .caster
            .invoke(self.event, &format!("{}", self.message_id))
        {
            self.send_buffers(st).join().unwrap()
        } else {
            Err(())
        }
    }
}

impl<'t> Caster<'t> {
    pub fn prepare_packets(&'t self, event: &'t Event) -> Packets<'t> {
        Packets::<'t> {
            message_id: Uuid::new_v4(),
            caster: self,
            event,
        }
    }
}

impl<'t> Antenna<'t> {
    pub fn receive_packets(
        &'t self,
        id: String,
        time_out: u32,
        mut wr: impl Write + Send + 'static,
    ) {
        use crate::rd_tools::IRedisClient;
        let mut conn = self.get_service().get_provider().get_conn();
        std::thread::spawn(move || loop {
            if let Ok(re) = redis::cmd("blpop")
                .arg(id.clone())
                .arg(time_out)
                .query::<Option<Vec<String>>>(&mut conn)
            {
                if let Some(x) = re {
                    if let Err(_) = wr.write(&x[x.len() - 1].as_bytes()) {
                        break;
                    }
                    wr.flush().unwrap();
                } else {
                    break;
                }
            }
        });
    }
}
