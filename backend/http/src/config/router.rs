use axum::{Extension, Router, routing::get};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::{db::get_db_pool, redis::get_redis_pool},
    controller::home::hello,
};

// type Schema = RootNode<'static, Query, EmptyMutation, Subscription>;

pub fn configure_router() -> Router {
    // let schema = Schema::new(Query, EmptyMutation::new(), Subscription);

    Router::new()
        .route("/api/", get(hello))
        // .route(
        //     "/graphql",
        //     on(
        //         MethodFilter::GET.or(MethodFilter::POST),
        //         graphql::<Arc<Schema>>,
        //     ),
        // )
        // .route("/ws", any(ws_handler))
        // .route(
        //     "/subscriptions",
        //     get(ws::<Arc<Schema>>(ConnectionConfig::new(()))),
        // )
        // .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        // .route("/playground", get(playground("/graphql", "/subscriptions")))
        // .route("/metrics", get(metrics_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        // .layer(middleware::from_fn(track_metrics))
        // .layer(Extension(Arc::new(schema)))
        .layer(Extension(get_redis_pool()))
        .layer(Extension(get_db_pool()))
}
