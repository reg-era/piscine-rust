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
            Security::UnexpectedUrl => panic!("{}", m),
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
            Security::UnexpectedUrl => String::from(m),
        },
    }
}
