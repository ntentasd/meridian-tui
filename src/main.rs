mod api;
mod cli;

use anyhow::{Context, ensure};

use api::Route;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut url = args.url;

    if !url.contains("://") {
        if url.starts_with("localhost") || url.starts_with("127.0.0.1") {
            url.insert_str(0, "http://");
        } else {
            url.insert_str(0, "https://");
        }
    }

    let base_url = reqwest::Url::parse(&url)?;
    ensure!(
        base_url.scheme() == "http" || base_url.scheme() == "https",
        "URL must start with http:// or https://"
    );

    let routes_url = base_url.join("api/routes")?;

    let routes = reqwest::get(routes_url)
        .await?
        .error_for_status()?
        .json::<Vec<Route>>()
        .await
        .context("failed getting routes from Meridian")?;

    if args.dump {
        for route in routes.iter() {
            println!("Got: {} @ {}", route.name, route.hostname);
        }
    }

    Ok(())
}
