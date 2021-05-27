//! Collect releases from GitHub starred projects.

#[macro_use]
mod config;

mod feed;
mod github;

use std::collections::HashSet;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let tokens = match config::tokens() {
        Ok(t) => t,
        Err(e) => panic!("Failed to load tokens: {}", e),
    };

    // Number of releases to include in the generated digest.
    let max_releases_in_digest = load_var!("RF_RELEASES_COUNT", 50);

    // TTL in hours to keep the digest in a cache.
    let ttl_cache = load_var!("RF_CACHE_HOURS", 1);

    // Collect project releases.
    let (tx, mut rx) = mpsc::channel(16);

    for access_token in tokens {
        tokio::spawn(github::find_releases(access_token, tx.clone()));
    }

    drop(tx);

    let mut releases = Vec::new();
    while let Some(item) = rx.recv().await {
        releases.push(item);
    }

    // Keep only the recent releases, and remove multiple releases from the
    // same project.
    releases.sort_unstable_by(|a, b| match (&a.latest_release, &b.latest_release) {
        (Some(a), Some(b)) => b.timestamp().cmp(&a.timestamp()),
        _ => unreachable!(),
    });

    let mut found_ids = HashSet::new();
    releases.retain(|repo| found_ids.insert(repo.database_id));

    releases.truncate(max_releases_in_digest);

    // Render the RSS feed.
    let channel = feed::build_channel(&releases[..], ttl_cache);
    channel.write_to(::std::io::stdout()).unwrap();
}
