use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crossbeam_channel::{unbounded, RecvError, TryRecvError};
#[allow(unused_imports)]
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task_id: i32,
    pub command_type: i64,
    pub function: String,
    pub iterations: i32,
    pub state: i32, // 0 for new, 1 for processing, 2 for done
    pub params: HashMap<String, serde_json::Value>,
}

pub struct TaskCommandTypes {
    command_types: HashMap<i64, String>
}

impl TaskCommandTypes {
    pub fn new() -> Self {
        let mut type_mapping = HashMap::new();
        type_mapping.insert(1, String::from("filesystem"));
        type_mapping.insert(2, String::from("web"));
        Self {
            command_types: type_mapping,
        }
    }

    pub fn determine_task_type(&self, command_type: i64) -> String {
        match self.command_types.get(&command_type) {
            Some(name) => String::from(name),
            _ => String::from("")
        }
    }
}

//
// pub fn handle_filesystem(task: Task, channel_out: Sender<String>) {
//     // look at the @task.function to see if we should call create_file, delete_file, etc etc
//
//     // just a PoC for now to see that this whole thing works
//     let filename = task.params.get("filename").unwrap();
//     channel_out.send(filename.to_string()); // should probably handle error...
// }

// impl Default for Task {
//     fn default() -> Self {
//         Self {
//             data: String::from(""),
//             name: String::from("na"),
//             counter: 0,
//         }
//     }
// }
