



pub struct asm_registers{
    pub RAX:u64,
    pub RBX:u64,
    pub RCX:u64,
    pub RDX:u64,
    pub RBP:u64,
    pub RSP:u64,
    pub RSI:u64,
    pub RDI:u64,
     
    pub R8:u64,
    pub R9:u64,
    pub R10:u64,
    pub R11:u64,
    pub R12:u64,
    pub R13:u64,
    pub R14:u64,
    pub R15:u64,
     
    pub RIP:u64,
     
    pub RFLAGS:u64,
 
    pub LastError:u32,
    pub LastStatus:u32,
 
    pub GS:u16,
    pub FS:u16,
    pub ES:u16,
    pub DS:u16,
    pub CS:u16,
    pub SS:u16,
     
    // x87 stuff??
    pub x87r0:u64,
    pub x87r1:u64,
    pub x87r2:u64,
    pub x87r3:u64,
    pub x87r4:u64,
    pub x87r5:u64,
    pub x87r6:u64,
    pub x87r7:u64,
 
    pub x87TagWord:u16,
    pub x87StatusWord:u16,
    pub x87ControlWord:u16,
    pub MxCsr:u32,
 
    // XMM0 - XMM15
    // YMM0 - YMM15
 
    // debug registers
    pub DR0:u64,
    pub DR1:u64,
    pub DR2:u64,
    pub DR3:u64,
    pub DR4:u64,
    pub DR5:u64,
    pub DR6:u64,
    pub DR7:u64,
}
impl asm_registers{
    //fn get_AX(&self) -> u16{
    //    return self.RAX & ;
    //}
    pub fn new() -> asm_registers{
        asm_registers{
            RAX:0,RBX:0,RCX:0,RDX:0,RBP:0,RSP:0,RSI:0,RDI:0,
            R8:0,R9:0,R10:0,R11:0,R12:0,R13:0,R14:0,R15:0,
            RIP:0,
            RFLAGS:0,
            LastError: 0, LastStatus:0, 
            GS:0, FS:0, ES:0, DS:0, CS:0, SS:0,
            x87r0:0, x87r1:0, x87r2:0, x87r3:0, x87r4:0, x87r5:0, x87r6:0, x87r7:0,
            x87TagWord: 0, x87StatusWord:0, x87ControlWord:0, MxCsr:0,
            DR0:0, DR1:0, DR2:0, DR3:0, DR4:0, DR5:0, DR6:0, DR7:0
        }
    }
    
}

// all getter & setter functions

// debug registers function