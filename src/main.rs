use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use diesel::{QueryDsl, SelectableHelper, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use model::{Gust, GustDisplay};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod connection;
pub mod model;
pub mod schema;
pub mod error;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = connection::create_pool().unwrap();

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .with_state(pool);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<GustDisplay>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(error::internal_error)?;
    let res = conn
        .interact(|conn| schema::gusts::table.select(Gust::as_select()).load(conn))
        .await
        .map_err(error::internal_error)?
        .map_err(error::internal_error)?;

    let mut display = Vec::new();
    res.iter().for_each(|g| {
        let d = GustDisplay{
            key: g.key.clone(),
            title: g.title.clone(),
            content: String::from_utf8(g.content.clone()).expect("invalid utf8 sequence in content"),
            created_at: g.created_at,
            accessed: g.accessed,
            starred: g.starred,
        };

        display.push(d);
    });

    Ok(Json(display))
}