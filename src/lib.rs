#[allow(warnings)]
mod bindings;

use crate::bindings::{exports::wasi::http::incoming_handler::Guest, wasi::http::types::*};

struct Component;

impl Guest for Component {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let headers = Fields::new();
        let response = OutgoingResponse::new(headers);
        let body = response.body().expect("Outgoing response");

        ResponseOutparam::set(response_out, Ok(response));

        let out = body.write().expect("Outgoing stream");
        out.blocking_write_and_flush(b"Hello, world!")
            .expect("Writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
