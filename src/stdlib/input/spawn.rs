extern crate log;
use rhai::{Dynamic, Array, NativeCallContext, EvalAltResult};
use expectrl;

pub fn expect_input(_context: NativeCallContext, cmd: String, exp: Array) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut res = Array::new();
    let mut cmds = exp.clone();
    let mut sr = true;
    match expectrl::spawn(cmd) {
        Ok(mut sess) => {
            while ! cmds.is_empty() {
                let s = cmds.remove(0);
                if s.is_string() {
                    let line = s.cast::<String>();
                    if sr {
                        if line.len() > 0 {
                            log::trace!("input::spawn send: {}", &line);
                            let _ = sess.send_line(&line);
                        }
                        sr = false;
                    } else {
                        if line.len() > 0 {
                            log::trace!("input::spawn expect {}", &line);
                            match sess.expect(&line) {
                                Ok(e) => {
                                    match String::from_utf8(e.before().to_vec()) {
                                        Ok(buf) => res.push(Dynamic::from(buf)),
                                        _ => continue,
                                    }
                                }
                                Err(_) => continue,
                            }
                        }
                        sr = true;
                    }
                }
            }
            match sess.exit(true) {
                Ok(_) => {},
                Err(err) => log::error!("input::spawn exit error: {}", err),
            }
        }
        Err(err) => {
            let msg = format!("input::spawn() returned: {}", err);
            log::error!("{}", msg);
            return Err(msg.into());
        }
    }
    Result::Ok(Dynamic::from(res))
}
