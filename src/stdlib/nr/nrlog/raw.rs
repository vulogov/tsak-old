extern crate log;
use ureq::post;

pub fn send_log_payload(url: &String, key: &String, payload: &String) -> bool {
    let data = format!(r#"[{{"logs":{}}}]"#, &payload);
    log::trace!("Payload to send: {:?}", &data);
    let url = format!("https://{}/log/v1", url);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Api-Key", &key)
        .set("Content-Type", "application/json")
        .send_bytes(data.as_bytes());
    match resp {
        Ok(rsp) => {
            if rsp.status() == 202 {
                log::debug!("Request was succesful");
                return true;
            } else {
                log::error!("Request failed");
                return false;
            }
        }
        Err(err) => {
            log::error!("Request failed: {:?}", err);
            return false;
        }
    }
}
