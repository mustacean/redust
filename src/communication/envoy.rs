use crate::communication::Caster;
use crate::service::Event;
use crate::service::IServiceOwned;
use crate::service::Service;
use crate::service::ServiceMetaProvider;
use std::io::Read;
use uuid::Uuid;

struct Envoy<'t> {
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
        let mut conn = self.get_service().get_provider().get_conn();

        std::thread::spawn(move || {
            use redis::*;
            let mut ct = 0;
            let mut buffer = &mut [0u8; 1024];
            while ct < 3 {
                if let Ok(i) = st.read(buffer) {
                    let reslt = conn
                        .publish::<String, &[u8], i32>(
                            format!("{}", "test" /*self.message_id*/),
                            &buffer[0..i],
                        )
                        .unwrap();

                    println!("sent to '{}' service.", reslt);
                }
                ct += 1;
            }
        })
    }

    fn send(&'t self, st: impl Read + Send + 'static) -> Result<i32, ()> {
        if let Ok(i) = self
            .caster
            .invoke(self.event, &format!("{}", self.message_id))
        {
            self.send_buffers(st).join();
            return Ok(i);
        }
        Err(())
    }
}

impl<'t> Caster<'t> {
    fn assign_envoy(&'t self, event: &'t Event) -> Envoy<'t> {
        Envoy::<'t> {
            message_id: Uuid::new_v4(),
            caster: self,
            event,
        }
    }
}

#[test]
fn test_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let caster = mca_service.get_caster();
    let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    caster
        .assign_envoy(&mca_service.get_events()[1])
        .send(file)
        .unwrap();
}
