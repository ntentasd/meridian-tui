use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Route {
    pub uid: String,
    pub name: String,
    pub namespace: String,
    pub kind: String,
    pub hostname: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Upsert,
    Delete,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Event {
    pub kind: Kind,
    pub route: Route,
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
