// service:list -> [service_names]

// service.{service name} -> {host}

mod event;
mod iredisclient;
mod s_meta_provider;
mod service;

pub use event::Event;
pub use s_meta_provider::ServiceMetaProvider;
pub use service::Service;
