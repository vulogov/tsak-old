extern crate log;
use ureq::post;

use crate::stdlib::nr;

pub fn send_metric_payload(url: &String, key: &String, payload: &String) -> bool {
    let data = format!(r#"[{{"metrics":{}}}]"#, &payload);
    println!("!!! {}", data);
    let url = format!("https://{}/metric/v1", url);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Api-Key", &key)
        .set("Content-Type", "application/json")
        .send_bytes(data.as_bytes()).unwrap();
    if resp.status() == 202 {
        log::debug!("Request was succesful");
        std::process::exit(0);
    } else {
        log::error!("Request failed");
        std::process::exit(1);
    }
}
