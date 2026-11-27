use std::net::SocketAddr;

use api::{setup_config, setup_db, setup_router, with_middleware, Config};
use doc::ApiDoc;
use utils::migrate;

use crate::browser;

async fn worker(
    child_num: u32,
    config: &'static Config,
    prefork: bool,
    listener: std::net::TcpListener,
) {
    tracing::info!("Worker {} started", child_num);

    let conn = setup_db(&config.db_url, prefork).await;

    if child_num == 0 {
        migrate(&conn).await.expect("Migration failed!");
    }

    let mut router = setup_router(conn, &config.security);
    if config.security.docs_enabled {
        router = router.attach_doc();
    }
    let router = with_middleware(router, &config.security);

    let listener = tokio::net::TcpListener::from_std(listener).expect("bind to port");
    // once per start, and only now that migrations are done and the port is open
    if child_num == 0 && config.open_docs && config.security.docs_enabled {
        let address = listener.local_addr().expect("listening address");
        browser::open(&browser::docs_url(address));
    }
    // the rate limiter needs the client address
    let service = router.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, service).await.expect("start server");
}

#[cfg(feature = "prefork")]
fn run_prefork(config: &'static Config, listener: std::net::TcpListener) {
    let num_of_cores = std::thread::available_parallelism().unwrap().get() as u32;
    let is_parent = prefork::Prefork::from_resource(listener)
        .with_num_processes(num_of_cores)
        .with_tokio(move |child_num, listener| worker(child_num, config, true, listener))
        .fork()
        .expect("prefork failed");

    if is_parent {
        tracing::info!("All workers stopped");
    }
}

fn run_non_prefork(config: &'static Config, listener: std::net::TcpListener) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(worker(0, config, false, listener));
}

pub fn run() {
    // lives as long as the process
    let config: &'static Config = Box::leak(Box::new(setup_config()));

    let listener = std::net::TcpListener::bind(config.get_server_url()).expect("bind to port");
    listener.set_nonblocking(true).expect("non blocking failed");
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

    #[cfg(feature = "prefork")]
    if config.prefork {
        run_prefork(config, listener);
        return;
    }

    run_non_prefork(config, listener);
}
