// service:list -> [service_names]  | done!
// service.{service name} -> {host} | done!

// service.{service_name}:events -> [event_names] | done!

mod event;
mod iredisclient;
mod s_meta_provider;
mod service;

pub use event::Event;
pub use s_meta_provider::ServiceMetaProvider;
pub use service::Service;
