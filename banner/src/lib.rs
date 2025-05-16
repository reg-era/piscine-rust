use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag  {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: String::from(format!("-{}", name.chars().nth(0).unwrap())),
            long_hand: String::from(format!("--{}", name)),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input) {
            Some(func) => {
                let exec = func(argv[0], argv[1]);
                match exec {
                    Ok(res)=>Ok(res),
                    Err(err)=>Err(err.to_string()),
                }
            },
            None => Err("wiiiiw".to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let new_a: Result<f64, ParseFloatError> = a.parse();
    let new_b: Result<f64, ParseFloatError> = b.parse();

    match (new_a, new_b) {
        (Ok(a), Ok(b)) => Ok((a / b).to_string()),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let new_a: Result<f64, ParseFloatError> = a.parse();
    let new_b: Result<f64, ParseFloatError> = b.parse();

    match (new_a, new_b) {
        (Ok(a), Ok(b)) => Ok((a % b).to_string()),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut handler = FlagsHandler {
            flags: HashMap::new(),
        };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag(d, div);
        handler.add_flag(r, rem);
        
        println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));
        
        println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));
        
        println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));
        
        println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
    }
}
