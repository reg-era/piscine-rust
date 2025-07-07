#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}
pub use std::fmt;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.len() == 0 {
            return Ok(());
        }

        let mut margin = self.headers.iter().map(|h| h.len()).collect::<Vec<usize>>();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                margin[i] = margin[i].max(cell.len());
            }
        }

        write!(f, "| ")?;
        for (i, h) in self.headers.iter().enumerate() {
            write!(f, "{:^1$} |", h, margin[i])?;
            if i != self.headers.len() - 1 {
                write!(f, " ")?;
            }
        }
        writeln!(f, "")?;

        write!(f, "|")?;
        for (i, _) in self.headers.iter().enumerate() {
            write!(f, "{}", "-".repeat(margin[i] + 2))?;
            if i != self.headers.len() - 1 {
                write!(f, "+")?;
            }
        }
        writeln!(f, "|")?;

        for row in &self.body {
            write!(f, "| ")?;
            for (i, col) in row.iter().enumerate() {
                write!(f, "{:^1$} |", col, margin[i])?;
                if i != row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(Vec::from(row));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_work() {
        let mut table = Table::new();
        println!("{}", table);
        table.headers = vec![
            String::from("Model"),
            String::from("Piece N°"),
            String::from("In Stock"),
            String::from("Description"),
        ];
        table.add_row(&[
            String::from("model 1"),
            String::from("43-EWQE304"),
            String::from("30"),
            String::from("Piece for x"),
        ]);
        table.add_row(&[
            String::from("model 2"),
            String::from("98-QCVX5433"),
            String::from("100000000"),
            String::from("-"),
        ]);
        table.add_row(&[
            String::from("model y"),
            String::from("78-NMNH"),
            String::from("60"),
            String::from("nothing"),
        ]);
        println!("{}", table);
    }

    #[test]
    fn it_displays() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Fillips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        assert_eq!(
    		table.to_string(),
    		"|   Name   | Last Name |   ID Number   |\n|----------+-----------+---------------|\n| Ackerley |  Fillips  |   123456789   |\n| Adamaris |  Fillips  | 1111123456789 |\n| Ackerley |  Fillips  |   123456789   |\n"
    	);
    }

    // An empty table must not display anything
    #[test]
    fn display_table_with_no_headers() {
        let table = Table::new();
        assert_eq!(table.to_string(), "");
    }

    #[test]
    fn display_table_with_headers_only() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        assert_eq!(
            table.to_string(),
            "| Name | Last Name | ID Number |\n|------+-----------+-----------|\n"
        );
    }

    #[test]
    fn display_second() {
        let mut table = Table::new();
        table.headers = vec![
            "ID".to_string(),
            "Car Brand".to_string(),
            "Model".to_string(),
            "Is Electric".to_string(),
        ];
        table.add_row(&[
            "1".to_string(),
            "Tesla".to_string(),
            "Model 3".to_string(),
            "True".to_string(),
        ]);
        table.add_row(&[
            "2".to_string(),
            "Ford".to_string(),
            "Focus".to_string(),
            "False".to_string(),
        ]);
        assert_eq!(
    		table.to_string(),
    		"| ID | Car Brand |  Model  | Is Electric |\n|----+-----------+---------+-------------|\n| 1  |   Tesla   | Model 3 |    True     |\n| 2  |   Ford    |  Focus  |    False    |\n"
    	);
    }
}

// if self.headers.len() == 0 {
//     return Ok(());
// }

// let mut margins: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();

// // Adjust the column widths based on the body content
// for row in &self.body {
//     for (i, cell) in row.iter().enumerate() {
//         margins[i] = margins[i].max(cell.len());
//     }
// }

// // Print headers
// for (i, header) in self.headers.iter().enumerate() {
//     write!(f, "| {:^1$} ", header, margins[i])?;
// }
// writeln!(f, "|")?;

// // Print the separator line
// write!(f, "|")?;
// for (i, _) in margins.iter().enumerate() {
//     write!(f, "-{:-<1$}-", "", margins[i])?;
//     if i != margins.len() - 1 {
//         write!(f, "+")?;
//     }
// }
// writeln!(f, "|")?;

// // Print the body
// for row in &self.body {
//     for (i, cell) in row.iter().enumerate() {
//         write!(f, "| {:^1$} ", cell, margins[i])?;
//     }
//     writeln!(f, "|")?;
// }

// Ok(())
