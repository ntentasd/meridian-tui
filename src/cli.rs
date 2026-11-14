use clap::Parser;
use reqwest::Url;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// URL of the Meridian instance
    ///
    /// Scheme defaults to http for localhost and 127.0.0.1,
    /// https otherwise.
    ///
    /// Example: https://meridian.company.org
    #[arg(short, long, value_parser = parse_base_url)]
    pub url: Url,

    /// Print all routes and exit
    ///
    /// Routes are written to stdout, one per line, without starting the TUI.
    #[arg(short, long)]
    pub dump: bool,
}

pub fn parse_base_url(url: &str) -> Result<Url, String> {
    let input = url;
    let mut url = String::from(input);

    if !url.contains("://") {
        if url.starts_with("localhost") || url.starts_with("127.0.0.1") {
            url.insert_str(0, "http://");
        } else {
            url.insert_str(0, "https://");
        }
    }

    let mut base_url = Url::parse(&url).map_err(|err| err.to_string())?;

    // Handle query strings & paths with no trailing '/'
    if !base_url.path().ends_with('/') {
        let path = format!("{}/", base_url.path());
        base_url.set_path(&path);
    }

    if !matches!(base_url.scheme(), "http" | "https") {
        return Err("scheme must be http or https".into());
    }

    Ok(base_url)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "https://meridian.company.org/",
        "https://meridian.company.org/api/routes"
    )]
    #[case(
        "https://meridian.company.org",
        "https://meridian.company.org/api/routes"
    )]
    #[case(
        "https://meridian.company.org/v1",
        "https://meridian.company.org/v1/api/routes"
    )]
    #[case("meridian.company.org", "https://meridian.company.org/api/routes")]
    #[case("localhost:8080", "http://localhost:8080/api/routes")]
    #[case("127.0.0.1:8080", "http://127.0.0.1:8080/api/routes")]
    fn builds_routes_url(#[case] input: &str, #[case] expected: &str) {
        let base_url = parse_base_url(input).unwrap().join("api/routes").unwrap();
        assert_eq!(base_url.as_str(), expected);
    }

    #[rstest]
    #[case("ftp://meridian.company.org")]
    #[case("file:///etc/passwd")]
    #[case("http://")]
    #[case("https://:8080")]
    #[case("")]
    fn rejects_invalid_url(#[case] input: &str) {
        assert!(parse_base_url(input).is_err());
    }
}
