pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.len() == 0 || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}