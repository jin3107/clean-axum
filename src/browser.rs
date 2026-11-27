use std::{
    net::SocketAddr,
    process::{Command, Stdio},
};

/// Address of the Swagger UI as reached from this machine.
pub fn docs_url(listening_on: SocketAddr) -> String {
    // 0.0.0.0 and [::] are addresses to listen on, not to visit
    if listening_on.ip().is_unspecified() {
        return format!("http://localhost:{}/docs/", listening_on.port());
    }
    format!("http://{listening_on}/docs/")
}

/// Opens `url` in the default browser. A failure is logged and otherwise ignored.
pub fn open(url: &str) {
    let mut command = if cfg!(target_os = "windows") {
        let mut command = Command::new("rundll32");
        command.arg("url.dll,FileProtocolHandler");
        command
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else {
        Command::new("xdg-open")
    };

    let spawned = command
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match spawned {
        // collect the exit status so that no finished process is left behind
        Ok(mut child) => {
            std::thread::spawn(move || child.wait());
        }
        Err(error) => tracing::warn!("could not open a browser, visit {url}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::docs_url;

    #[test]
    fn uses_the_listening_address() {
        let address = "127.0.0.1:3000".parse().unwrap();
        assert_eq!(docs_url(address), "http://127.0.0.1:3000/docs/");
    }

    #[test]
    fn visits_localhost_when_listening_on_every_interface() {
        for address in ["0.0.0.0:8000", "[::]:8000"] {
            let address = address.parse().unwrap();
            assert_eq!(docs_url(address), "http://localhost:8000/docs/");
        }
    }
}
