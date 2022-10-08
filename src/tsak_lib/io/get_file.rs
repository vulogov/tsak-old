use std::io::{self, BufRead};
use curl::easy::{Easy2, Handler, WriteError};

pub fn get_file(some_url: String) -> String {
    match &some_url as &str {
        "-" => get_file_from_stdio(),
        _   => get_file_from_url(some_url),
    }
}

fn get_file_from_stdio() -> String {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
    }
    user_input
}

fn get_file_from_url(some_url: String) -> String {
    struct Collector(Vec<u8>);

    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(&some_url).unwrap();
    match easy.perform() {
        Err(err) => {
            log::error!("Request from {} returns {}", some_url, err);
            return "".to_string();
        }
        _ => {}
    }
    let contents = easy.get_ref();
    String::from_utf8_lossy(&contents.0).to_string()
}
