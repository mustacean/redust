use redis::Cmd;

pub fn cmd_del_events(ser: &str) -> Cmd {
    let mut cmd: redis::Cmd = redis::cmd("del");
    cmd.arg(format!("service.{}:events", ser).as_str());
    cmd
}

pub fn cmd_add_events(ser: &str, e: &str) -> Cmd {
    let mut cmd = redis::cmd("sadd");
    cmd.arg(format!("service.{}:events", ser).as_str());
    cmd.arg(e);
    cmd
}

pub fn cmd_fetch_events(ser: &str) -> Cmd {
    let mut cmd = redis::cmd("smembers");
    cmd.arg(format!("service.{}:events", ser).as_str());
    cmd
}

pub fn cmd_fetch_service_host(ser: &str) -> Cmd {
    let mut cmd = redis::cmd("get");
    cmd.arg(format!("service.{}", ser).as_str());
    cmd
}

pub fn cmd_del_service_host(ser: &str) -> Cmd {
    let mut cmd = redis::cmd("del");

    cmd.arg(format!("service.{}", ser).as_str());
    cmd
}

pub fn cmd_rem_from_service_list(ser: &str) -> Cmd {
    let mut cmd = redis::cmd("srem");

    cmd.arg("service:list");
    cmd.arg(ser);
    cmd
}

pub fn cmd_set_service_host(ser: &str, host: &str) -> Cmd {
    let mut cmd = redis::cmd("set");
    cmd.arg(format!("service.{}", ser).as_str());
    cmd.arg(host);
    cmd
}

pub fn cmd_add_to_service_list(ser: &str) -> Cmd {
    let mut cmd = redis::cmd("sadd");
    cmd.arg("service:list");
    cmd.arg(ser);
    cmd
}
