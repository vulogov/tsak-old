use ureq::post;
use crate::stdlib::nr::graphql::query_tpl::{make_nrql_query};

pub fn nrql_query(url: String, a: String, key: String, q: String) {
    let gql_q = make_nrql_query(a, q);
    let payload = format!(r#"{{"query": {} }}"#, gql_q);
    let api = format!("https://{}/graphql", url);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&api)
        .set("API-Key", &key)
        .set("Content-Type", "application/json")
        .send_bytes(payload.as_bytes()).unwrap();
    if resp.status() == 200 {
        log::debug!("Request was succesful");
    } else {
        log::error!("Request failed: {}", resp.status());
    }
}
