mod errors;
mod rooms;

use warp::Filter;

use errors::handle_rejection;
use rooms::rooms_filter;

#[tokio::main]
async fn main() {
    // API routes
    let initial_route = warp::get()
        .and(warp::path::end())
        .map(|| "Zoomer API active");

    let routes = initial_route.or(rooms_filter()).recover(handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 4000)).await;
}
