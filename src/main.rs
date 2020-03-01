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

fn desearlizer(req: &mut reqwest::Response) -> Result<Client, Error> {
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

fn main() {
    // stand up and send post request to the server to get our clientID
    let mut map = HashMap::new();
    map.insert("clientName", "rust");

    let req_client = reqwest::Client::new();
    let mut res = req_client.post("http://localhost:7777/client/new")
        .json(&map)
        .send().unwrap();

    let client = desearlizer(&mut res).unwrap();
    println!("serialized: {:?}", client);

    // now loop forever getting tasks every now and then
    let duration = (client.interval * 1000.0) as u64;

    let sleep_duration = time::Duration::from_millis(duration);

    // sleep for duration given by server, every interval wake up and ask for new tasks
    loop {
        thread::sleep(sleep_duration);
        println!("hi there!");
    }


    //
    // // get incoming clients and spawn them into a thread
    // let listener = TcpListener::bind("localhost:8080").unwrap();
    //
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             let thread_hndl = thread::spawn(move || {
    //                 handle_client(&mut stream);
    //             });
    //         }
    //         Err(e) => {
    //             println!("Unable to connect client: {}", e);
    //         }
    //     }
    // }
}


/*
    @brief: gets the client connected

    This function is run as a thread. It spins and spins waiting for data from the client, then
    decides how to handle the data
    @params:
        - stream: the TCP stream that the client came in over
*/
fn handle_client(stream: &mut TcpStream) {
    // loop forever waiting for data from the client
    let (channel_out, channel_in) = unbounded();
    let task_types = TaskCommandTypes::new();
    loop {
        let mut buf = [0; 1024];

        // once we get data, send it to get_command() to desearlize it
        let task = match get_command(stream, &mut buf) {
            Ok(task) => task,
            Err(e) => return send_err(stream, e)
        };
        // now that we have our Task{}, determine the event type
        let task_type = task_types.determine_task_type(task.command_type);
        if task_type == "filesystem" {
            // start the filesystem thread and go go go
            let out_c = channel_out.clone();
            filesystem::handle_filesystem(task, out_c);
        }

        // peek into the channel from our thread to see if there is data
        // if there is, send it back
        if let Ok(resp_from_thread) = channel_in.try_recv() {
            println!("yayyy {}", &resp_from_thread);
            let _ = stream.write(resp_from_thread.to_string().as_bytes()).expect("failed to send task response");
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
