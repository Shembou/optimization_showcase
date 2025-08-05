use std::sync::Arc;

use axum::{
    routing::{any, get, on, MethodFilter}, Extension, Router
};
use juniper::{EmptyMutation, RootNode};
use juniper_axum::{graphiql, graphql, playground, ws};
use juniper_graphql_ws::ConnectionConfig;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    api::{home::hello, websocket::ws_handler},
    graphql::{queries::Query, subscriptions::Subscription},
};

type Schema = RootNode<'static, Query, EmptyMutation, Subscription>;

pub fn configure_router() -> Router {
    let schema = Schema::new(Query, EmptyMutation::new(), Subscription);

    Router::new()
        .route("/api/", get(hello))
        .route(
            "/graphql",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                graphql::<Arc<Schema>>,
            ),
        )
        .route("/ws", any(ws_handler))
        .route(
            "/subscriptions",
            get(ws::<Arc<Schema>>(ConnectionConfig::new(()))),
        )
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(Extension(Arc::new(schema)))
}
