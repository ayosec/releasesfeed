use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

const DIR_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct AccessToken {
    pub user: String,
    pub token: String,
}

/// Load tokens from configuration file.
pub fn tokens() -> io::Result<Vec<AccessToken>> {
    match std::env::var("RF_GITHUB_TOKENS") {
        Ok(var) => {
            let tokens = var.split(';').filter_map(split_parts);
            Ok(tokens.collect())
        }

        Err(_) => {
            let config_path = dirs::config_dir()
                .unwrap()
                .join(PathBuf::from(DIR_NAME))
                .join("tokens");

            BufReader::new(File::open(config_path)?)
                .lines()
                .map(parse_line)
                .filter_map(|access_token| match access_token {
                    Ok(None) => None,
                    Ok(Some(v)) => Some(Ok(v)),
                    Err(e) => Some(Err(e)),
                })
                .collect()
        }
    }
}

/// Extract an AccessToken instance from a line in the configuration file.
///
/// Lines without ":" or with the "#" prefix are ignored.
fn parse_line(line: io::Result<String>) -> io::Result<Option<AccessToken>> {
    let line = line?;
    let trim = line.trim();

    if trim.starts_with('#') {
        return Ok(None);
    }

    Ok(split_parts(trim))
}

fn split_parts(item: &str) -> Option<AccessToken> {
    item.split_once(':').map(|(u, t)| AccessToken {
        user: u.into(),
        token: t.into(),
    })
}

/// Try to read an environment variable, then parse its value.
macro_rules! load_var {
    ($var:expr, $default:expr) => {
        match std::env::var($var).map(|v| v.parse()) {
            Ok(Ok(n)) => n,
            Err(std::env::VarError::NotPresent) => $default,
            e => panic!("Invalud value for {}: {:?}", $var, e),
        }
    };
}
