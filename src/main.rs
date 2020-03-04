use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use serde_json::{Error};
// #[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crossbeam_channel::{unbounded, RecvError, TryRecvError};
#[allow(unused_imports)]
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
extern crate reqwest;
// use std::time::Duration;
use std::time;

mod tasks;
use tasks::*;
mod filesystem;
use filesystem::*;
use log::{info, trace, warn};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub client_name: String,
    #[serde(rename = "clientID")]
    pub client_id: i64,
    pub task_queue: Vec<Task>,
    pub lastcheckintime: i64,
    pub interval: f32,
}

fn desearlizer_client(req: &mut reqwest::Response) -> Result<Client, Error> {
    let mut buffer = String::new();
    match req.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => println!("error : {}", e.to_string())
    };
    println!("buffer before serializaztion: {}", buffer);

    let v = match serde_json::from_str::<Client>(&buffer){
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    Ok(v)
}

fn desearlizer_task(req: &mut reqwest::Response) -> Result<Task, Error> {
    let mut buffer = String::new();
    match req.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => println!("error : {}", e.to_string())
    };
    println!("buffer before serializaztion: {}", buffer);

    let v = match serde_json::from_str::<Task>(&buffer){
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    Ok(v)
}

impl Client {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("clientName", "rust".to_string());
        let req_client = reqwest::Client::new();
        let mut res = req_client.post("http://localhost:7777/client/new")
                                .json(&map).send().unwrap();

        let mut buffer = String::new();
        match res.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => println!("error : {}", e.to_string())
        };
        println!("buffer before serializaztion: {}", buffer);

        let v = serde_json::from_str::<Client>(&buffer).expect("oh");

        Self {
            ..v
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.task_queue.push(task);
    }

    pub fn get_task(&mut self) {
        let mut map = HashMap::new();
        map.insert("clientID", self.client_id);
        let req_client = reqwest::Client::new();
        let mut res = req_client.post("http://localhost:7777/client/get_tasks")
                                .json(&map).send().unwrap();

        let mut buffer = String::new();
        match res.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => println!("error : {}", e.to_string())
        };
        if res.status() != 204 {
            println!("task buffer before serializaztion: {}", buffer);
            let v = serde_json::from_str::<Client>(&buffer).expect("oh");
            // check to see that the taskID doesn't already exist in our task_queue
            let mut found = false;
            for current_task in &mut self.task_queue {
                for new_task in & v.task_queue {
                    if current_task.task_id == new_task.task_id {
                        found = true;
                        break;
                    }
                }
            }
            // if we haven't found this task, then add it to the queue. Otherwise, we are probably
            // already processing it
            if found != true {
                for task in v.task_queue {
                    self.task_queue.push(task);
                }
            }
        }



    }
}

fn main() {
    // let name = String::from("rust");
    let mut client = Client::new();

    // now loop forever getting tasks every now and then
    let duration = (&client.interval * 1000.0) as u64;

    let sleep_duration = time::Duration::from_millis(duration);

    let (channel_out, channel_in) = unbounded();
    // sleep for duration given by server, every interval wake up and ask for new tasks
    loop {
        thread::sleep(sleep_duration);

        // get new tasks from the server
        // need to return success/failure so we know if we should send something into the thread or not
        client.get_task();
        // fuck me
        let mut c = client.clone();
        let out_c = channel_out.clone();
        // spawn a thread to deal with the new tasks
        let thread_hndl = thread::spawn(move || {
            handle_task(&mut c, out_c);
        });
        if let Ok(resp_from_thread) = channel_in.try_recv() {
            println!("yayyy from main {}", &resp_from_thread);
            // need to send resp to server, and remvoe task from the queue
            let resp_task_id = resp_from_thread.parse::<i32>().unwrap();
            client.task_queue.retain(|x| x.task_id != resp_task_id);
        }
    }
}

fn handle_task(client: &mut Client, main_out_c: Sender<String>) {
    let (channel_out, channel_in) = unbounded();
    let task_types = TaskCommandTypes::new();

    // walk over the task queue. For any task_queue.state == 0, handle it.
    for task in &mut client.task_queue {
        // all tasks will have at least 1 iteration, but may have more. We also may have a sleep
        // between iterations
        let duration = (task.iteration_delay * 1000) as u64;
        let sleep_duration = time::Duration::from_millis(duration);
        for _iteration in 0..task.iterations {
            let task_type = task_types.determine_task_type(task.command_type);
            if task_type == "filesystem" {
                // start the filesystem thread and go go go
                let out_c = channel_out.clone();
                filesystem::handle_filesystem(task, out_c);
                task.state = 1;
            }
            // peek into the channel from our thread to see if there is data
            // if there is, send it back
            if let Ok(resp_from_thread) = channel_in.try_recv() {
                println!("handle_task got something: {}", &resp_from_thread);
                // should send the task ID back out if successful. Otherwise, an err string
                main_out_c.send(resp_from_thread).unwrap();
                task.state = 2;
            }
            thread::sleep(sleep_duration);
        }
    }
}

fn serializer(msg: String) -> Result<Task, Error> {
    let v = match serde_json::from_str::<Task>(&msg){
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    Ok(v)
}

fn get_command(stream: &mut TcpStream, buf: &mut[u8]) -> Result<Task, Error> {
    let buf_sz = stream.read(buf).expect("failed to read from stream");
    let buf_usize = buf_sz as usize;

    let v = match serde_json::from_slice::<Task>(&buf[..buf_usize]){
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    Ok(v)
}

fn send_err(stream: &mut TcpStream, err: Error) {
    let _ = stream.write(err.to_string().as_bytes()).expect("failed a write");
}
