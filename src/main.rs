
mod state;
mod routes;
mod handlers;
mod utils;

use state::ServerState;

#[tokio::main]
async fn main() {
    let state = ServerState::new();
    let routes = routes::create_routes(state);

    warp::serve(routes).run(([127, 0, 0, 1], 8181)).await;
}
