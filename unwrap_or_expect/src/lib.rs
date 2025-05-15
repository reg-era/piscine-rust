pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(m) => match security_level {
            Security::UnexpectedUrl => String::from(m),
            _ => String::from(m),
        },
        Err(m) => match security_level {
            Security::Unknown => {
                panic!("called `Result::unwrap()` on an `Err` value: \"{m}\"");
            }
            Security::Message => {
                panic!("ERROR: program stops");
            }
            Security::Warning => String::from("WARNING: check the server"),
            Security::NotFound => String::from(format!("Not found: {}", m)),
            Security::UnexpectedUrl => panic!("{}", m),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
        println!("{}", fetch_data(Err("server.com"), Security::Warning));
        println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

        // Panics with no custom message
        // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

        // Panics with the message "ERROR: program stops"
        // fetch_data(Err("server.com"), Security::Message);

        // Panics with the message "malicious_server.com"
        // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
}
