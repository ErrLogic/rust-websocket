use warp::Filter;
use crate::{state::ServerState, handlers};

pub fn create_routes(state: ServerState) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_state(state.clone()))
        .map(|ws: warp::ws::Ws, state| ws.on_upgrade(move |socket| handlers::handle_ws(socket, state)));

    let send_route = warp::path("send")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handlers::handle_send);

    ws_route.or(send_route).boxed()
}

fn with_state(state: ServerState) -> impl Filter<Extract = (ServerState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
