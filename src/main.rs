mod redust;

use redust::ServiceMetaProvider;

fn main() {
    let sp = ServiceMetaProvider::provide();

    match sp.clone().get_service("my_service") {
        Ok(service) => {
            println!(
                "SERVICE-> \nname : {}\nhost: {}",
                service.get_name(),
                service.get_host()
            );
        }
        Err(()) => {
            println!("couldn't find the service. create? (Y/N)?");
        }
    }

    //let _= sp.clone().get_service("other_service");
}
