#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Self {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col<T: Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {
        let mut res = Self::new();

        // Determine the indices of columns to keep
        let mut selected_indices = Vec::new();
        for (i, h) in self.headers.iter().enumerate() {
            if filter(h) {
                selected_indices.push(i);
                res.headers.push(h.clone());
            }
        }

        if selected_indices.is_empty() {
            return None;
        }

        // Keep only the selected columns for each row
        for row in &self.body {
            let filtered_row: Vec<String> =
                selected_indices.iter().map(|&i| row[i].clone()).collect();
            res.body.push(filtered_row);
        }

        Some(res)
    }

    pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
        let mut res = Self::new();

        let col_index = self.headers.iter().position(|h| h == col_name)?;
        res.headers = self.headers.clone();

        for row in &self.body {
            if filter(&row[col_index]) {
                res.body.push(row.clone());
            }
        }

        if res.body.is_empty() {
            return None;
        }

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtering_columns() {
        let mut table = Table::new();
        table.headers = vec![
            "name".to_string(),
            "lastname".to_string(),
            "id number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Philips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);

        let filter = |col: &str| col == "name";

        let new_table = Table {
            headers: vec!["name".to_string()],
            body: vec![
                vec!["Ackerley".to_string()],
                vec!["Adamaris".to_string()],
                vec!["Ackerley".to_string()],
            ],
        };
        assert_eq!(new_table, table.filter_col(filter).unwrap());
    }

    #[test]
    fn filtering_rows() {
        let tab = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Thomas".to_string(),
                    "Shelby".to_string(),
                    "123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };

        let get_fillips = |s: &str| s == "Philips";
        // filter the elements with last name Philips
        let expected_table = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };
        assert_eq!(
            tab.filter_row("Last Name", get_fillips).unwrap(),
            expected_table
        );
    }
}