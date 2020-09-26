use crate::communication::Antenna;
use crate::communication::Caster;
use crate::service::Event;
use crate::service::IServiceOwned;
use crate::service::Service;
use crate::service::ServiceMetaProvider;
use std::io::Read;
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
        if let Ok(i) = self
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
    pub fn receive_packets(&'t self, time_out: u32, call_back: &dyn Fn(Event, &str)) {
        use crate::rd_tools::IRedisClient;
        for (e, id) in self.receive() {
            let mut conn = self.get_service().get_provider().get_conn();
            //std::thread::spawn(move || {
            loop {
                if let Ok(re) = redis::cmd("blpop")
                    .arg(id.clone())
                    .arg(time_out)
                    .query::<Option<Vec<String>>>(&mut conn)
                {
                    if let Some(x) = re {
                        (call_back)(e.clone(), &x[x.len() - 1]);
                    //println!("{}", x[x.len() - 1]);
                    } else {
                        break;
                    }
                }
            }
            //});
        }
    }
}

#[test]
fn test_send_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let caster = mca_service.get_caster();
    caster
        .prepare_packets(&mca_service.get_events()[1])
        .send(
            "hello, world!, today it's the day that 
            I be blowin' up like a bubble."
                .as_bytes(),
        )
        .unwrap();
}

#[test]

fn test_receive_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let ante = mca_service.get_antenna(mca_service.get_events());
    ante.launch();
    ante.receive_packets(7, &|e, m| {
        println!("event : {} \npacket : {}", e.to_string(), m);
    });
}
