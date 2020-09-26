use crate::communication::Antenna;
use crate::communication::Caster;
use crate::service::Event;
use crate::service::IServiceOwned;
use crate::service::Service;
use crate::service::ServiceMetaProvider;
use std::io::Read;
use uuid::Uuid;

const BUFFER_SIZE: usize = 1024;

pub struct Envoy<'t> {
    message_id: Uuid,
    caster: &'t Caster<'t>,
    event: &'t Event,
}

impl<'t> IServiceOwned<'t> for Envoy<'t> {
    fn get_service(&self) -> &'t Service {
        self.caster.get_service()
    }
}

impl<'t> Envoy<'t> {
    fn send_buffers(&'t self, mut st: impl Read + Send + 'static) -> std::thread::JoinHandle<()> {
        use crate::rd_tools::IRedisClient;
        use redis::*;
        let mut conn = self.get_service().get_provider().get_conn();
        let id = format!("{}", self.message_id);
        std::thread::spawn(move || {
            let mut ct = 0;
            let mut buffer = &mut [0u8; BUFFER_SIZE];
            while ct < 3 {
                if let Ok(i) = st.read(buffer) {
                    redis::cmd("rpush")
                        .arg(/*id*/ "test")
                        .arg::<&[u8]>(&mut buffer[0..i])
                        .query::<i32>(&mut conn)
                        .unwrap();
                    if i < buffer.len() {
                        break;
                    }
                }
                ct += 1;
            }
        })
    }

    pub fn send(&'t self, st: impl Read + Send + 'static) -> Result<i32, ()> {
        if let Ok(i) = self
            .caster
            .invoke(self.event, &format!("{}", self.message_id))
        {
            self.send_buffers(st).join().unwrap();
            return Ok(i);
        }
        Err(())
    }
}

impl<'t> Caster<'t> {
    pub fn assign_envoy(&'t self, event: &'t Event) -> Envoy<'t> {
        Envoy::<'t> {
            message_id: Uuid::new_v4(),
            caster: self,
            event,
        }
    }
}

impl<'t> Antenna<'t> {
    pub fn receive_envoy(&'t self, time_out: u32) {
        use crate::rd_tools::IRedisClient;
        use std::iter::*;
        //for (event, id) in self.receive() {
        let mut conn = self.get_service().get_provider().get_conn();
        std::thread::spawn(move || {
            loop {
                if let Ok(re) = redis::cmd("brpop")
                    .arg(/*id*/ "test")
                    .arg(time_out)
                    .query::<Option<Vec<String>>>(&mut conn)
                {
                    if let Some(x) = re {
                        //println!("{}", x.last().unwrap());
                        for i in 0..x.len() {
                            println!("{}", x[i]);
                        }
                    } else {
                        break;
                    }
                }
            }
        })
        .join()
        .unwrap();
        //}
    }
}

#[test]
fn test_send_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let caster = mca_service.get_caster();
    caster
        .assign_envoy(&mca_service.get_events()[1])
        .send(
            "hello, world!, lre werwer ower we krwekrwerrwwr \
        weör wlreöw leörlw eröwler öwleör lewörl öewlrö wqlr öw\
        elröalf  lwpelrwp leprlw perlwpel rpwle prlwep lrpewl prlw\
        e prlwpe lrpwle rplwqepr lqwplr p23l45p25p34p65lo3 4py6l45p \
        lypslrxövlöm şs okgtskrt poakto kapotk pek\
rwerw eqwepqw pewqe qpw wle pqlwe\

 vroewrwoerow erpewrlw ep tlwtwt
 r tre
 te
 rt
 er
 te
 rtrt phtrdğphoğpsdotğph odsğpohğpsotğph rsh posroh
  gtpreogtsre gsses"
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
    ante.receive_envoy(15);
}
