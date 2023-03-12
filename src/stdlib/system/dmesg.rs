extern crate log;
use rmesg;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn dmesg_buffer(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match rmesg::logs_raw(rmesg::Backend::Default, true) {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res.clone()));
        }
        Err(err) => return Err(format!("metrics::dmesg::buffer error: {}", err).into()),
    }
}

pub fn dmesg_buffer_dev(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match rmesg::logs_raw(rmesg::Backend::DevKMsg, true) {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res.clone()));
        }
        Err(err) => return Err(format!("metrics::dmesg::buffer error: {}", err).into()),
    }
}

pub fn dmesg_buffer_klog(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match rmesg::logs_raw(rmesg::Backend::KLogCtl, true) {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res.clone()));
        }
        Err(err) => return Err(format!("metrics::dmesg::buffer error: {}", err).into()),
    }
}
