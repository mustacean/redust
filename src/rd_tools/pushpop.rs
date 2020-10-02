use redis::cmd;
use redis::Connection;
use redis::ToRedisArgs;
use std::io::Read;
use std::io::Write;

pub fn rpush<T: ToRedisArgs>(conn: &mut Connection, list_name: &str, st: T) -> Result<i32, ()> {
    if let Ok(rx) = cmd("rpush").arg(list_name).arg::<T>(st).query::<i32>(conn) {
        return Ok(rx);
    }
    Err(())
}

pub fn rpush_str(mut conn: Connection, list_name: &str, st: &str) -> Result<i32, ()> {
    rpush(&mut conn, list_name, st)
}

const BUFFER_SIZE: usize = 128;

pub fn rpush_buffers(mut conn: Connection, list_name: &str, mut st: impl Read) -> Result<i32, ()> {
    let buffer = &mut [0u8; BUFFER_SIZE];
    loop {
        if let Ok(i) = st.read(buffer) {
            if let Ok(rx) = rpush(&mut conn, list_name, &buffer[0..i]) {
                if i < buffer.len() {
                    return Ok(rx);
                }
            }
        }
    }
}

//
//

pub fn blpop<T: ToRedisArgs + Clone>(
    conn: &mut Connection,
    subs: T,
    time_out: u32,
) -> Result<(String, String), ()> {
    if let Ok(re) = cmd("blpop")
        .arg(subs)
        .arg(time_out)
        .query::<Option<Vec<String>>>(conn)
    {
        if let Some(x) = re {
            return Ok((x[x.len() - 1].clone(), x[0].clone()));
        } else {
            return Err(());
        }
    } else {
        Err(())
    }
}

pub fn blpop_str(
    mut conn: Connection,
    list_name: &str,
    time_out: u32,
) -> Result<(String, String), ()> {
    blpop(&mut conn, list_name, time_out)
}

pub fn blpop_str_multiple(
    mut conn: redis::Connection,
    lists: &Vec<String>,
    time_out: u32,
    action: impl Fn(String, String),
) {
    loop {
        if let Ok((a, b)) = blpop(&mut conn, lists.clone(), time_out) {
            (action)(a, b)
        }
    }
}

pub fn blpop_buffers(mut conn: Connection, list_name: &str, time_out: u32, mut wr: impl Write) {
    loop {
        if let Ok((a, _)) = blpop(&mut conn, list_name, time_out) {
            if let Err(_) = wr.write(a.as_bytes()) {
                break;
            }
            wr.flush().unwrap();
        } else {
            break;
        }
    }
}
