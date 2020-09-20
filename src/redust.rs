// service:list -> [service_names] 
// service.{service name} -> {host}
// service.{service_name}:events -> [event_names]

mod event;
mod iredisclient;
mod s_meta_provider;
mod service;
mod redis_cmd_fac;
mod redis_exec;
mod test_cmds;
mod event_handler;
mod test_pubsub;

pub use event::Event;
pub use s_meta_provider::ServiceMetaProvider;
pub use service::Service;
