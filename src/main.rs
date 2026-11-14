mod api;
mod cli;
// TODO: Consume by the SSE client
#[allow(dead_code)]
mod store;

use anyhow::Context;

use api::Route;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let routes_url = args.url.join("api/routes")?;

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
