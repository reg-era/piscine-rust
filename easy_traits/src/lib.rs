#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;
    fn append_number(&mut self, nb_to_append: f64) -> Self;
    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        let new_val = Self {
            value: format!("{}{}", self.value, str_to_append),
        };
        *self = new_val.clone();
        new_val
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        let new_val = Self {
            value: format!("{}{}", self.value, nb_to_append),
        };
        *self = new_val.clone();
        new_val
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        let new_val = Self {
            value: self
                .value
                .chars()
                .filter(|&c| c != '.' && c != ',' && c != '?' && c != '!')
                .collect::<String>(),
        };
        *self = new_val.clone();
        new_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };

        println!("Before append: {}", str_aux.value);

        str_aux.append_str(String::from(" there!"));
        println!("After append: {}", str_aux.value);

        str_aux.remove_punctuation_marks();
        println!("After removing punctuation: {}", str_aux.value);
    }
}
