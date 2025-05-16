use chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let new_form = Form {
            name: field_name.to_string(),
            password: field_value.to_string(),
        };

        match new_form.validate() {
            Ok(()) => FormError {
                form_values: ("name".to_string(), new_form.name.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: err.to_string(),
            },
            Err(error) => error,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.len() == 0 {
            Err(FormError {
                form_values: ("name".to_string(), self.name.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty".to_string(),
            })
        } else if self.password.len() < 8 {
            Err(FormError {
                form_values: ("password".to_string(), self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long".to_string(),
            })
        } else {
            let mut is_alpha = false;
            let mut is_num = false;
            let mut is_punc = false;
            for c in self.password.chars() {
                if c.is_ascii_alphabetic() {
                    is_alpha = true;
                }
                if c.is_ascii_digit() {
                    is_num = true;
                }
                if c.is_ascii_punctuation() {
                    is_punc = true;
                }
            }
            if is_alpha && is_num && is_punc {
                Ok(())
            } else {
                Err(FormError {
                    form_values: ("password".to_string(), self.password.clone()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols"
                        .to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut form_output = Form {
            name: "Lee".to_owned(),
            password: "qwqwsa1dty_".to_owned(),
        };

        println!("{:?}", form_output);
        println!("{:?}", form_output.validate());

        form_output.name = "".to_owned();
        println!("{:?}", form_output.validate());

        form_output.name = "as".to_owned();
        form_output.password = "dty_1".to_owned();
        println!("{:?}", form_output.validate());

        form_output.password = "asdasASd(_".to_owned();
        println!("{:?}", form_output.validate());

        form_output.password = "asdasASd123SA".to_owned();
        println!("{:?}", form_output.validate());
    }
}
