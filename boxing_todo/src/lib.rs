mod err;

pub use std::error::Error;
pub use std::fs;
// use std::{fs::File, io::Write};

pub use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let data_file = fs::read_to_string(path).map_err(|err| ReadErr {
            child_err: Box::new(err),
        })?;

        let content = json::parse(&data_file).map_err(|err| ParseErr::Malformed(Box::new(err)))?;

        if content["tasks"].is_empty() {
            return Err(ParseErr::Empty.into());
        }

        Ok(Self {
            title: content["title"].to_string(),
            tasks: content["tasks"]
                .members()
                .map(|task_data| Task {
                    id: task_data["id"].as_u32().unwrap(),
                    description: task_data["description"].to_string(),
                    level: task_data["level"].as_u32().unwrap(),
                })
                .collect(),
        })
    }
}

// #[cfg(test)]
// mod tests {
// use super::*;
//
// #[test]
// fn it_works() {
// let files = [
// (
// "todo.json",
// r#"{
// "title" : "TODO LIST FOR PISCINE RUST",
// "tasks": [
// { "id": 0, "description": "do this", "level": 0 },
// { "id": 1, "description": "do that", "level": 5 }
// ]
// }"#,
// ),
// (
// "todo_empty.json",
// r#"{
// "title" : "TODO LIST FOR PISCINE RUST",
// "tasks": []
// }"#,
// ),
// (
// "malformed_object.json",
// r#"{
// "something": ,
// }"#,
// ),
// ];
//
// for (name, content) in files {
// File::create(name)
// .unwrap()
// .write(content.as_bytes())
// .unwrap();
//
// let todos = TodoList::get_todo(name);
// match todos {
// Ok(list) => println!("{:?}", list),
// Err(e) => {
// println!("{}: {:?}", e.to_string(), e.source());
// }
// }
// }

// #[test]
// fn test_read_err() {
// let result = TodoList::get_todo("invalid_file.json").unwrap_err();
//
// let ReadErr { child_err } = result.downcast_ref().unwrap();
//
// assert!(child_err.is::<io::Error>());
// assert!(result.source().unwrap().is::<io::Error>());
// assert_eq!(result.to_string(), "Failed to read todo file");
// }

// }
// }
