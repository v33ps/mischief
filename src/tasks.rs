use serde::{Serialize, Deserialize};
use crossbeam_channel::{unbounded, RecvError, TryRecvError};
use crossbeam_channel::{Receiver, Sender};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub data: String,
    pub name: String,
    pub counter: i32,
}

impl Task {
    pub fn determine_task_type(&self) -> &String {
        &self.name
    }
}

pub fn handle_filesystem(task: Task, channel_out: Sender<i32>) {
    let result = task.counter + 5;
    channel_out.send(result);
}

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
