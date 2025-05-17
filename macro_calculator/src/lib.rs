pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    struct ToParseData {
        cals: f64,
        carbs: f64,
        proteins: f64,
        fats: f64,
    }

    let mut final_macrose = ToParseData {
        cals: 0.,
        carbs: 0.,
        proteins: 0.,
        fats: 0.,
    };

    for item in foods {
        let (_, item_cal): (String, String) = item.calories.clone();

        let cal_value: f64 = item_cal.replace("kcal", "").parse().unwrap();

        final_macrose.cals +=
            (cal_value * item.nbr_of_portions * 100.0).round() / 100.0;
        final_macrose.carbs += (item.carbs * item.nbr_of_portions * 100.0).round() / 100.0;
        final_macrose.proteins += (item.proteins * item.nbr_of_portions * 100.0).round() / 100.0;
        final_macrose.fats += (item.fats * item.nbr_of_portions * 100.0).round() / 100.0;
    }

    let mut maped_data: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    maped_data.insert(
        "cals".to_string(),
        format!("{:.2}", final_macrose.cals).parse().unwrap(),
    );
    maped_data.insert(
        "carbs".to_string(),
        format!("{:.2}", final_macrose.carbs).parse().unwrap(),
    );
    maped_data.insert(
        "proteins".to_string(),
        format!("{:.2}", final_macrose.proteins).parse().unwrap(),
    );
    maped_data.insert(
        "fats".to_string(),
        format!("{:.2}", final_macrose.fats).parse().unwrap(),
    );

    json::from(maped_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foods = [
            Food {
                name: "big mac".to_owned(),
                calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
                proteins: 27.,
                fats: 26.,
                carbs: 41.,
                nbr_of_portions: 2.,
            },
            Food {
                name: "pizza margherita".to_owned(),
                calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];

        println!("{:#}", calculate_macros(&foods));
    }
}
