mod errors;
mod handlers;
mod models;
mod routes;

use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use warp::Filter;

use errors::handle_rejection;
use routes::rooms_routes;

#[tokio::main]
async fn main() {
    // Load env variables
    dotenv().ok();

    // Setup logging
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "rooms=info");
    }
    pretty_env_logger::init();

    // CORS setup
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Content-Type",
            "Accept",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(vec!["POST", "GET"]);

    // Connect db
    let db_string = env::var("DB_STRING").expect("Missing env var: DB_STRING");
    let db_pool = connect_to_db(&db_string).await;

    // API routes
    let initial_route = warp::get()
        .and(warp::path::end())
        .map(|| "Zoomer API active");

    let routes = initial_route
        .or(rooms_routes(db_pool.clone()))
        .with(warp::log("rooms"))
        .with(cors)
        .recover(handle_rejection);

    // Get port and start server
    if env::var_os("PORT").is_none() {
        env::set_var("PORT", "4000");
    }

    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("Invalid env var: PORT");

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

pub type DBPool = Pool<Postgres>;

async fn connect_to_db(db_string: &str) -> DBPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_string)
        .await;

    match pool {
        Ok(db) => db,
        Err(e) => panic!("Couldn't connect to the database!: {}", e),
    }
}

pub fn with_db(
    db: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
