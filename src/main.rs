#[macro_use] extern crate nickel;

use nickel::{Nickel, MediaType};

fn main() {

    let mut server = Nickel::new();

    server.utilize(router! {
        get "/orders" => |_request, mut response| {
            response.set(MediaType::Json);
            r#"{ "foo": "bar" }"#
        }
    });

    server.listen("127.0.0.1:6767");
}
