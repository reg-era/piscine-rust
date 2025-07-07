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

    if count == 3 {
        return true;
    }
    x = 2;
    y = 0;
    count = 0;
    for _ in 0..3 {
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

