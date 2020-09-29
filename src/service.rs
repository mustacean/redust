mod endpoint_t;
mod event_t;
mod service_owned;
mod service_t;
mod remoteinit;

pub use endpoint_t::Endpoint;
pub use event_t::Event;
pub use service_owned::IServiceOwned;
pub use service_t::Service;
use remoteinit::RemoteInit;

// service:list -> [service_names]
// service.{service name} -> {host}
// service.{service_name}:events -> [event_names]
