use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Route {
    pub name: String,
    pub namespace: String,
    pub kind: String,
    pub hostname: String,
}

#[cfg(test)]
mod tests {
    use serde_json::Result;

    use super::*;

    fn load_routes() -> Result<Vec<Route>> {
        let json = include_str!("../fixtures/routes.json");
        let routes = serde_json::from_str::<Vec<Route>>(json)?;

        Ok(routes)
    }

    #[test]
    fn number_of_records() {
        let routes = load_routes().expect("failed loading routes");
        assert_eq!(routes.len(), 4);
    }

    #[test]
    fn second_hostname() {
        let routes = load_routes().expect("failed loading routes");
        assert_eq!(routes[1].hostname, "cd.platform.local");
    }
}
