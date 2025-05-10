pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return String::from("player X won");
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return String::from("player O won");
    }
    String::from("tie")
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    for _ in 0..3 {
        if table[x][y] == player {
            count += 1;
        }
        x += 1;
        y += 1;
    }
    // return false;
    if count == 3 {
        return true;
    }
    x = 2;
    y = 0;
    count = 0;
    for _ in 0..3 {
        // println!("{} {}", x, y);
        if table[x][y] == player {
            count += 1;
        }
        if x == 0 {
            break;
        }
        x -= 1;
        y += 1;
    }
    if count == 3 { true } else { false }
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    for _ in 0..3 {
        for _ in 0..3 {
            if table[x][y] == player {
                count += 1;
            }
            y += 1;
        }
        if count == 3 {
            return true;
        }
        count = 0;
        x += 1;
        y = 0;
    }
    if count == 3 { true } else { false }
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    for _ in 0..3 {
        for _ in 0..3 {
            if table[x][y] == player {
                count += 1;
            }
            x += 1;
        }
        if count == 3 {
            return true;
        }
        count = 0;
        x = 0;
        y += 1;
    }
    if count == 3 { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIAGONAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('X', [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]),
        ('O', [['O', 'X', 'O'], ['X', 'O', 'O'], ['X', '#', 'O']]),
    ];
    //
    const HORIZONTAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'O', 'O'], ['X', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'X', 'O'], ['O', 'O', 'O'], ['O', '#', 'X']]),
        ('X', [['O', 'X', 'O'], ['O', '#', 'O'], ['X', 'X', 'X']]),
    ];

    const VERTICAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'X', 'O'], ['O', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']]),
        ('X', [['O', 'X', 'X'], ['O', 'X', 'X'], ['X', '#', 'X']]),
    ];

    const TIE_TESTS: &[[[char; 3]; 3]] = &[
        [['O', 'X', 'O'], ['O', 'X', 'O'], ['X', '#', 'X']],
        [['O', 'X', 'O'], ['X', 'X', 'O'], ['X', '#', 'X']],
    ];
    //
    #[test]
    fn test_diagonal() {
        DIAGONAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(diagonals(p, t)));
    }
    //
    #[test]
    fn test_horizontal() {
        HORIZONTAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(horizontal(p, t)));
    }

    #[test]
    fn test_vertical() {
        VERTICAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(vertical(p, t)));
    }
    //
    #[test]
    fn test_tic_tac_toe() {
        [DIAGONAL_TESTS, HORIZONTAL_TESTS, VERTICAL_TESTS]
            .concat()
            .into_iter()
            .for_each(|(p, t)| assert_eq!(tic_tac_toe(t), format!("player {} won", p)));

        TIE_TESTS
            .iter()
            .copied()
            .for_each(|t| assert_eq!(tic_tac_toe(t), "tie"));
    }
}
