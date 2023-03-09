extern crate log;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn docker_stat(context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    docker_stat_with_path(context, "unix:///var/run/docker.sock".to_string())
}

pub fn docker_stat_with_path(_context: NativeCallContext, _docker_path: String) -> Result<Dynamic, Box<EvalAltResult>> {

    Result::Ok(Dynamic::default())
}
