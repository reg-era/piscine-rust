fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}

pub fn brain_fuck(code: &str) {
    if code.is_empty() {
        return;
    }

    let code = code.chars().collect::<Vec<char>>();
    let mut pattern = [0 as u8; 2048];
    let mut pointer = 0;
    let mut i = 0;

    while i < code.len() {
        match code[i] {
            '.' => print!("{}", pattern[pointer] as char),
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => pattern[pointer] += 1,
            '-' => pattern[pointer] -= 1,
            '[' => {
                if pattern[pointer] == 0 {
                    loop {
                        i += 1;
                        if code[i] == ']' {
                            break;
                        }
                    }
                }
            }
            ']' => {
                if pattern[pointer] != 0 {
                    loop {
                        i -= 1;
                        if code[i] == '[' {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn nothing_passed() {
        assert_eq!("", run(""));
    }

    #[test]
    fn single_chars() {
        assert_eq!(
            "a",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.")
        );
        assert_eq!(
            "S",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++++++++.")
        );
        assert_eq!(
            "7",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>---------------.")
        );
    }
    #[test]
    fn phrases() {
        assert_eq!(
            "Wow",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>-------------.++++++++++++++++++++++++.++++++++.")
        );
        assert_eq!(
            "Good job!",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+.>+++++++++++..-----------.<<++.>>++++++.+++++.-------------.<<+.")
        );
    }

    #[test]
    fn with_characters_in_middle() {
        assert_eq!("keep going", run("++++++++++[>+>ke+++>+++++++>++++++++++<<<<-]>>>>+++++++e.------p..+++++++++++.<<++.>g>---------.+o+++++++.------i.+++++.-n------.g"));
    }

    #[test]
    fn big_test() {
        assert_eq!(
            "3, 2, 1... Happy New Year",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>-------------------.-------.<++.>++++++.------.<.>+++++.---...<.>++++++++++++++++++++++++++.>---.+++++++++++++++..+++++++++.<<.>++++++.>--------------------.++++++++++++++++++.<<.>+++++++++++.++++++++++++.----.>-----.")
        );
        assert_eq!(
            "To be or not be, that is the question!", 
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++++++++++++++.>+++++++++++.<<++.>>-------------.+++.<<.>>++++++++++.+++.<<.>>----.+.+++++.<<.>++++++++++++++.+++.<++++++++++++.------------.>>.<+++.-------.>.<<.>++++++++.>-.<<.>>+.<-.---.<.>>---.++++.<.>--.+.<++++.>-----.-.<<+.")
        );
    }
}
