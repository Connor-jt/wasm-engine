// these are notes taken from the intel x86 document
// so i can effectively note down all the opcodes and their functionality

// 'prefixes'
// 'group 1'
// 0xF0 - 'Lock'
// 0xF2 - 'REPNE/REPNZ', also 'BND'
// 0xF3 - 'REP or REPE/REPZ'
// 'group 2'
// supposedly these don't exist in 64 bit mode?
// // 0x2E - 'CS', also 'Branch not taken'
// // 0x36 - 'SS'
// // 0x3E - 'DS', also 'Branch taken'
// // 0x26 - 'ES'
// 0x64 - 'FS'
// 0x65 - 'GS'
// 'group 3'
// 0x66 - 'Operand size overide'
// 'group 4'
// 0x67 - 'Address size overide'

// do rex thing? (the specifications weren't too clear however)


// 0x9B - 'wait' ???

// 0x0F - extension opcode?



// do all regular opcodes + whatever data they need


// prefix bytes // 4-5 bytes??
// rex byte?
// opcode bytes // 1-3
// mod rm // 1 byte
// sib // 1 byte 
// displacement // 1-8 bytes
// immediate // 1-8 bytes



macro_rules! r_bits{ ($l:tt) => { ($l & 0b00111000) >> 3; }}

struct opcode{
    op1:opperand,
    op2:opperand,
    op3:opperand,
    op4:opperand,

    func:u64, // placeholder function reference

    // error checking stuff
    has_rm:bool,

    // debug info (we'll have a function to gen)
    bc1:u8, // byte0
    bc2:Option<u8>, 
    bc3:Option<u8>,
    bc4_reg:Option<u8>, // the 3 bits used in the r/m byte // 0-7

    name:&str,
}


enum operands{
    test1,
    test2,
}


fn get_instruction(byte1:u8, byte2:u8, byte3:u8, byte4:u8) -> i32{
    match byte1{
        0 => { return 0}
        _ => {return -1}
    }



}