use std::collections::HashMap;

mod mall;

use mall::*;

pub fn biggest_store(mall: &Mall) -> Store {
    let mut big_store: Store = Store {
        employees: HashMap::new(),
        square_meters: 0,
    };

    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            if store.square_meters > big_store.square_meters {
                big_store = store.clone();
            }
        }
    }

    big_store
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<Employee> {
    let mut highest_employer: Employee = Employee {
        age: 0,
        working_hours: (0, 0),
        salary: 0.,
    };

    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            for (_, employeer) in store.employees.iter() {
                if employeer.salary > highest_employer.salary {
                    highest_employer = employeer.clone();
                }
            }
        }
    }
    vec![highest_employer]
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = 0;
    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            for (_, _) in store.employees.iter() {
                count += 1;
            }
        }
    }
    count
}

pub fn check_for_securities(mall: &mut Mall, guard: HashMap<String, Guard>) {
    let mut size_of_mall = 0;

    for (_, floor) in mall.floors.iter() {
        size_of_mall += floor.size_limit;
    }

    while size_of_mall / 200 < mall.guards.len() as u64 {
        if let Some((name, new_guard)) = guard.iter().next() {
            mall.hire_guard(name, *new_guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (_, employer) in store.employees.iter_mut() {
                if employer.working_hours.1 - employer.working_hours.0 >= 10 {
                    let new_salary = employer.salary * 0.1;
                    employer.salary = new_salary;
                } else {
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut mall = Mall::new(
            "La Vie Funchal",
            [
                (
                    "John Oliver",
                    Guard {
                        age: 34,
                        years_experience: 7,
                    },
                ),
                (
                    "Bob Schumacher",
                    Guard {
                        age: 53,
                        years_experience: 15,
                    },
                ),
            ]
            .into(),
            [
                (
                    "Ground Floor",
                    Floor::new(
                        [
                            (
                                "Footzo",
                                Store::new(
                                    [
                                        (
                                            "Finbar Haines",
                                            Employee {
                                                age: 36,
                                                working_hours: (9, 14),
                                                salary: 650.88,
                                            },
                                        ),
                                        (
                                            "Sienna-Rose Penn",
                                            Employee {
                                                age: 26,
                                                working_hours: (9, 22),
                                                salary: 1000.43,
                                            },
                                        ),
                                    ]
                                    .into(),
                                    50,
                                ),
                            ),
                            (
                                "Swashion",
                                Store::new(
                                    [
                                        (
                                            "Abdallah Stafford",
                                            Employee {
                                                age: 54,
                                                working_hours: (8, 22),
                                                salary: 1234.21,
                                            },
                                        ),
                                        (
                                            "Marian Snyder",
                                            Employee {
                                                age: 21,
                                                working_hours: (8, 14),
                                                salary: 831.9,
                                            },
                                        ),
                                    ]
                                    .into(),
                                    43,
                                ),
                            ),
                        ]
                        .into(),
                        300,
                    ),
                ),
                (
                    "Supermarket",
                    Floor::new(
                        [(
                            "Pretail",
                            Store::new(
                                [
                                    (
                                        "Yara Wickens",
                                        Employee {
                                            age: 39,
                                            working_hours: (9, 14),
                                            salary: 853.42,
                                        },
                                    ),
                                    (
                                        "Indiana Baxter",
                                        Employee {
                                            age: 33,
                                            working_hours: (13, 20),
                                            salary: 991.71,
                                        },
                                    ),
                                    (
                                        "Jadine Page",
                                        Employee {
                                            age: 48,
                                            working_hours: (13, 20),
                                            salary: 743.21,
                                        },
                                    ),
                                    (
                                        "Tyler Hunt",
                                        Employee {
                                            age: 63,
                                            working_hours: (13, 20),
                                            salary: 668.25,
                                        },
                                    ),
                                    (
                                        "Mohsin Mcgee",
                                        Employee {
                                            age: 30,
                                            working_hours: (19, 24),
                                            salary: 703.83,
                                        },
                                    ),
                                    (
                                        "Antoine Goulding",
                                        Employee {
                                            age: 45,
                                            working_hours: (19, 24),
                                            salary: 697.12,
                                        },
                                    ),
                                    (
                                        "Mark Barnard",
                                        Employee {
                                            age: 53,
                                            working_hours: (19, 24),
                                            salary: 788.81,
                                        },
                                    ),
                                ]
                                .into(),
                                950,
                            ),
                        )]
                        .into(),
                        1000,
                    ),
                ),
            ]
            .into(),
        );

        // returns the biggest store
        println!("Biggest store: {:#?}", biggest_store(&mall));

        // returns the list with the highest paid employees
        println!("Highest paid employee: {:#?}", highest_paid_employee(&mall));

        // returns the number of employees
        println!("Number of employees: {}", nbr_of_employees(&mall));

        // checks if it is needed to add securities
        check_for_securities(
            &mut mall,
            [
                (
                    "Peter Solomons",
                    Guard {
                        age: 45,
                        years_experience: 20,
                    },
                ),
                (
                    "William Charles",
                    Guard {
                        age: 32,
                        years_experience: 10,
                    },
                ),
                (
                    "Leonardo Changretta",
                    Guard {
                        age: 23,
                        years_experience: 0,
                    },
                ),
                (
                    "Vlad Levi",
                    Guard {
                        age: 38,
                        years_experience: 8,
                    },
                ),
                (
                    "Faruk Berkai",
                    Guard {
                        age: 40,
                        years_experience: 15,
                    },
                ),
                (
                    "Christopher Smith",
                    Guard {
                        age: 35,
                        years_experience: 9,
                    },
                ),
                (
                    "Jason Mackie",
                    Guard {
                        age: 26,
                        years_experience: 2,
                    },
                ),
                (
                    "Kenzie Mair",
                    Guard {
                        age: 34,
                        years_experience: 8,
                    },
                ),
                (
                    "Bentley Larson",
                    Guard {
                        age: 33,
                        years_experience: 10,
                    },
                ),
                (
                    "Ray Storey",
                    Guard {
                        age: 37,
                        years_experience: 12,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
        );

        // raises or cuts the salary of every employee
        cut_or_raise(&mut mall);

        println!("{:#?}", mall);
    }
}
