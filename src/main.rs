pub mod communication;
mod rd_tools;
pub mod service;

fn main() {
    let host = "127.0.0.1";
    list_services(host);
}

fn list_services(host: &str) {
    let sp = crate::service::ServiceMetaProvider::provide(host);

    match sp.get_services() {
        Ok(x) => {
            for service in x {
                println!(
                    "\n<< SERVICE-> \nname : {}\nhost: {}",
                    service.get_name(),
                    service.get_host()
                );
                let evs = service.get_events();
                for e in evs {
                    println!("[event : {}]", e.get_name());
                }
                println!(">>");
            }
        }
        Err(()) => {
            println!("ne service found..!");
        }
    }
}
