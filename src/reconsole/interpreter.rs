

use std::fs::File;
use std::io::Read;
use rfd::AsyncFileDialog;
use futures::executor;

static mut loaded_exe: Vec<u8> = vec![];

pub struct loaded_file{
    pub name:String,
    pub data:Vec<u8>
}
// we have to return the error message or if there was no error then we return null somehow
pub unsafe fn run_file(filename:&loaded_file) -> Option<String> {
    




    return Some("success ??".to_owned());
}

// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think