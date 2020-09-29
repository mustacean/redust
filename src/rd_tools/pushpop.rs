use redis::cmd;
use redis::Connection;
use std::io::Read;
use std::io::Write;
use std::thread::spawn;
use std::thread::JoinHandle;

const BUFFER_SIZE: usize = 64;

pub fn rpush_buffers(
    mut conn: Connection,
    list_name: String,
    mut st: Box<dyn Read + Send>,
) -> JoinHandle<Result<i32, ()>> {
    spawn(move || {
        let buffer = &mut [0u8; BUFFER_SIZE];
        loop {
            if let Ok(i) = st.read(buffer) {
                if let Ok(rx) = cmd("rpush")
                    .arg(list_name.clone().as_str())
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

pub fn rpush_str(
    mut conn: Connection,
    list_name: String,
    st: String,
) -> JoinHandle<Result<i32, ()>> {
    spawn(move || loop {
        if let Ok(rx) = cmd("rpush")
            .arg(&list_name.clone())
            .arg::<&str>(&st)
            .query::<i32>(&mut conn)
        {
            return Ok(rx);
        }
    })
}

pub fn blpop_buffers(
    mut conn: Connection,
    list_name: String,
    time_out: u32,
    mut wr: Box<dyn Write + Send>,
) -> JoinHandle<()> {
    spawn(move || loop {
        if let Ok(re) = cmd("blpop")
            .arg(&list_name)
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

pub fn blpop_str(
    mut conn: Connection,
    list_name: String,
    time_out: u32,
) -> JoinHandle<Result<String, ()>> {
    spawn(move || loop {
        if let Ok(re) = cmd("blpop")
            .arg(&list_name.clone())
            .arg(time_out)
            .query::<Option<Vec<String>>>(&mut conn)
        {
            if let Some(x) = re {
                return Ok(x[x.len() - 1].clone());
            } else {
                return Err(());
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
    blpop_buffers(
        get_default_con(),
        "foo".to_owned(),
        5,
        Box::new(std::io::stdout()),
    )
    .join()
    .unwrap();
}

#[test]
fn test_rpush() {
    rpush_buffers(
        get_default_con(),
        "foo".to_owned(),
        Box::new(
            "what's up dude, how you doin'? hope you be doin' fine.\
        lately, I've been very perplexed that I am hard to decide. \
        it is mostly because I am a f*ckin' perfectionist."
                .as_bytes(),
        ),
    )
    .join()
    .unwrap()
    .unwrap();
}

#[test]
fn test_blpop_str() {
    println!(
        "{}",
        blpop_str(get_default_con(), "foo".to_owned(), 5)
            .join()
            .unwrap().unwrap()
    )
}

#[test]
fn test_rpush_str() {
    rpush_str(
        get_default_con(),
        "foo".to_owned(),
        "what's up dude, how you doin'? hope you be doin' fine.\
        lately, I've been very perplexed that I am hard to decide. \
        it is mostly because I am a f*ckin' perfectionist."
            .to_owned(),
    )
    .join()
    .unwrap()
    .unwrap();
}
