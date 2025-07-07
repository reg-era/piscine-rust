pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut name = message.to_string();
    let mut res: Vec<Vec<char>> = Vec::new();

    let mut j = 0;
    while j < message.len() {
        let mut first: Vec<char> = vec![' '; i as usize];
        let mut newname = String::new();
        for (index, chare) in name.chars().enumerate() {
            j += 1;
            first[index] = chare;
            newname = name[index + 1..].to_string();
            if index + 1 == i as usize {
                break;
            }
        }
        name = newname;
        res.push(first);
    }

    let mut result = String::new();
    for j in 0..i as usize {
        for arr in res.clone() {
            result.push(arr[j]);
        }
    }

    return result.trim().to_string();
}
