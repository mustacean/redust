use redis::cmd;
use redis::Connection;
// use std::io::Read;
// use std::io::Write;

const BUFFER_SIZE: usize = 64;

// pub fn rpush_buffers(
//     mut conn: Connection,
//     list_name: String,
//     mut st: Box<dyn Read>,
// ) -> Result<i32, ()> {
//     let buffer = &mut [0u8; BUFFER_SIZE];
//     loop {
//         if let Ok(i) = st.read(buffer) {
//             if let Ok(rx) = cmd("rpush")
//                 .arg(list_name.clone().as_str())
//                 .arg::<&[u8]>(&mut buffer[0..i])
//                 .query::<i32>(&mut conn)
//             {
//                 if i < buffer.len() {
//                     return Ok(rx);
//                 }
//             }
//         }
//     }
// }

pub fn rpush_str(mut conn: Connection, list_name: String, st: String) -> Result<i32, ()> {
    loop {
        if let Ok(rx) = cmd("rpush")
            .arg(&list_name.clone())
            .arg::<&str>(&st)
            .query::<i32>(&mut conn)
        {
            return Ok(rx);
        }
    }
}

// pub fn blpop_buffers(
//     mut conn: Connection,
//     list_name: String,
//     time_out: u32,
//     mut wr: Box<dyn Write>,
// ) {
//     loop {
//         if let Ok(re) = cmd("blpop")
//             .arg(&list_name)
//             .arg(time_out)
//             .query::<Option<Vec<String>>>(&mut conn)
//         {
//             if let Some(x) = re {
//                 if let Err(_) = wr.write(&x[x.len() - 1].as_bytes()) {
//                     break;
//                 }
//                 wr.flush().unwrap();
//             } else {
//                 break;
//             }
//         }
//     }
// }

pub fn blpop_str(
    mut conn: Connection,
    list_name: String,
    time_out: u32,
) -> Result<(String, String), ()> {
    if let Ok(re) = cmd("blpop")
        .arg(&list_name.clone())
        .arg(time_out)
        .query::<Option<Vec<String>>>(&mut conn)
    {
        if let Some(x) = re {
            return Ok((x[x.len() - 1].clone(), list_name));
        } else {
            return Err(());
        }
    } else {
        Err(())
    }
}

pub fn blpop_str_multiple(
    mut conn: redis::Connection,
    lists: Vec<String>,
    time_out: u32,
    action: impl Fn(String, String),
) {
    loop {
        if let Ok(re) = cmd("blpop")
            .arg(lists.clone())
            .arg(time_out)
            .query::<Option<Vec<String>>>(&mut conn)
        {
            if let Some(x) = re {
                (action)(x[x.len() - 1].clone(), x[0].clone());
            }
        } else {
            panic!("command couldn't be executed")
        }
    }
}
