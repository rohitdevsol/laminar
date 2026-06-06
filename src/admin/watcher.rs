use notify::{RecursiveMode, Watcher};

use crate::{admin::reload::reload_config, state::app::SharedAppState};

pub async fn start_config_watcher(state: SharedAppState, path: String) -> notify::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let mut watcher = notify::recommended_watcher(move |result| {
        let _ = tx.blocking_send(result);
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    while let Some(event) = rx.recv().await {
        match event {
            Ok(_) => {
                tracing::info!("config file changed");

                if let Err(error) = reload_config(state.clone()).await {
                    tracing::error!(
                        error = %error,
                        "config reload failed"
                    );
                }
            }

            Err(error) => {
                tracing::error!(
                    error = %error,
                    "watch error"
                );
            }
        }
    }

    Ok(())
}
