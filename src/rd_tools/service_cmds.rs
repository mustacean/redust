// use redis::Cmd;

// fn crt_cmd(cmd: &str, args: &[&str]) -> Cmd {
//     let mut cmd = redis::cmd(cmd);
//     for i in args {
//         cmd.arg(*i);
//     }
//     cmd
// }

// pub fn cmd_fetch_service_names() -> Cmd {
//     crt_cmd("smembers", &["service:list"])
// }
// pub fn cmd_add_to_service_list(ser: &str) -> Cmd {
//     crt_cmd("sadd", &["service:list", ser])
// }
// pub fn cmd_rem_from_service_list(ser: &str) -> Cmd {
//     crt_cmd("srem", &["service:list", ser])
// }

// //
// pub fn cmd_fetch_service_host(ser: &str) -> Cmd {
//     crt_cmd("get", &[&format!("service.{}", ser)])
// }
// pub fn cmd_set_service_host(ser: &str, host: &str) -> Cmd {
//     crt_cmd("set", &[&format!("service.{}", ser), host])
// }
// pub fn cmd_del_service_host(ser: &str) -> Cmd {
//     crt_cmd("del", &[&format!("service.{}", ser)])
// }

// //
// pub fn cmd_fetch_events(ser: &str) -> Cmd {
//     crt_cmd("smembers", &[&format!("service.{}:events", ser)])
// }
// pub fn cmd_add_events(ser: &str, e: &str) -> Cmd {
//     crt_cmd("sadd", &[&format!("service.{}:events", ser), e])
// }
// pub fn cmd_del_events(ser: &str) -> Cmd {
//     crt_cmd("del", &[&format!("service.{}:events", ser)])
// }
