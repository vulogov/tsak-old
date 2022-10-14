use ureq::post;
use serde_json::json;
use crate::stdlib::nr::graphql::query_tpl::{make_nrql_query};

pub fn nrql_query(url: String, a: String, key: String, q: String) -> String {
    let gql_q = make_nrql_query(a, q);
    let payload = json!({"query": gql_q}).to_string();
    let api = format!("https://{}/graphql", url);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&api)
        .set("API-Key", &key)
        .set("Content-Type", "application/json")
        .send_bytes(payload.as_bytes());
    match resp {
        Ok(rsp) => {
            if rsp.status() == 200 {
                log::debug!("Request was succesful");
                rsp.into_string().unwrap()
            } else {
                log::error!("Request failed");
                "{}".to_string()
            }
        }
        Err(err) => {
            log::error!("Request failed: {:?}", err);
            "{}".to_string()
        }
    }
}
