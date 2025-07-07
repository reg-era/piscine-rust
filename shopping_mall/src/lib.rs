pub mod mall;

pub use std::collections::HashMap;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut big_store: Store = Store {
        employees: HashMap::new(),
        square_meters: 0,
    };
    let mut store_name = String::new();
    for (_, floor) in mall.floors.iter() {
        for (name, store) in floor.stores.iter() {
            if store.square_meters > big_store.square_meters {
                big_store = store.clone();
                store_name = name.clone();
            }
        }
    }

    (store_name, big_store)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, Employee)> {
    let mut highest_salary = 0.;
    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            for (_, employeer) in store.employees.iter() {
                if employeer.salary > highest_salary {
                    highest_salary = employeer.salary;
                }
            }
        }
    }
    let mut res = Vec::new();

    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            for (name, employeer) in store.employees.iter() {
                if employeer.salary == highest_salary {
                    res.push((name, employeer.clone()));
                }
            }
        }
    }

    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = mall.guards.len();
    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            count += store.employees.len();
        }
    }
    count
}

pub fn check_for_securities(mall: &mut Mall, guard: HashMap<String, Guard>) {
    let mut size_of_floor = 0;
    for (_, floor) in mall.floors.iter() {
        size_of_floor += floor.size_limit;
    }

    while size_of_floor / 200 > mall.guards.len() as u64 {
        for (guard_name, guard_data) in guard.iter() {
            if !mall.guards.contains_key(guard_name) {
                mall.guards.insert(guard_name.clone(), guard_data.clone());
                break;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (_, employer) in store.employees.iter_mut() {
                if employer.working_hours.1 - employer.working_hours.0 >= 10 {
                    employer.salary += employer.salary * 0.1;
                } else {
                    employer.salary -= employer.salary * 0.1;
                }
            }
        }
    }
}
