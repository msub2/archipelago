use prometheus_client::{
    metrics::counter::Counter,
    registry::Registry,
};

#[derive(Clone)]
pub struct Metrics {
    pub homepage_requests: Counter,
}

impl Metrics {
    pub fn new(registry: &mut Registry) -> Self {
        let homepage_requests = Counter::default();

        registry.register(
            "homepage_requests",
            "Number of requests to the homepage",
            homepage_requests.clone(),
        );

        Self {
            homepage_requests,
        }
    }
}