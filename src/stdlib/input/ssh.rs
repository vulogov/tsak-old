extern crate log;
use ssh::*;
use std::io::Read;

pub fn ssh_command(addr: String, cmd: String) -> String {
     let mut session=Session::new().unwrap();
     session.set_host(&addr).unwrap();
     session.parse_config(None).unwrap();
     session.connect().unwrap();
     log::trace!("input::ssh session: {:?}",session.is_server_known());
     session.userauth_publickey_auto(None).unwrap();

     let mut s=session.channel_new().unwrap();
     s.open_session().unwrap();
     s.request_exec(cmd.as_bytes()).unwrap();
     s.send_eof().unwrap();
     let mut buf=Vec::new();
     s.stdout().read_to_end(&mut buf).unwrap();
     return std::str::from_utf8(&buf).unwrap().to_string();

}
