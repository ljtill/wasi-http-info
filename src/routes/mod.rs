use crate::Router;

mod base;
mod environment;
mod version;

pub fn register(router: &mut Router) {
    router.route(crate::Method::Get, "/", self::base::get_base);
    router.route(crate::Method::Get, "/version", self::version::get_version);
    router.route(
        crate::Method::Get,
        "/environment",
        self::environment::get_environment,
    );
}
