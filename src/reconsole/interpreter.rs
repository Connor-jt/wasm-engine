

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
//use asm86::r_bits;
static mut proc_address:u64 = 0;

pub struct loaded_file{
    pub name:String,
    pub data:Vec<u8>,
}
static mut last_id:u64 = 0;
impl loaded_file{
    pub unsafe fn load_file(&self) -> Result<running_process, String> {
        let loaded: loaded_exe = executable::load_exe(&self.data)?;
        // we need to config some basic data here
        //let mut var = asm_registers::new();
        //var.RIP = loaded.runtime_data.as_ptr().add(loaded.header.optional_header.address_of_entry_point as usize) as u64;
        let ptr = loaded.runtime_data.as_ptr().add(loaded.header.optional_header.address_of_entry_point as usize) as u64;
        // idk what else
        last_id += 1;
        return Ok(running_process{name:self.name.to_owned(), id:last_id, exe:loaded, curr_address:ptr});
    }
}

pub struct running_process{
    pub name:String,
    pub id:u64,
    pub exe:loaded_exe,
    pub curr_address:u64
    //pub regs:asm_registers,
}

// pub struct process_result{
//     pub state:process_result_state,
//     pub context:Option<String>,
//     //data:Option<Vec<u8>>, // potentially unneeded
// }
// pub enum process_result_state{
//     running,
//     print,
//     breakpoint,
//     warning,
//     error,
//     exit,

//     unimplemented, // this refers to loading dll's, reading/writing files. etc
// }
impl Iterator for running_process{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { // apparently this function is not allowed to be unsafe, thanks john rust
            proc_address = self.curr_address;
            // run thing
            let result = self.print_instruction();
            // update process pointer
            self.curr_address = proc_address; 
            return result;
}}}
impl running_process{
    pub unsafe fn run_single(&mut self) ->Option<String>{
        proc_address = self.curr_address;
        let result = self.print_instruction();
        self.curr_address = proc_address; 
        return result;
    }

    pub unsafe fn read_byte(&self, address:u64) -> u8 {*(address.to_owned() as *const u8)}
    pub unsafe fn read_sbyte(&self, address:u64) -> i8 {*(address.to_owned() as *const i8)}
    pub unsafe fn read_word(&self, address:u64) -> u16 {*(address.to_owned() as *const u16)}
    pub unsafe fn read_sword(&self, address:u64) -> i16 {*(address.to_owned() as *const i16)}
    pub unsafe fn read_dword(&self, address:u64) -> u32 {*(address.to_owned() as *const u32)}
    pub unsafe fn read_sdword(&self, address:u64) -> i32 {*(address.to_owned() as *const i32)}
    pub unsafe fn read_qword(&self, address:u64) -> u64 {*(address.to_owned() as *const u64)}

    pub unsafe fn get_prefix(&self) -> u16 {
        let mut curr_prefix:u16 = 0;
        loop{
            let curr_byte = self.read_byte(proc_address);
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
            proc_address += 1;
        }
    }
    pub unsafe fn get_instruction(&self) -> Option<&asm86::instruction>{
        // prefixes (various length)
        // read RIP pointer and get first 3 bytes?
        let result = asm86::get_instruction(self.read_byte(proc_address), self.read_byte(proc_address+1), self.read_byte(proc_address+2));
        
        // then read any required, RM byte, SIB, disp8/16/32/64, imm8/16/32/64
        // we need to push the RIP pointer by the size of the instruction without RM?????
        // if the instruction doesn't have a ModR/M parameter but does use the rm reg, then we do need to count it
        // but then if it doesn't, then that would be hard to track, so for the time being, we'll have to read it here
        return result;
    }
    // pub fn run_instruction(&self){
    // }
    pub unsafe fn print_instruction(&self) -> Option<String>{
        let mut starting_offset = proc_address; // mut because we reuse it to count upwards to our current address to get all the operand bytes
        let prefix = 0; //self.get_prefix();
        let ins_result = self.get_instruction();
        if (ins_result.is_none()){ return None;}
        let instruction = ins_result.unwrap().to_owned();

        proc_address += (instruction.opc_length) as u64;

        let mut printed_instruction = instruction.name.to_owned();
        // first we get the operand name, then we process each operand into its respective representation

        // for the operands, we have to assume that they're in the correct order to begin with
        // just start with a simple set for now that covers the basic types that occupy the bytes after the opcode

        // this is a terrible solution, ideally we'd read everything without having to loop through params, because then it creates problems like this
        let rmoffs = proc_address-1;
        let mut is_first: bool = true;
        for op in instruction.params.iter(){
            if !is_first {printed_instruction += ","} // put comma in to separate params
            printed_instruction += " "; // separate all elements with space

            let operand_string:String;
            match (op){
                // R/M32 OPERANDS //
                asm86::operand::r_m8 => { 
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r8, prefix);},
                asm86::operand::r_m16 => { 
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r16, prefix);},
                asm86::operand::r_m16_32 => {
                    if prefix & (asm86::prefixes::operand_size as u16) != 0{ 
                          operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r16, prefix);}
                    else{ operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r32, prefix);}},
                asm86::operand::r_m16_32_64 => {
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{ // prioritize rex, otherwise check for 0x66 operand size override
                          operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r64, prefix);}
                    else if prefix & (asm86::prefixes::operand_size as u16) != 0{
                          operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r16, prefix);} 
                    else{ operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r32, prefix);}},
                asm86::operand::r_m32 => { 
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r32, prefix);},
                asm86::operand::r_m32_64 => {
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{ // check for rex.w
                          operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r64, prefix);}
                    else{ operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r32, prefix);}},
                asm86::operand::r_m64 => {
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::r64, prefix);},
                // EXTRA R/M32 OPERANDS // (what are the r/m16 operands??)
                asm86::operand::mm_m64 => { // what are we supposed to do with the memory size information??? 
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::mm64, prefix);},
                asm86::operand::xmm_m32 => {
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::xmm32, prefix);},
                asm86::operand::xmm_m64 => {
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::xmm64, prefix);},
                asm86::operand::xmm_m128 => {
                    operand_string = self.print_rm_operand(self.read_byte(rmoffs), reg_type::xmm128, prefix);},

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
                    operand_string = self.print_reg_short(reg_type::r64, prefix,rmoffs);},
                // EXTRA REGISTER OPERANDS //
                asm86::operand::mm => {
                    operand_string = self.print_reg_short(reg_type::mm, prefix,rmoffs);},
                asm86::operand::xmm => {
                    operand_string = self.print_reg_short(reg_type::xmm, prefix,rmoffs);},
                
                // IMMEDIATE OPERANDS //
                asm86::operand::imm8 => {
                    operand_string = self.read_byte(proc_address).to_string();
                    proc_address += 1;},
                asm86::operand::imm16 => {
                    operand_string = self.read_word(proc_address).to_string();
                    proc_address += 2;},
                asm86::operand::imm16_32 => {
                    if prefix & (asm86::prefixes::operand_size as u16) != 0{
                        operand_string = self.read_word(proc_address).to_string();
                        proc_address += 2;}
                    else{ 
                        operand_string = self.read_dword(proc_address).to_string();
                        proc_address += 4;}},
                asm86::operand::imm16_32_64 => {
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{
                        operand_string = self.read_qword(proc_address).to_string();
                        proc_address += 8;}
                    else if prefix & (asm86::prefixes::operand_size as u16) != 0{
                        operand_string = self.read_word(proc_address).to_string();
                        proc_address += 2;}
                    else{ 
                        operand_string = self.read_dword(proc_address).to_string();
                        proc_address += 4;}},
                // rel imm's
                asm86::operand::rel8 => {
                    operand_string = "RIP+".to_owned() + &(self.read_byte(proc_address).to_string());
                    proc_address += 1;
                },
                asm86::operand::rel16_32 => {
                    if prefix & (asm86::prefixes::rex_w as u16) != 0{ // 16bit
                        operand_string = "RIP+".to_owned() + &(self.read_sword(proc_address).to_string());
                        proc_address += 1;} 
                    else {
                        operand_string = "RIP+".to_owned() + &(self.read_sdword(proc_address).to_string());
                        proc_address += 1;}},
                // moffs imm's
                asm86::operand::moffs8 => {
                    let num:String;
                    if  prefix & (asm86::prefixes::address_size as u16) != 0{
                          num = self.read_dword(proc_address).to_string(); proc_address += 4;} // 32
                    else {num = self.read_qword(proc_address).to_string(); proc_address += 8;} // 64
                    operand_string = "byte ptr [".to_owned() + &num + "]";},
                asm86::operand::moffs16_32_64 => {
                    let num:String;
                    if  prefix & (asm86::prefixes::address_size as u16) != 0{
                          num = self.read_dword(proc_address).to_string(); proc_address += 4;} // 32
                    else{ num = self.read_qword(proc_address).to_string(); proc_address += 8;} // 64

                    if prefix & (asm86::prefixes::rex_w as u16) != 0{
                        operand_string = "qword ptr [".to_owned() + &num + "]";} // 64
                    else if prefix & (asm86::prefixes::operand_size as u16) != 0{
                          operand_string = "word ptr [".to_owned() + &num + "]";} // 16
                    else{ operand_string = "dword ptr [".to_owned() + &num + "]";} // 32
                },

                // AUTO GENERATED TYPES // (the a_types_convert script packages these because i haven't got explicit functionality yet)
                asm86::operand::XMM0=>{operand_string="XMM0".to_owned();},
                asm86::operand::rAX=>{operand_string="_rAX".to_owned();},
                asm86::operand::eAX=>{operand_string="_eAX".to_owned();},
                asm86::operand::eCX=>{operand_string="_eCX".to_owned();},
                asm86::operand::eDX=>{operand_string="_eDX".to_owned();},
                asm86::operand::eBX=>{operand_string="_eBX".to_owned();},
                asm86::operand::eSP=>{operand_string="_eSP".to_owned();},
                asm86::operand::eBP=>{operand_string="_eBP".to_owned();},
                asm86::operand::eSI=>{operand_string="_eSI".to_owned();},
                asm86::operand::eDI=>{operand_string="_eDI".to_owned();},
                asm86::operand::rCX=>{operand_string="_rCX".to_owned();},
                asm86::operand::rBP=>{operand_string="_rBP".to_owned();},
                asm86::operand::rDX=>{operand_string="_rDX".to_owned();},
                asm86::operand::AL=>{operand_string="AL".to_owned();},  
                asm86::operand::CL=>{operand_string="CL".to_owned();},  
                asm86::operand::DL=>{operand_string="DL".to_owned();},  
                asm86::operand::BL=>{operand_string="BL".to_owned();},  
                asm86::operand::CH=>{operand_string="CH".to_owned();},  
                asm86::operand::DH=>{operand_string="DH".to_owned();},
                asm86::operand::BH=>{operand_string="BH".to_owned();},
                asm86::operand::AH=>{operand_string="AH".to_owned();},
                asm86::operand::AX=>{operand_string="AX".to_owned();},
                asm86::operand::DX=>{operand_string="DX".to_owned();},
                asm86::operand::EDX=>{operand_string="EDX".to_owned();},
                asm86::operand::EAX=>{operand_string="EAX".to_owned();},
                asm86::operand::ECX=>{operand_string="ECX".to_owned();},
                asm86::operand::RSP=>{operand_string="RSP".to_owned();},
                asm86::operand::RCX=>{operand_string="RCX".to_owned();},
                asm86::operand::R11=>{operand_string="R11".to_owned();},
                asm86::operand::m=>{operand_string="m".to_owned();},
                asm86::operand::m8=>{operand_string="m8".to_owned();},
                asm86::operand::m16=>{operand_string="m16".to_owned();},
                asm86::operand::m16_32=>{operand_string="m16_32".to_owned();},
                asm86::operand::m16_32_64=>{operand_string="m16_32_64".to_owned();},
                asm86::operand::m32=>{operand_string="m32".to_owned();},
                asm86::operand::m32_64=>{operand_string="m32_64".to_owned();},
                asm86::operand::m64=>{operand_string="m64".to_owned();},
                asm86::operand::m128=>{operand_string="m128".to_owned();},
                asm86::operand::m512=>{operand_string="m512".to_owned();},
                asm86::operand::m16_16_32_64=>{operand_string="m16_16_32_64".to_owned();},
                asm86::operand::Sreg=>{operand_string="Sreg".to_owned();},
                asm86::operand::_1=>{operand_string="1".to_owned();},
                asm86::operand::_3=>{operand_string="3".to_owned();},
                asm86::operand::Flags=>{operand_string="Flags".to_owned();},
                asm86::operand::eFlags=>{operand_string="eFlags".to_owned();},
                asm86::operand::EFlags=>{operand_string="EFlags".to_owned();},
                asm86::operand::m14_28=>{operand_string="m14_28".to_owned();},
                asm86::operand::m94_108=>{operand_string="m94_108".to_owned();},
                asm86::operand::m16int=>{operand_string="m16int".to_owned();},
                asm86::operand::m32int=>{operand_string="m32int".to_owned();},
                asm86::operand::m64int=>{operand_string="m64int".to_owned();},
                asm86::operand::m80dec=>{operand_string="m80dec".to_owned();},
                asm86::operand::m32real=>{operand_string="m32real".to_owned();},
                asm86::operand::m80real=>{operand_string="m80real".to_owned();},
                asm86::operand::m64real=>{operand_string="m64real".to_owned();},
                asm86::operand::ST=>{operand_string="ST".to_owned();},
                asm86::operand::STi=>{operand_string="STi".to_owned();},
                asm86::operand::STi_m32real=>{operand_string="STi_m32real".to_owned();},
                asm86::operand::ST1=>{operand_string="ST1".to_owned();},
                asm86::operand::ST2=>{operand_string="ST2".to_owned();},
                asm86::operand::SS=>{operand_string="SS".to_owned();},
                asm86::operand::GS=>{operand_string="GS".to_owned();},
                asm86::operand::FS=>{operand_string="FS".to_owned();},
                asm86::operand::LDTR=>{operand_string="LDTR".to_owned();},
                asm86::operand::TR=>{operand_string="TR".to_owned();},
                asm86::operand::GDTR=>{operand_string="GDTR".to_owned();},
                asm86::operand::IDTR=>{operand_string="IDTR".to_owned();},
                asm86::operand::XCR=>{operand_string="XCR".to_owned();},
                asm86::operand::MSW=>{operand_string="MSW".to_owned();},
                asm86::operand::MSR=>{operand_string="MSR".to_owned();},
                asm86::operand::PMC=>{operand_string="PMC".to_owned();},
                asm86::operand::CR0=>{operand_string="CR0".to_owned();},
                asm86::operand::CRn=>{operand_string="CRn".to_owned();},
                asm86::operand::DRn=>{operand_string="DRn".to_owned();},
                asm86::operand::IA32_KERNEL_GSBASE=>{operand_string="IA32_KERNEL_GSBASE".to_owned();},
                asm86::operand::IA32_TIME_STAMP_COUNTER=>{operand_string="IA32_TIME_STAMP_COUNTER".to_owned();},
                asm86::operand::IA32_SYSENTER_CS=>{operand_string="IA32_SYSENTER_CS".to_owned();},
                asm86::operand::IA32_BIOS_SIGN_ID=>{operand_string="IA32_BIOS_SIGN_ID".to_owned();},
                asm86::operand::r=>{operand_string="r".to_owned();},
                asm86::operand::r_m=>{operand_string="r_m".to_owned();},
                _ => {operand_string="ERROR".to_owned();},
            }
            
            printed_instruction += &operand_string;
            is_first = false;
        }
        // we want to write in the line of the instruction and the instruction bytes
        let mut instruction_info = format!("{:x}", starting_offset) + " | ";
        while starting_offset < proc_address{
            instruction_info += &format!("{:x} ", self.read_byte(starting_offset));
            starting_offset += 1;}
        instruction_info += "| ";
        // combine header and stuff into results
        return Some(instruction_info + &printed_instruction);
    }
    unsafe fn print_reg_short(&self, _type:reg_type, prefix:u16, rmoffs:u64) -> String{
        return print_register_operand((self.read_byte(rmoffs)>>3)&0b111, _type, prefix&(asm86::prefixes::rex_r as u16)!=0).to_owned();
    }
    unsafe fn print_rm_operand(&self, data:u8, data_type:reg_type, prefix:u16) -> String{
        // read mod bits
        let mod_bits = data >> 6;
        let rm_bits:u8 = data & 0b111;
        let use_extension_set = (prefix & (asm86::prefixes::rex_b as u16)) != 0;
        let demote_rm_reg = (prefix & (asm86::prefixes::address_size as u16)) != 0;
    
        if mod_bits == 3{
            return print_register_operand(rm_bits, data_type, use_extension_set).to_owned();}
        
        let reg:String;
        let mut disp:Option<String> = None;

        if rm_bits == 5 && mod_bits == 0{ // relative offset
            disp = Some(self.read_sdword(proc_address).to_string());
            proc_address += 4;
            if demote_rm_reg{ reg = "EIP".to_owned();} 
            else {reg = "RIP".to_owned();}} 
        else if rm_bits == 4{ // SIB byte
            let sib_base:String; // this can be a displacement
            let sib_reg:Option<&str>;
            let sib_multiplier:Option<&str>;

            let sib_data = self.read_byte(proc_address);
            proc_address += 1;

            let sib_base_bits = sib_data & 0b111;
            let sib_reg_bits: u8 = (sib_data >> 3) & 0b111;
            let use_sib_reg_extension = (prefix & (asm86::prefixes::rex_x as u16)) != 0;

            // match sib SS
            match sib_data >> 6{
                0 => {sib_multiplier = None;}
                1 => {sib_multiplier = Some("*2");}
                2 => {sib_multiplier = Some("*4");}
                3 => {sib_multiplier = Some("*8");}
                _ => {sib_multiplier = None;}} // error
            // match sib base
            if mod_bits == 0 && sib_base_bits == 5 { // then its a baseless displacement
                sib_base = self.read_dword(proc_address).to_string();
                proc_address += 4;}
            else {
                if demote_rm_reg {sib_base = print_register_operand(sib_base_bits, reg_type::r32, use_extension_set).to_owned();}
                else {sib_base = print_register_operand(sib_base_bits, reg_type::r64, use_extension_set).to_owned();}}
            // match sib reg
            if (sib_reg_bits == 4){ 
                sib_reg = None;} // RSP is replaced with the option for 'none'
            else{ 
                if demote_rm_reg { sib_reg = Some(print_register_operand(sib_reg_bits, reg_type::r32, use_sib_reg_extension));}
                else { sib_reg = Some(print_register_operand(sib_reg_bits, reg_type::r64, use_sib_reg_extension));}}
            
            // resolve stuff into reg string
            if sib_reg.is_some(){reg = sib_base + " + " + sib_reg.unwrap() + sib_multiplier.unwrap()}
            else {reg = sib_base}

        }else { // regular register
            if demote_rm_reg {reg = print_register_operand(rm_bits, reg_type::r32, use_extension_set).to_owned();}
            else {reg = print_register_operand(rm_bits, reg_type::r64, use_extension_set).to_owned();}}

        // then we have to apply the displacement if mod is 0b01 || 0b10
        if mod_bits == 1{
            disp = Some(self.read_sbyte(proc_address).to_string());
            proc_address += 1;}
        else if mod_bits == 2{
            disp = Some(self.read_sdword(proc_address).to_string());
            proc_address += 4;}
        
        let final_string:String;
        if disp.is_none(){ final_string = reg_type_string(data_type).to_owned()+" ptr ["+&reg+" + "+&disp.unwrap()+"]";} 
        else{              final_string = reg_type_string(data_type).to_owned()+" ptr ["+&reg+"]";}
        return final_string; // placeholder
    }
}

enum reg_type{
    r8 = 0,
    r8_rex,
    r16,
    r32,
    r64,
    mm,
    mm64,
    xmm,
    xmm32,
    xmm64,
    xmm128,
    sreg,
    // eee, // unsure how this one is supposed to work
}
fn reg_type_string(data_type:reg_type) -> &'static str{
    match data_type{
        reg_type::r8 => "byte",
        reg_type::r16 => "word",
        reg_type::r32 => "dword",
        reg_type::r64 => "qword",
        reg_type::mm => "mm",
        reg_type::mm64 => "mm64",
        reg_type::xmm => "xmm",
        reg_type::xmm32 => "xmm32",
        reg_type::xmm64 => "xmm64",
        reg_type::xmm128 => "xmm128",
        reg_type::sreg => "sreg",

        reg_type::r8_rex => "ERROR",
    }
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
        // why is this language like this??? why can i not just convert data_type mm64 to mm instead??
        reg_type::mm => { return mm_reg_print(data);}
        reg_type::mm64 => { return mm_reg_print(data);}
        reg_type::xmm => { return xmm_reg_print(data);}
        reg_type::xmm32 => { return xmm_reg_print(data);}
        reg_type::xmm64 => { return xmm_reg_print(data);}
        reg_type::xmm128 => { return xmm_reg_print(data);}
        _ => {return "ERROR:unimplemented"}
    }
}
fn mm_reg_print(data:u8) -> &'static str{
    match data & 0b111{ // mm cant get extended
        0  => return "MM0",
        1  => return "MM1",
        2  => return "MM2",
        3  => return "MM3",
        4  => return "MM4",
        5  => return "MM5",
        6  => return "MM6",
        7  => return "MM7",
        _ => return "ERROR"}}
fn xmm_reg_print(data:u8) -> &'static str{
    match data {
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

// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think