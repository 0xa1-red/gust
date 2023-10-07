use axum::{extract::{Path, State, FromRef}, http::{HeaderValue, Method, StatusCode}, response::Json, routing::{get, post}, Router};
use diesel::{prelude::*, QueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use key_service::KeyActorHandle;
use languages::{Language, get_languages};
use model::{Gust, GustDisplay, GustListItem, NewGust};
use std::{net::SocketAddr, collections::HashMap};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::CorsLayer;

pub mod connection;
pub mod error;
pub mod model;
pub mod schema;
pub mod languages;
pub mod key_service;

#[derive(Clone)]
struct AppState {
    deadpool: deadpool_diesel::Pool<deadpool_diesel::Manager<PgConnection>>,
    key_service: key_service::KeyActorHandle,
}

impl FromRef<AppState> for deadpool_diesel::Pool<deadpool_diesel::Manager<PgConnection>> {
    fn from_ref(input: &AppState) -> deadpool_diesel::postgres::Pool {
        input.deadpool.clone()
    }
}

impl FromRef<AppState> for key_service::KeyActorHandle {
    fn from_ref(input: &AppState) -> key_service::KeyActorHandle {
        input.key_service.clone()
    }
}

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

    let key_actor_handle = key_service::KeyActorHandle::new(pool.clone()).await;

    let state = AppState { deadpool: pool, key_service: key_actor_handle };

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/langs", get(langs))
        .route("/g", post(new_gust))
        .route("/g/:gust_key", get(gust))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .with_state(state);

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
            let d = GustDisplay::from(gust);
            return Ok(Json(d))
        },
        Ok(None) => {
            return Err((StatusCode::NOT_FOUND, String::from("Gust not found")))
        },
        Err(e) => return Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn langs() -> Result<Json<HashMap<String, Language>>, (StatusCode, String)> {
    return Ok(Json(get_languages()));
}

async fn new_gust(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    State(key_actor): State<KeyActorHandle>,
    Json(new_gust): Json<NewGust>,
) -> Result<Json<GustDisplay>, (StatusCode, String)> {
    println!("{:?}", new_gust);

    let pet_name = key_actor.get_unique_key().await;  

    let mut g = Gust::from(new_gust);
    g.key = pet_name;

    let conn = pool.get().await.map_err(error::internal_error)?;
    let res: Gust = conn
        .interact(|conn| {
            diesel::insert_into(crate::schema::gusts::table)
                .values(g)
                .returning(Gust::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(error::internal_error)?
        .map_err(error::internal_error)?;

    return Ok(Json(GustDisplay::from(res)))
}