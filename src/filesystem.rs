use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::path::Path;
#[allow(unused_imports)]
use std::fs::OpenOptions;
#[allow(unused_imports)]
use std::fs::remove_file;
#[allow(unused_imports)]
use crossbeam_channel::{Receiver, Sender};
use log::{info, trace, warn};

// mod tasks;
// use tasks::*;
use crate::tasks::*;

pub fn handle_filesystem(task: &mut Task, channel_out: Sender<String>) {
    // look at the @task.function to see if we should call create_file, delete_file, etc etc
    let filename = task.params["filename"].as_str().unwrap();

    // convert @filename to &str to pass into Path::new()
    // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
    let path = Path::new(&filename);
    if task.function == String::from("create_file") {
        let ret = match create_file(&path) {
            Ok(_) => filename.to_string(),
            Err(err) => err.to_string()
        };
        channel_out.send(ret.to_string()).unwrap();
    } else if task.function == String::from("write_file") {
        let contents = task.params["content"].as_str().unwrap();
        let ret = match write_file(&path, contents.to_string()) {
            Ok(_) => filename.to_string(),
            Err(err) => err.to_string()
        };
        channel_out.send(ret.to_string()).unwrap();
    }
}


// create a file at the given @path.
fn create_file(path: &Path) -> std::io::Result<()> {
    File::create(&path)?;
    Ok(())
}

fn write_file(path: &Path, content: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
