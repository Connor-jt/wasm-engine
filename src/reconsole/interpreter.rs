

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
        let var = asm_registers::new();
        // we actually should just grab the pointer
        //var.RIP = 0x8000000000000000 + loaded.header.optional_header.address_of_entry_point; // we use the 0x80.. as an unreachable address
        // RIP from code insertion point
        // idk what else
        return Ok(running_process{name:self.name.to_owned(), exe:loaded, regs:});
    }
}

pub struct running_process{
    name:String,
    exe:loaded_exe,
    regs:asm_registers,
}

pub struct process_result{
    state:process_result_state,
    context:Option<String>,
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
        
    }
    pub fn run_instruction(&self){

    }
}

// construct instruction fn

// execute instruction fn





// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think