pub use chrono::Utc;
// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]

pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}        

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let new_form = Form {
            name: field_name.to_string(),
            password: field_value,
        };

        match new_form.validate() {
            Ok(()) => FormError {
                form_values: ("name", new_form.name.clone().to_string()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: err,
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
                form_values: ("name", self.name.clone().to_string()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty",
            })
        } else if self.password.len() < 8 {
            Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long",
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
                    form_values: ("password", self.password.clone()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols",
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
        let cases = [
            (
                Form {
                    name: "Katy".to_owned(),
                    password: "qwTw12&%$3sa1dty_".to_owned(),
                },
                Ok(()),
            ),
            (
                Form {
                    name: "".to_owned(),
                    password: "qwTw12&%$3sa1dty_".to_owned(),
                },
                Err(FormError {
                    form_values: ("name", "".to_owned()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Username is empty",
                }),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "12345".to_owned(),
                },
                Err(FormError {
                    form_values: ("password", "12345".to_owned()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be at least 8 characters long",
                }),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "sdASDsrW".to_owned(),
                },
                Err(FormError {
                    form_values: ("password", "sdASDsrW".to_owned()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols",
                }),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "dsGE1SAD213".to_owned(),
                },
                Err(FormError {
                    form_values: ("password", "dsGE1SAD213".to_owned()),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols",
                }),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "dsaSD&%DF!?=".to_owned(),
                },
                Err(FormError {
                    form_values: ("password", String::from("dsaSD&%DF!?=")),
                    date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols",
                }),
            ),
        ];

        for (form, expected) in cases {
            assert_eq!(form.validate(), expected);
        }
    }
}
