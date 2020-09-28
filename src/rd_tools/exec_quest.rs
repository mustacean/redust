pub fn quest<T: redis::FromRedisValue>(
    cmd: redis::Cmd,
    con: &mut redis::Connection,
) -> Result<T, ()> {
    match cmd.query::<T>(con) {
        Ok(i) => Ok(i),
        _ => Err(()),
    }
}

pub fn exec<T: redis::FromRedisValue>(cmd: redis::Cmd, con: &mut redis::Connection) {
    cmd.execute(con)
}
