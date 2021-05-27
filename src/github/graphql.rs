//! Types to send GraphQL queries.

use std::borrow::Cow;

use serde_json as json;

const QUERY: &str = r#"
    query {
        viewer {
            starredRepositories(first: 100, after: <AFTER>) {
                pageInfo {
                    hasNextPage
                    endCursor
                }

                nodes {
                    databaseId,
                    nameWithOwner,
                    url,
                    description,
                    forkCount,
                    stargazerCount,

                    latestRelease {
                        name,
                        tagName,
                        url,
                        createdAt,
                        publishedAt,
                        isPrerelease,
                        descriptionHTML
                    }

                    languages(first: 10) {
                        nodes {
                            name
                        }
                    }
                }
            }
        }
    }
"#;

#[derive(serde::Deserialize)]
pub struct QueryResponse {
    pub data: ResponseData,
}

#[derive(serde::Deserialize)]
pub struct ResponseData {
    pub viewer: ViewerData,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerData {
    pub starred_repositories: StarredRepositories,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarredRepositories {
    pub page_info: PageInfo,

    pub nodes: Vec<Repository>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,

    pub end_cursor: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub database_id: i64,

    pub name_with_owner: String,

    pub url: String,

    pub description: Option<String>,

    pub fork_count: u64,

    pub stargazer_count: u64,

    pub latest_release: Option<Release>,

    #[serde(rename = "languages")]
    pub languages: Option<Languages>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Languages {
    pub nodes: Vec<Language>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Language {
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub name: Option<String>,

    pub tag_name: String,

    pub url: String,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub published_at: Option<chrono::DateTime<chrono::Utc>>,

    pub is_prerelease: bool,

    #[serde(rename = "descriptionHTML")]
    pub description_html: Option<String>,
}

impl Release {
    pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.published_at.unwrap_or(self.created_at)
    }

    pub fn name(&self) -> &str {
        self.name
            .as_deref()
            .filter(|n| !n.trim().is_empty())
            .unwrap_or(&self.tag_name)
    }
}

/// Request.
#[derive(serde::Serialize)]
pub struct Query {
    query: String,
}

/// Build a query using the specified cursor.
pub fn build_query(cursor: Option<&str>) -> Query {
    let cursor = match cursor {
        Some(c) => Cow::from(json::to_string(&c).unwrap()),
        None => Cow::from("null"),
    };

    let query = QUERY.replace("<AFTER>", &cursor);

    Query { query }
}
