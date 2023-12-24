// these are notes taken from the intel x86 document
// so i can effectively note down all the opcodes and their functionality

// 'prefixes'
// 'group 1'
// 0xF0 - 'Lock'
// 0xF2 - 'REPNE/REPNZ', also 'BND'
// 0xF3 - 'REP or REPE/REPZ'
// 'group 2'
// 0x2E - 'CS', also 'Branch not taken'
// 0x36 - 'SS'
// 0x3E - 'DS', also 'Branch taken'
// 0x26 - 'ES'
// 0x64 - 'FS'
// 0x65 - 'GS'
// 'group 3'
// 0x66 - 'Operand size overide'
// 'group 4'
// 0x67 - 'Address size overide'

// do rex thing? (the specifications weren't too clear however)




// 0x0F - extension opcode?



// do all regular opcodes + whatever data they need