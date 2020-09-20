pub fn exec(cmd: redis::Cmd, con: &mut redis::Connection) -> Result<(), ()> {
    match cmd.query::<i32>(con) {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

pub fn pred(cmd: redis::Cmd, con: &mut redis::Connection) -> Result<(), ()> {
    match cmd.query::<bool>(con) {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

pub fn quest<T: redis::FromRedisValue>(
    cmd: redis::Cmd,
    con: &mut redis::Connection,
) -> Result<T, ()> {
    match cmd.query::<T>(con) {
        Ok(i) => Ok(i),
        _ => Err(()),
    }
}
