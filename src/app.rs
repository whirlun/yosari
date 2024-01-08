use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    task::Tasks,
    worker::{AppWorker, Processor},
    prelude::get,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;


use crate::{
    controllers,
};
use axum::Router as AxumRouter;

use axum::extract::DefaultBodyLimit;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer};

use stretto::AsyncCache;

#[derive(Clone)]
struct AppState {
    dbconn: DatabaseConnection,
    cache: AsyncCache<String, String>
}
pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &str) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes() -> AppRoutes {
        let cache: AsyncCache<String, String> = AsyncCache::new(1024, 1e6 as i64, tokio::spawn).unwrap();

        AppRoutes::with_default_routes()
            .add_route(controllers::music::routes())
            .add_route(controllers::manga::routes())
    }

    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
    }

    fn register_tasks(tasks: &mut Tasks) {
        // tasks.register(tasks::seed::SeedData);
    }

    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router
            .route("/", get(|| async { "It Just Works." }))
            .layer(CorsLayer::permissive())
            .layer(DefaultBodyLimit::max(1024*1000*1000*100)))
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
    
        Ok(())
    }
}

// async fn csp_header<B>(req: Request<B>, next: Next<B>) -> Response {
//     let mut res = next.run(req).await;
//     res.headers_mut().insert("Content-Security-Policy", "connect-src 'self'".parse().unwrap());
//     res
// }
