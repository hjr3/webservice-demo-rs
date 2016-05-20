#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate nickel_postgres;

use nickel::{Nickel, MediaType};
use rustc_serialize::json;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, SslMode};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

#[derive(RustcEncodable)]
struct Order {
    id: i32,
    total: f64,
    currency: String,
    status: String,
}

fn main() {

    let db_url = "postgresql://myapp:dbpass@localhost:15432/myapp";
    let db_mgr = PostgresConnectionManager::new(db_url, SslMode::None)
        .expect("Unable to connect to database");

    let db_pool = Pool::new(Config::default(), db_mgr)
        .expect("Unable to initialize connection pool");

    let mut server = Nickel::new();
    server.utilize(PostgresMiddleware::new(db_pool));

    server.utilize(router! {
        get "/orders" => |request, mut response| {
            let query = "SELECT id, total, currency, status FROM orders";
            let mut orders = Vec::new();
            let db = request.db_conn().expect("Failed to get a connection from pool");
            for row in &db.query(query, &[]).expect("Failed to select orders") {
                let order = Order {
                    id: row.get(0),
                    total: row.get(1),
                    currency: row.get(2),
                    status: row.get(3),
                };

                orders.push(order);
            }

            response.set(MediaType::Json);
            json::encode(&orders).expect("Failed to serialize orders")
        }
    });

    server.listen("127.0.0.1:6767");
}
