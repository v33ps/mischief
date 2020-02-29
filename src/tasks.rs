use serde::{Serialize, Deserialize};
use crossbeam_channel::{unbounded, RecvError, TryRecvError};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task_id: i32,
    pub command_type: i64,
    pub function: String,
    pub iterations: i32,
    pub params: HashMap<String, serde_json::Value>,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct FilesystemTaskParams {
//     pub directory: String,
//     pub filename: String,
//     pub content: String,
//     pub permissions: i32,
// }
//
// impl Task {
//     pub fn determine_task_type(&self) -> &String {
//         &self.name
//     }
// }
//
// pub fn handle_filesystem(task: Task, channel_out: Sender<i32>) {
//     let result = task.counter + 5;
//     channel_out.send(result);
// }

//
// pub trait JSONTrait {
//     fn to_json(&self) -> Result<()>;
// }
//
// impl Default for Task {
//     fn default() -> Self {
//         Self {
//             data: String::from(""),
//             name: String::from("na"),
//             counter: 0,
//         }
//     }
// }
//
// impl JSONTrait for Task {
//     fn to_json(&self) -> Result<()> {
//         let v: Value = serde_json::from_string(self.data)?;
//         v
//
//     }
// }
