pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    if i == 1 {
        return vec![format!(" {}", v)];
    }

    let mut res: Vec<String> = Vec::new();

    for y in 1..i {
        let sp = " ".repeat(y as usize);
        let pt = v.repeat(y as usize);
        res.push(format!("{}{}", sp, pt));
    }
    let sp = " ".repeat(i as usize);
    let pt = v.repeat(i as usize);
    res.push(format!("{}{}", sp, pt));
    for y in (1..i).rev() {
        let sp = " ".repeat(y as usize);
        let pt = v.repeat(y as usize);
        res.push(format!("{}{}", sp, pt));
    }

    res
}

// [
// " >",
// "  >>",
// "   >>>",
// "    >>>>",
// "    >>>>",
// "   >>>",
// "  >>",
// " >",
// ]
// [
// " >",
// "  >>",
// "   >>>",
// "    >>>>",
// "     >>>>>",
// "    >>>>",
// "   >>>",
// "  >>",
// " >"
// ]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("{:#?}", inv_pyramid(String::from("&"), 8));
        println!("{:#?}", inv_pyramid(String::from(">"), 5));
    }

    #[test]
    fn test() {
        let data_sets = vec![
            vec![],
            vec![" #"],
            vec![" a", "  aa", " a"],
            vec![
                " >",
                "  >>",
                "   >>>",
                "    >>>>",
                "     >>>>>",
                "    >>>>",
                "   >>>",
                "  >>",
                " >",
            ],
            vec![
                " &",
                "  &&",
                "   &&&",
                "    &&&&",
                "     &&&&&",
                "      &&&&&&",
                "       &&&&&&&",
                "        &&&&&&&&",
                "       &&&&&&&",
                "      &&&&&&",
                "     &&&&&",
                "    &&&&",
                "   &&&",
                "  &&",
                " &",
            ],
        ];
        assert_eq!(inv_pyramid(String::from("#"), 0), data_sets[0]);
        assert_eq!(inv_pyramid(String::from("#"), 1), data_sets[1]);
        assert_eq!(inv_pyramid(String::from("a"), 2), data_sets[2]);
        assert_eq!(inv_pyramid(String::from(">"), 5), data_sets[3]);
        assert_eq!(inv_pyramid(String::from("&"), 8), data_sets[4]);
    }
}
