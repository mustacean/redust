mod redust;
use redust::ServiceMetaProvider;

fn main() {
    let host = "127.0.0.1";
    let serv_name = "my_service";
    let events = vec![String::from("added"), String::from("deleted")];

    exec(host, serv_name, events);
}

fn exec(host: &str, serv_name: &str, events: Vec<String>) {
    let sp = ServiceMetaProvider::provide(host);

    match sp.clone().get_service(serv_name) {
        Ok(service) => {
            println!(
                "SERVICE-> \nname : {}\nhost: {}",
                service.get_name(),
                service.get_host()
            );
            let evs = service.get_events();
            for e in evs {
                println!("[event : {}]", e.get_name());
            }

            if let Ok(_) = sp.clone().remove_service(serv_name) {
                println!("service succesfuly removed!");
            }
        }
        Err(()) => {
            println!("couldn't find the service. create? (Y/N)?");

            if let Ok(_) = sp.clone().add_service(serv_name, &host, events) {
                println!("service succesfuly added!");
            }
        }
    }
}
