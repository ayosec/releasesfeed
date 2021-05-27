//! RSS generator

use std::collections::HashMap;
use std::fmt::{self, Write};

use crate::github::Repository;

use rss::extension::syndication::{self, SyndicationExtension, UpdatePeriod};

pub fn build_channel(repositories: &[Repository], ttl_cache: u32) -> rss::Channel {
    let items: Vec<_> = repositories
        .iter()
        .map(|repository| {
            let mut item_body = String::new();

            let _ = write!(
                &mut item_body,
                "<p><a href=\"{}\">{}</a> | {} stars| {} forks.</p>",
                Escaped(&repository.url),
                Escaped(&repository.name_with_owner),
                repository.stargazer_count,
                repository.fork_count
            );

            if let Some(languages) = &repository.languages {
                item_body.push_str("<p><small>| ");
                for lang in &languages.nodes {
                    let _ = write!(&mut item_body, "{} | ", Escaped(&lang.name));
                }
                item_body.push_str("</small></p>\n");
            }

            if let Some(description) = &repository.description {
                let html = Escaped(description);
                let _ = writeln!(&mut item_body, "<blockquote>{}</blockquote>", html);
            }

            let release = repository.latest_release.as_ref().unwrap();

            if let Some(body) = &release.description_html {
                let _ = write!(&mut item_body, "<hr>\n{}", body);
            }

            rss::ItemBuilder::default()
                .title(format!(
                    "[{}] {}",
                    repository.name_with_owner,
                    release.name()
                ))
                .link(release.url.clone())
                .pub_date(release.timestamp().to_rfc2822())
                .description(item_body)
                .build()
        })
        .filter_map(|i| i.ok())
        .collect();

    // Use syndication extension to provide a hint for aggregators.
    let mut syn_ext = SyndicationExtension::default();
    syn_ext.set_base("1970-01-01T00:00+00:00");
    syn_ext.set_frequency(ttl_cache);
    syn_ext.set_period(UpdatePeriod::HOURLY);

    let mut namespaces = HashMap::new();
    namespaces.insert("syn".to_string(), syndication::NAMESPACE.to_string());

    // Add items to a channel.
    let channel = rss::ChannelBuilder::default()
        .title("GitHub Releases")
        .namespaces(namespaces)
        .syndication_ext(Some(syn_ext))
        .items(items)
        .build()
        .unwrap();

    channel
}

/// Escape characters to insert a string in a HTML source.
struct Escaped<'a>(&'a str);

impl<'a> fmt::Display for Escaped<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for c in self.0.chars() {
            match c {
                '"' => fmt.write_str("&quot;")?,
                '&' => fmt.write_str("&amp;")?,
                '<' => fmt.write_str("&lt;")?,
                '>' => fmt.write_str("&gt;")?,
                c => fmt.write_char(c)?,
            }
        }

        Ok(())
    }
}

#[test]
fn test_escape() {
    let source = "<a>✓&lt;";
    let esc = format!("{}", Escaped(source));
    assert_eq!(esc, "&lt;a&gt;✓&amp;lt;");
}
