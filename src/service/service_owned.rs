use crate::service::Service;

pub trait IServiceOwned<'t> {
    fn get_service(&self) -> &'t Service;
}
