use std::thread::JoinHandle;

const BUFFER_SIZE: usize = 64;

// reads st stream pushing buffers to the list named @id
pub fn push_buffers(
    mut conn: redis::Connection,
    list_name: &'static str,
    mut st: impl std::io::Read + Send + 'static,
) -> JoinHandle<Result<i32, ()>> {
    std::thread::spawn(move || {
        let mut buffer = &mut [0u8; BUFFER_SIZE];
        loop {
            if let Ok(i) = st.read(buffer) {
                if let Ok(rx) = redis::cmd("rpush")
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

// this reads(pop) the response from the list named @id
// which was pushed by the target_endpoint.
pub fn blpop_buffers(
    mut conn: redis::Connection,
    list_name: &'static str,
    time_out: u32,
    mut wr: impl std::io::Write + Send + 'static,
) -> JoinHandle<()> {
    std::thread::spawn(move || loop {
        if let Ok(re) = redis::cmd("blpop")
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
