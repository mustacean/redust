use redis::cmd;
use redis::Connection;
use std::io::Read;
use std::io::Write;
use std::thread::spawn;
use std::thread::JoinHandle;

const BUFFER_SIZE: usize = 64;

pub fn rpush_buffers(
    mut conn: Connection,
    list_name: &'static str,
    mut st: impl Read + Send + 'static,
) -> JoinHandle<Result<i32, ()>> {
    spawn(move || {
        let mut buffer = &mut [0u8; BUFFER_SIZE];
        loop {
            if let Ok(i) = st.read(buffer) {
                if let Ok(rx) = cmd("rpush")
                    .arg(list_name)
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

pub fn blpop_buffers(
    mut conn: Connection,
    list_name: &'static str,
    time_out: u32,
    mut wr: impl Write + Send + 'static,
) -> JoinHandle<()> {
    spawn(move || loop {
        if let Ok(re) = cmd("blpop")
            .arg(list_name)
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
    })
}

fn get_default_con() -> Connection {
    redis::Client::open("redis://127.0.0.1/")
        .unwrap()
        .get_connection()
        .unwrap()
}

#[test]
fn test_blpop() {
    blpop_buffers(get_default_con(), "foo", 5, std::io::stdout())
        .join()
        .unwrap();
}

#[test]
fn test_rpush() {
    rpush_buffers(get_default_con(), "foo", "what's up dude!".as_bytes())
        .join()
        .unwrap()
        .unwrap();
}
