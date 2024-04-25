#[allow(warnings)]
mod bindings;
mod routes;

use bindings::wasi::http::types;

use crate::bindings::{exports::wasi::http::incoming_handler::Guest, wasi::http::types::*};

impl PartialEq for types::Method {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Other(l0), Self::Other(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn new_response(response_out: ResponseOutparam, status_code: StatusCode, content: String) {
    let headers = Fields::new();
    let response = OutgoingResponse::new(headers);
    let body = response.body().expect("Outgoing response");

    if status_code == 200 {
        ResponseOutparam::set(response_out, Ok(response));
    } else {
        // TODO: Return an error response
        ResponseOutparam::set(response_out, Err(ErrorCode::InternalError(None)));
    }

    let out = body.write().expect("Outgoing stream");
    out.blocking_write_and_flush(content.as_bytes())
        .expect("Writing response");

    drop(out);
    OutgoingBody::finish(body, None).unwrap();
}

struct Route {
    method: Method,
    path: String,
    handler: fn() -> (StatusCode, String),
}

struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Router { routes: Vec::new() }
    }
}

impl Router {
    fn new() -> Self {
        Self::default()
    }

    fn route(&mut self, method: Method, path: &str, handler: fn() -> (StatusCode, String)) {
        let route = Route {
            method,
            path: path.to_string(),
            handler,
        };

        self.routes.push(route);
    }

    fn handle(&self, request: IncomingRequest, response_out: ResponseOutparam) {
        // TODO: Search for a route that matches the path
        // TODO: Search for a route that matches the method

        let route = self.routes.iter().find(|route| {
            route.path == request.path_with_query().unwrap() && route.method == request.method()
        });

        if route.is_some() {
            let (status_code, content) = (route.unwrap().handler)();
            new_response(response_out, status_code, content);
        } else {
            new_response(response_out, 404, "Not Found".to_string());
        }
    }
}

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let mut router = Router::new();
        routes::register(&mut router);

        router.handle(request, response_out);
    }
}

bindings::export!(Component with_types_in bindings);
