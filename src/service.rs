mod event_t;
mod s_meta_provider;
mod service_owned;
mod service_t;
mod test_cmds;

pub use event_t::Event;
pub use s_meta_provider::ServiceMetaProvider;
pub use service_owned::IServiceOwned;
pub use service_t::Service;

// service:list -> [service_names]
// service.{service name} -> {host}
// service.{service_name}:events -> [event_names]
