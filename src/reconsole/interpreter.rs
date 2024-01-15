

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
use asm86::r_bits;

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
    pub unsafe fn read_byte(&self, address:u64) -> u8 {*(address as *const u8)}
    pub unsafe fn read_sbyte(&self, address:u64) -> i8 {*(address as *const i8)}
    pub unsafe fn read_sdword(&self, address:u64) -> i32 {*(address as *const i32)}

    pub unsafe fn get_prefix(&mut self) -> u16 {
        let mut curr_prefix:u16 = 0;
        loop{
            let curr_byte = self.read_byte(self.regs.RIP);
            // if it falls within the rex prefix range, then we just take the rex prefix and pop it in ours
            if (curr_byte > 0x40 && curr_byte < 0x50){
                curr_prefix |= asm86::prefixes::rex as u16; // assign generic REX type, next wont do anything if rex is 0x40
                curr_prefix |= (curr_byte & 0b1111) as u16;
            // regular prefixes
            } else{ match curr_byte{
                0x64 => {curr_prefix |= asm86::prefixes::fs as u16;}, 
                0x65 => {curr_prefix |= asm86::prefixes::gs as u16;}, 
                0x66 => {curr_prefix |= asm86::prefixes::operand_size as u16;}, 
                0x67 => {curr_prefix |= asm86::prefixes::address_size as u16;}, 
                0x9B => {curr_prefix |= asm86::prefixes::wait as u16;}, 
                0xF0 => {curr_prefix |= asm86::prefixes::lock as u16;}, 
                0xF2 => {curr_prefix |= asm86::prefixes::repnz as u16;}, 
                0xF3 => {curr_prefix |= asm86::prefixes::repz as u16;}, 
                _ => return curr_prefix
            }}
            // push RIP each successful prefix
            self.regs.RIP += 1;
        }
    }
    pub unsafe fn get_instruction(&self) -> Option<&asm86::instruction>{
        // prefixes (various length)
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

        // this is a terrible solution, ideally we'd read everything without having to loop through params, because then it creates problems like this
        let rmoffs = self.regs.RIP-1;

        for op in instruction.params.iter(){
            let operand_string:&str;
            match (op){
                // R/M32 OPERANDS //
                asm86::operand::r_m8 => { },
                asm86::operand::r_m16 => { },
                asm86::operand::r_m16_32 => {
                },
                asm86::operand::r_m16_32_64 => {
                },
                asm86::operand::r_m32 => { },
                asm86::operand::r_m32_64 => {
                },
                asm86::operand::r_m64 => {
                }
                // REGULAR REGISTER OPERANDS //
                asm86::operand::r8 => {
                    if prefix & (asm86::prefixes::rex as u16) != 0{ // if using any rex extension, then we enable r8_rex mode
                          operand_string = self.print_reg_short(reg_type::r8_rex, prefix,rmoffs);}
                    else{ operand_string = self.print_reg_short(reg_type::r8, prefix,rmoffs);}},
                asm86::operand::r16_32_64 => { 
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{ // prioritize rex, otherwise check for 0x66 operand size override
                          operand_string = self.print_reg_short(reg_type::r64, prefix,rmoffs);}
                    else if prefix & (asm86::prefixes::operand_size as u16) != 0{
                          operand_string = self.print_reg_short(reg_type::r16, prefix,rmoffs);} 
                    else{ operand_string = self.print_reg_short(reg_type::r32, prefix,rmoffs);}},
                asm86::operand::r32_64 => {
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{ // check for rex.w
                          operand_string = self.print_reg_short(reg_type::r64, prefix,rmoffs);}
                    else{ operand_string = self.print_reg_short(reg_type::r32, prefix,rmoffs);}},
                asm86::operand::r64 => {
                    operand_string = self.print_reg_short(reg_type::r64, prefix,rmoffs);}
                // EXTRA REGISTER OPERANDS //
                

                _ => {return process_result{state:process_result_state::error, context: Some("no/bad operand??".to_owned())};},
            }
        }
        return process_result{state:process_result_state::error, context: Some("unexpected return".to_owned())};
    }
    unsafe fn print_reg_short(&self, _type:reg_type, prefix:u16, rmoffs:u64) -> &'static str{
        return print_register_operand((self.read_byte(rmoffs)>>3)&7, _type, prefix&(asm86::prefixes::rex_r as u16)!=0);
    }
    unsafe fn print_rm_operand(&mut self, mut data:u8, data_type:reg_type, prefix:u16) -> &'static str{
        // read mod bits
        let mod_bits = data >> 5;
        let rm_bits:u8 = data & 0b111;
        let use_extension_set = (prefix & (asm86::prefixes::rex_b as u16)) != 0;
        let demote_rm_reg = (prefix & (asm86::prefixes::operand_size as u16)) != 0;
    
        if mod_bits == 3{
            return print_register_operand(rm_bits, data_type, use_extension_set);}
        let reg:&str;
        let disp:String;
        let sib:String;
        match rm_bits{
            0 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R8D";} 
                    else {reg = "R8";}} 
                else {
                    if demote_rm_reg{ reg = "EAX";} 
                    else {reg = "RAX";}}}
            1 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R9D";} 
                    else {reg = "R9";}} 
                else {
                    if demote_rm_reg{ reg = "ECX";} 
                    else {reg = "RCX";}}}
            2 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R10D";} 
                    else {reg = "R10";}} 
                else {
                    if demote_rm_reg{ reg = "EDX";} 
                    else {reg = "RDX";}}}
            3 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R11D";} 
                    else {reg = "R11";}} 
                else {
                    if demote_rm_reg{ reg = "EBX";} 
                    else {reg = "RBX";}}}
            5 => { // works differently for Mod 0b00
                // NOTE: lets just ignore the proper way to do this (RIP/EIP + disp32)
                if mod_bits == 0 {
                    let result = self.read_sdword(self.regs.RIP);
                    self.regs.RIP += 4;
                    // convert to number and pop into string
                    disp = "offs:" + result.to_string();
                }else {
                    if use_extension_set{
                        if demote_rm_reg{ reg = "R13D";} 
                        else {reg = "R13";}} 
                    else {
                        if demote_rm_reg{ reg = "EBP";} 
                        else {reg = "RBP";}}}}
            6 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R14D";} 
                    else {reg = "R14";}} 
                else {
                    if demote_rm_reg{ reg = "ESI";} 
                    else {reg = "RSI";}}}
            7 => {
                if use_extension_set{
                    if demote_rm_reg{ reg = "R15D";} 
                    else {reg = "R15";}} 
                else {
                    if demote_rm_reg{ reg = "EDI";} 
                    else {reg = "RDI";}}}
            4 => { // SIB BYTE
                let sib_reg:&str;
                let sib_multiplier:&str;
                let sib_base:&str; // this can be a displacement
    
    
            } 
        }
        // then we have to apply the displacement if mod is 0b01 || 0b10
    
        return reg; // placeholder
    }
}

enum reg_type{
    r8,
    r8_rex,
    r16,
    r32,
    r64,
    mm,
    xmm,
    sreg,
    // eee, // unsure how this one is supposed to work
}
fn print_register_operand(mut data:u8, data_type:reg_type, use_extension_set:bool) -> &'static str{
    if data > 7 {return "ERROR:overflow"}
    if use_extension_set {data |= 8} // extend the bit
    match data_type{
        reg_type::r8 => { match data{
            0 => return "AL",
            1 => return "CL",
            2 => return "DL",
            3 => return "BL",
            4 => return "AH",
            5 => return "CH",
            6 => return "DH",
            7 => return "BH",
            8  => return "R8B", // not used in the reg part of ModR/M, but we're reusing this for the r/m part mod_11, which doesn't upgrade the set but can access rex registers
            9  => return "R9B",
            10 => return "R10B",
            11 => return "R11B",
            12 => return "R12B",
            13 => return "R13B",
            14 => return "R14B",
            15 => return "R15B",
            _ => return "ERROR"}}
        reg_type::r8_rex => { match data{
            0  => return "AL",
            1  => return "CL",
            2  => return "DL",
            3  => return "BL",
            4  => return "SPL",
            5  => return "BPL",
            6  => return "SIL",
            7  => return "DIL",
            8  => return "R8B",
            9  => return "R9B",
            10 => return "R10B",
            11 => return "R11B",
            12 => return "R12B",
            13 => return "R13B",
            14 => return "R14B",
            15 => return "R15B",
            _ => return "ERROR"}}
        reg_type::r16 => { match data{
            0  => return "AX",
            1  => return "CX",
            2  => return "DX",
            3  => return "BX",
            4  => return "SP",
            5  => return "BP",
            6  => return "SI",
            7  => return "DI",
            8  => return "R8W",
            9  => return "R9W",
            10 => return "R10W",
            11 => return "R11W",
            12 => return "R12W",
            13 => return "R13W",
            14 => return "R14W",
            15 => return "R15W",
            _ => return "ERROR"}}
        reg_type::r32 => { match data{
            0  => return "EAX",
            1  => return "ECX",
            2  => return "EDX",
            3  => return "EBX",
            4  => return "ESP",
            5  => return "EBP",
            6  => return "ESI",
            7  => return "EDI",
            8  => return "R8D",
            9  => return "R9D",
            10 => return "R10D",
            11 => return "R11D",
            12 => return "R12D",
            13 => return "R13D",
            14 => return "R14D",
            15 => return "R15D",
            _ => return "ERROR"}}
        reg_type::r64 => { match data{
            0  => return "RAX",
            1  => return "RCX",
            2  => return "RDX",
            3  => return "RBX",
            4  => return "RSP",
            5  => return "RBP",
            6  => return "RSI",
            7  => return "RDI",
            8  => return "R8",
            9  => return "R9",
            10 => return "R10",
            11 => return "R11",
            12 => return "R12",
            13 => return "R13",
            14 => return "R14",
            15 => return "R15",
            _ => return "ERROR"}}
        reg_type::mm => { match data & 0b111{ // mm cant get extended
            0  => return "MM0",
            1  => return "MM1",
            2  => return "MM2",
            3  => return "MM3",
            4  => return "MM4",
            5  => return "MM5",
            6  => return "MM6",
            7  => return "MM7",
            _ => return "ERROR"}}
        reg_type::xmm => { match data {
            0  => return "XMM0",
            1  => return "XMM1",
            2  => return "XMM2",
            3  => return "XMM3",
            4  => return "XMM4",
            5  => return "XMM5",
            6  => return "XMM6",
            7  => return "XMM7",
            8  => return "XMM8",
            9  => return "XMM9",
            10 => return "XMM10",
            11 => return "XMM11",
            12 => return "XMM12",
            13 => return "XMM13",
            14 => return "XMM14",
            15 => return "XMM15",
            _ => return "ERROR"}}
        reg_type::sreg => { match data & 0b111 { // sreg doesn't have an extension
            0  => return "ES",
            1  => return "CS",
            2  => return "SS",
            3  => return "DS",
            4  => return "FS",
            5  => return "GS",
            6  => return "res",
            7  => return "res",
            _ => return "ERROR"}}
        _ => {return "ERROR:unimplemented"}
    }
}




// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think