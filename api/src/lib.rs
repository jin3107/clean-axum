mod error;
mod extractor;
mod init;
mod middleware;
mod validation;

pub mod models;
pub mod routers;

pub use init::{setup_config, setup_db, setup_router, with_middleware};
// the binary configures the server through these
pub use app::config::{Config, SecurityConfig};
