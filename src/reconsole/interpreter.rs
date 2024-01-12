

use std::fmt::Pointer;
use std::fs::File;
use std::io::Read;
use std::thread::yield_now;
use futures::stream::ForEach;
use rfd::AsyncFileDialog;
use futures::executor;

use executable::loaded_exe;
use registers::asm_registers;


mod registers;
mod executable;
mod asm86;

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
    fn next(&mut self) -> Self::Item {


        return process_result{state:process_result_state::exit, context:Some("program success".to_owned())};
    }
}
impl running_process{
    pub unsafe fn read_byte(&self, address:u64) -> u8 {
        let pointer = address as *const u8;
        return *pointer;
    }
    pub unsafe fn get_prefix(&mut self) -> u16 {
        let mut curr_prefix:u16 = 0;
        loop{
            let curr_byte = self.read_byte(self.regs.RIP);
            // if it falls within the rex prefix range, then we just take the rex prefix and pop it in ours
            if (curr_byte >= 0x40 && curr_byte < 0x50){
                curr_prefix |= (curr_byte & 0b1111) as u16
            // regular prefixes
            } else{ match curr_byte{
                0x64 => {curr_prefix |= asm86::prefixes::fs as u16}, 
                0x65 => {curr_prefix |= asm86::prefixes::gs as u16}, 
                0x66 => {curr_prefix |= asm86::prefixes::operand_size as u16}, 
                0x67 => {curr_prefix |= asm86::prefixes::address_size as u16}, 
                0x9B => {curr_prefix |= asm86::prefixes::wait as u16}, 
                0xF0 => {curr_prefix |= asm86::prefixes::lock as u16}, 
                0xF2 => {curr_prefix |= asm86::prefixes::repnz as u16}, 
                0xF3 => {curr_prefix |= asm86::prefixes::repz as u16}, 
                _ => return curr_prefix
            }}
            // push RIP each successful prefix
            self.regs.RIP += 1
        }
    }
    pub unsafe fn get_instruction(&self) -> Option<&asm86::instruction>{
        // prefixes (various length)
        // 

        // read RIP pointer and get first 3 bytes?
        let result = asm86::get_instruction(self.read_byte(self.regs.RIP), self.read_byte(self.regs.RIP+1), self.read_byte(self.regs.RIP+2));
        // then read any required, RM byte, SIB, disp8/16/32/64, imm8/16/32/64
        // we need to push the RIP pointer by the size of the instruction without RM?????
        // if the instruction doesn't have a ModR/M parameter but does use the rm reg, then we do need to count it
        // but then if it doesn't, then that would be hard to track, so for the time being, we'll have to read it here
        
        return result;
    }
    pub fn run_instruction(&self){

    }
    pub unsafe fn print_instruction(&mut self) -> process_result{

        let prefix = self.get_prefix();
        let ins_result = self.get_instruction();
        if (ins_result.is_none()){ return process_result{state:process_result_state::error, context: Some("invalid/unsupported instruction".to_owned())};}

        let instruction = ins_result.unwrap();
        self.regs.RIP += (instruction.opc_length) as u64;

        let printed_instruction = instruction.name.to_owned();
        // first we get the operand name, then we process each operand into its respective representation

        // for the operands, we have to assume that they're in the correct order to begin with
        // just start with a simple set for now that covers the basic types that occupy the bytes after the opcode

        for op in instruction.params.iter(){
            match (op){
                asm86::operand::r_m8 => {
                    
                },



                _ => {return process_result{state:process_result_state::error, context: Some("no/bad operand??".to_owned())};},
            }
        }
        return process_result{state:process_result_state::error, context: Some("unexpected return".to_owned())};
    }
}

fn print_operand(){

}


// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think