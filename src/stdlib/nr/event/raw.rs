extern crate log;
use ureq::post;

use crate::stdlib::nr;

pub fn send_event_payload(url: &String, account: &String, key: &String, payload: &String) -> bool {
    let zpayload = nr::compress_payload(payload).unwrap();
    let zp: &[u8] = &zpayload;
    let url = format!("https://{}/v1/accounts/{}/events", url, account);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Content-Encoding", "gzip")
        .set("Api-Key", key)
        .set("Content-Type", "application/json")
        .send_bytes(zp).unwrap();
    if resp.status() == 200 {
        log::debug!("Request was succesful");
        return true;
    } else {
        log::error!("Request failed");
        return false;
    }
}
