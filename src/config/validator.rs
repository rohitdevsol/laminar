use std::collections::HashSet;
use anyhow::{ bail, Result };
use crate::config::types::Config;

// The load balancer follows a fail-fast startup philosophy.
// Invalid topology or malformed configuration should prevent startup
// rather than causing runtime instability.

pub fn validate_config(config: &Config) -> Result<()> {
    let mut upstream_ids = HashSet::new();
    for upstream in &config.upstreams {
        if upstream.servers.is_empty() {
            bail!("upstream '{}' has no backend servers", upstream.id);
        }
        if !upstream_ids.insert(upstream.id.clone()) {
            bail!("duplicate upstream id '{}'", upstream.id);
        }
        for server in &upstream.servers {
            if server.port == 0 {
                bail!("server '{}' has invalid port", server.id);
            }

            if server.weight == 0 {
                bail!("server '{}' has invalid weight", server.id);
            }
        }
    }
    Ok(())
}
