use crate::Router;


/// TypeHandler is used to configure the spring-macro marked route handler
pub trait TypedHandler: Clone {
    fn install_route(self, router: Router) -> Router;
}

pub trait TypeRouter {
    fn typed_route<H: TypedHandler>(self, handler: H) -> Self;
}

impl TypeRouter for Router {
    fn typed_route<H: TypedHandler>(self, handler: H) -> Self {
        handler.install_route(self)
    }
}
