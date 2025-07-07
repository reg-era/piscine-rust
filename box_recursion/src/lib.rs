#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let copy = self.grade.clone();
        *self = WorkEnvironment {
            grade: Some(Box::new(Worker {
                role,
                name,
                next: copy,
            })),
        };
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.clone() {
            Some(next) => {
                let res = self.grade.clone().unwrap().name;
                *self = WorkEnvironment { grade: next.next };
                Some(res)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match self.grade.clone() {
            Some(next) => Some((next.name, next.role)),
            None => None,
        }
    }
}
