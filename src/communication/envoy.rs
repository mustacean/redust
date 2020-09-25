use crate::communication::Caster;
use crate::service::Event;
use crate::service::Service;
use crate::service::ServiceMetaProvider;
use std::io::Read;
use uuid::Uuid;

struct Envoy {
    message_id: Uuid,
}

impl<'t> Envoy {
    fn new(message_id: Uuid) -> Envoy {
        Envoy { message_id }
    }

    fn deliver(&'t self, mut rcon: redis::Connection, mut st: impl Read) {
        use redis::*;
        let mut ct = 0;
        let mut buffer = &mut [0u8; 1024];
        while ct < 3 {
            if let Ok(i) = st.read(buffer) {
                let reslt = rcon
                    .publish::<String, String, i32>(
                        format!("{}", "test" /*self.message_id*/),
                        format!("{}", String::from_utf8_lossy(&buffer[0..i])),
                    )
                    .unwrap();

                println!("sent to '{}' service.", reslt);
            }
            ct += 1;
        }
    }
}

impl<'t> Caster<'t> {
    fn send_via_envoy(&self, e: &Event, st: impl Read + Send + 'static) -> Result<i32, ()> {
        let id = Uuid::new_v4();
        use crate::rd_tools::IRedisClient;
        use crate::service::IServiceOwned;

        if let Ok(i) = self.invoke(e, &format!("{}", id)) {
            let conn = self.get_service().get_provider().get_conn();

            std::thread::spawn(move || {
                let envo = Envoy::new(Uuid::new_v4());
                envo.deliver(conn, st);
            })
            .join()
            .unwrap();
            return Ok(i);
        }
        Err(())
    }
}

#[test]
fn test_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let caster = mca_service.get_caster();
    let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    caster
        .send_via_envoy(mca_service.get_events().first().unwrap(), file)
        .unwrap();
}
