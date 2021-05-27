//! Client for GitHub REST API

use crate::config::AccessToken;

use tokio::sync::mpsc;

mod graphql;

pub use graphql::Repository;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

const API_URL: &str = "https://api.github.com/graphql";

pub async fn find_releases(access_token: AccessToken, tx: mpsc::Sender<Repository>) {
    let client = reqwest::Client::builder()
        .https_only(true)
        .user_agent(USER_AGENT)
        .build()
        .unwrap();

    let mut cursor = None;
    loop {
        let body = graphql::build_query(cursor.as_deref());

        let response = client
            .post(API_URL)
            .json(&body)
            .basic_auth(&access_token.user, Some(&access_token.token))
            .send()
            .await
            .unwrap()
            .json::<graphql::QueryResponse>()
            .await
            .unwrap()
            .data
            .viewer
            .starred_repositories;

        for repository in response.nodes {
            if let Some(release) = &repository.latest_release {
                if !release.is_prerelease {
                    tx.send(repository).await.unwrap();
                }
            }
        }

        if response.page_info.has_next_page {
            cursor = response.page_info.end_cursor;
        } else {
            break;
        }
    }
}
