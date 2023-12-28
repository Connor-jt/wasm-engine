

use std::fs::File;
use std::io::Read;
use std::thread::yield_now;
use rfd::AsyncFileDialog;
use futures::executor;

use executable::loaded_exe;
use registers::asm_registers;

mod registers;
mod executable;

pub struct loaded_file{
    pub name:String,
    pub data:Vec<u8>,
}
impl loaded_file{
    pub unsafe fn load_file(&self) -> Result<running_process, String> {
        let loaded: loaded_exe = executable::load_exe(&self.data)?;
        // we need to config some basic data here
        let mut var = asm_registers::new();
        var.RIP = loaded.runtime_data.as_ptr().add(loaded.header.optional_header.address_of_entry_point as usize) as u64;
        // idk what else
        return Ok(running_process{name:self.name.to_owned(), exe:loaded, regs:var});
    }
}

pub struct running_process{
    pub name:String,
    pub exe:loaded_exe,
    pub regs:asm_registers,
}

pub struct process_result{
    pub state:process_result_state,
    pub context:Option<String>,
    //data:Option<Vec<u8>>, // potentially unneeded
}
pub enum process_result_state{
    running,
    print,
    breakpoint,
    warning,
    error,
    exit,

    unimplemented, // this refers to loading dll's, reading/writing files. etc
}
impl Iterator for running_process{
    type Item = process_result;
    fn next(&mut self) -> Option<Self::Item> {


        return Some(process_result{state:process_result_state::exit, context:Some("program success".to_owned())});
    }
}
impl running_process{
    pub fn get_instruction(&self){
        // prefixes (various length)
        // 
    }
    pub fn run_instruction(&self){

    }
}

// construct instruction fn

// execute instruction fn





// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think