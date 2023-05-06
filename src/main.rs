mod errors;
mod rooms;

use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use warp::Filter;

use errors::handle_rejection;
use rooms::rooms_filter;

#[tokio::main]
async fn main() {
    // Load env variables
    dotenv().ok();

    // Connect db
    let db_string = env::var("DB_STRING").expect("Missing env var: DB_STRING");
    let db_pool = connect_to_db(&db_string).await;

    // API routes
    let initial_route = warp::get()
        .and(warp::path::end())
        .map(|| "Zoomer API active");

    let routes = initial_route
        .or(rooms_filter(db_pool.clone()))
        .recover(handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 4000)).await;
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
