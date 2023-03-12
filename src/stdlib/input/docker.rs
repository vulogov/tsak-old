extern crate log;
use rhai::{Dynamic, Array, Map, NativeCallContext, EvalAltResult};
use serde_json::{from_str};
use crate::stdlib::input::socket::try_get_from_socket;

pub fn docker_stat(context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    docker_stat_with_path(context, "/var/run/docker.sock".to_string())
}

pub fn docker_stat_with_path(_context: NativeCallContext, docker_path: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match try_get_from_socket(docker_path.clone(), "http://localhost/containers/json".to_string()) {
        Ok(raw_containers) => {
            match from_str::<Array>(&raw_containers.into_string().unwrap()) {
                Ok(containers) => {
                    let mut res = Map::new();
                    for c in containers.clone() {
                        let cid = format!("{}", &c.cast::<Map>().get("Id").unwrap());
                        let cid_url = format!("http://localhost/containers/{}/stats?stream=false", &cid);
                        match try_get_from_socket(docker_path.clone(), cid_url) {
                            Ok(raw_stat) => {
                                match from_str::<Map>(&raw_stat.into_string().unwrap()) {
                                    Ok(stat) => {
                                        res.insert(cid.into(), Dynamic::from(stat));
                                    }
                                    Err(err) => {
                                        let msg = format!("input::docker() stat error: {}", err);
                                        log::error!("{}", &msg);
                                        return Err(msg.into());
                                    }
                                }
                            }
                            Err(err) => {
                                let msg = format!("input::docker() stat error: {}", err);
                                log::error!("{}", &msg);
                                return Err(msg.into());
                            }

                        }

                    }
                    return Result::Ok(Dynamic::from(res));
                }
                Err(err) => {
                    let msg = format!("input::docker() error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            return Err(format!("{}", err).into());
        }
    }
}
