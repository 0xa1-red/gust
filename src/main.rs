use axum::{extract::{Path, State}, http::{HeaderValue, Method, StatusCode}, response::Json, routing::get, Router};
use diesel::{prelude::*, QueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use model::{Gust, GustDisplay, GustListItem};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::CorsLayer;

pub mod connection;
pub mod error;
pub mod model;
pub mod schema;

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
        .route("/g/:gust_key", get(gust))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
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
) -> Result<Json<Vec<GustListItem>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(error::internal_error)?;
    let res = conn
        .interact(|conn| schema::gusts::table.select(Gust::as_select()).load(conn))
        .await
        .map_err(error::internal_error)?
        .map_err(error::internal_error)?;

    let mut display = Vec::new();
    res.iter().for_each(|g| {
        let d = GustListItem {
            key: g.key.clone(),
            title: g.title.clone(),
            created_at: g.created_at,
            accessed: g.accessed,
            starred: g.starred,
        };

        display.push(d);
    });

    Ok(Json(display))
}

async fn gust(
    Path(gust_key): Path<String>,
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<GustDisplay>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(error::internal_error)?;
    let res: Result<Option<Gust>, diesel::result::Error> = conn
        .interact(|conn| {
            schema::gusts::table
                .filter(crate::schema::gusts::key.eq(gust_key))
                .select(Gust::as_select())
                .first(conn)
                .optional()
        })
        .await
        .map_err(error::internal_error)?;

    match res {
        Ok(Some(gust)) => {
            let d = GustDisplay {
                key: gust.key.clone(),
                title: gust.title.clone(),
                content: String::from_utf8(gust.content.clone()).expect("Found invalid UTF-8"),
                created_at: gust.created_at,
                accessed: gust.accessed,
                starred: gust.starred,
            };
            return Ok(Json(d))
        },
        Ok(None) => {
            return Err((StatusCode::NOT_FOUND, String::from("Gust not found")))
        },
        Err(e) => return Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}