use prometheus_parse;
use rhai::{Dynamic, Map, Identifier};
use crate::tsak_lib::io::get_file;

pub fn prometheus_get(addr: &String) -> Map {
    let data = get_file::get_file(addr.to_string());
    let lines: Vec<_> = data.lines().map(|s| Ok(s.to_owned())).collect();
    let m = prometheus_parse::Scrape::parse(lines.into_iter());
    match m {
        Ok(metrics) => {
            let mut res = Map::new();
            for s in &metrics.samples {
                let mut val = Map::new();
                let key = &s.metric;
                match metrics.docs.get(&key.clone()) {
                    Some(desc) => val.insert(Identifier::from("desc"), Dynamic::from(desc.clone())),
                    None => val.insert(Identifier::from("desc"), Dynamic::from("N/A")),
                };
                match s.value {
                    prometheus_parse::Value::Counter(value) |
                    prometheus_parse::Value::Untyped(value) |
                    prometheus_parse::Value::Gauge(value) =>
                        val.insert(Identifier::from("value"), Dynamic::from(value.clone())),
                    _ => continue,
                };
                val.insert(Identifier::from("timestamp"), Dynamic::from(s.timestamp.clone()));
                res.insert(Identifier::from(key.clone()), Dynamic::from(val));
            }
            return res;
        }
        Err(err) => {
            log::error!("Error parsing prometheus responce: {}", err);
        }
    }
    Map::new()
}
