



struct asm_registers{
    RAX:u64,
    RBX:u64,
    RCX:u64,
    RDX:u64,
    RBP:u64,
    RSP:u64,
    RSI:u64,
    RDI:u64,
    
    R8:u64,
    R9:u64,
    R10:u64,
    R11:u64,
    R12:u64,
    R13:u64,
    R14:u64,
    R15:u64,
    
    RIP:u64,
    
    RFLAGS:u64,

    LastError:u32,
    LastStatus:u32,

    GS:u16,
    FS:u16,
    ES:u16,
    DS:u16,
    CS:u16,
    SS:u16,
    
    // x87 stuff??
    x87r0:u64,
    x87r1:u64,
    x87r2:u64,
    x87r3:u64,
    x87r4:u64,
    x87r5:u64,
    x87r6:u64,
    x87r7:u64,

    x87TagWord:u16,
    x87StatusWord:u16,
    x87ControlWord:u16,
    MxCsr:u32,

    // XMM0 - XMM15
    // YMM0 - YMM15

    // debug registers
    DR0:u64,
    DR1:u64,
    DR2:u64,
    DR3:u64,
    DR4:u64,
    DR5:u64,
    DR6:u64,
    DR7:u64,
}
impl asm_registers{
    //fn get_AX(&self) -> u16{
    //    return self.RAX & ;
    //}

}