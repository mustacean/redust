use crate::service::Event;
use std::io::BufReader;
use uuid::Uuid;

type Message = (Uuid, String, usize);

fn build_messages(mut bfreader: BufReader<&std::fs::File>, cap: usize) -> Vec<Message> {
    use std::io::Read;
    let id = Uuid::new_v4();

    let mut x = 0;
    let mut messages: Vec<Message> = Vec::with_capacity(cap);

    loop {
        let mut my_string = String::with_capacity(1024);

        if let Ok(i) = bfreader.read_to_string(&mut my_string) {
            messages.push((id, my_string, i));
            x += 1;
            if x >= cap {
                break;
            }
        }
    }
    return messages;
}

fn noinvoke(ev: Event, id: Uuid, msg: &str) {
    println!("{}", ev.get_name());
    println!("\nid : {}", id);
    println!("{}", msg);
}

#[test]
fn test_buffer_read() {
    let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    let bfreader = std::io::BufReader::new(&file);
    for (id, payload, _) in build_messages(bfreader, 2usize) {
        noinvoke(Event::new("unknown", "demo_event"), id, &payload);
        assert_ne!(String::new(), payload);
    }
}
