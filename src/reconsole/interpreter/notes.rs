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


// ////////////////// //
// ACTUAL CODE BELOW //
// //////////////// //


macro_rules! r_bits{ ($l:tt) => { ($l & 0b00111000) >> 3; }}

// just to save some text space
fn get_instruction(byte1:u8, byte2:u8, byte3:u8, byte4:u8) -> Option<&'static instruction>{
    let opcode_index = get_instruction_index(byte1, byte2, byte3, byte4);
    if opcode_index.is_none(){return None};
    return Some(&INSTRUCTIONS[opcode_index.unwrap() as usize]);
}

struct instruction{
    name:String,
    params:Vec<operand>,
    //func:u64, // placeholder function reference
    rm_byte:rm_type,

    // debug data
    opc1:u8,
    opc2:Option<u8>, 
    opc3:Option<u8>,
    opc4_reg:Option<u8>, // the 3 or 5 bits used in the r/m byte // 0-4, 0-7 // i dont think its just 5 bits, it could be the whole byte?
    // we also need the prefix byte, but i dont think we're going to support that for this implementation
}

enum rm_type{
    none = 0,
    available = 1,
    reg_opcode = 2,
    full_opcode = 3
}

enum prefixes{
	rex_b = 1, //0x41: REX.B //Extension of r/m field, base field, or opcode reg field
	rex_x = 2, //0x42: REX.X //Extension of SIB index field
	rex_r = 4, //0x44: REX.R //Extension of ModR/M reg field
	rex_w = 8, //0x48: REX.W //64 Bit Operand Size
	fs = 16, //0x64: FS //FS segment override prefix
	gs = 32, //0x65: GS //GS segment override prefix
	operand_size = 64, //0x66: no mnemonic //Operand-size override prefix & Precision-size override prefix
	address_size = 128, //0x67: no mnemonic //Address-size override prefix
	wait = 256, //0x9B: no mnemonic //Wait Prefix
	lock = 512, //0xF0: LOCK //Assert LOCK# Signal Prefix
	repnz = 1024, //0xF2: REPNZ //Repeat String Operation Prefix
	// 0xF2: REP //Repeat String Operation Prefix
	// 0xF2: no mnemonic //Scalar Double-precision Prefix
	repz = 2048, //0xF3: REPZ //Repeat String Operation Prefix
	// 0xF3: REP //Repeat String Operation Prefix
	// 0xF3: no mnemonic //Scalar Single-precision Prefix
}

// /////////////////////////////////////////////// //
// WARNING: CODE BELOW IS AUTOMATICALLY GENERATED //
// ///////////////////////////////////////////// //

enum operand{
    r_m8 = 0,
    r8 = 1,
    r_m16_32 = 2,
    r16_32 = 3,
    AL = 4,
    imm8 = 5,
    eAX = 6,
    imm16_32 = 7,
    ES = 8,
    CS = 9,
    SS = 10,
    DS = 11,
    AH = 12,
    eCX = 13,
    eDX = 14,
    eBX = 15,
    eSP = 16,
    eBP = 17,
    eSI = 18,
    eDI = 19,
    AX = 20,
    CX = 21,
    DX = 22,
    EAX = 23,
    ECX = 24,
    EDX = 25,
    DI = 26,
    SI = 27,
    BP = 28,
    EDI = 29,
    ESI = 30,
    EBP = 31,
    m16_32 = 32,
    eFlags = 33,
    r_m16 = 34,
    r16 = 35,
    m8 = 36,
    m16 = 37,
    rel8 = 38,
    Sreg = 39,
    m = 40,
    ptr16_16_32 = 41,
    Flags = 42,
    EFlags = 43,
    moffs8 = 44,
    moffs16_32 = 45,
    CL = 46,
    DL = 47,
    BL = 48,
    CH = 49,
    DH = 50,
    BH = 51,
    imm16 = 52,
    m16_16_32 = 53,
    _3 = 54,
    _1 = 55,
    ST = 56,
    m32real = 57,
    STi_m32real = 58,
    ST1 = 59,
    STi = 60,
    m14_28 = 61,
    m32int = 62,
    m80real = 63,
    m64real = 64,
    m64int = 65,
    ST2 = 66,
    m94_108 = 67,
    m16int = 68,
    m80dec = 69,
    rel16_32 = 70,
    LDTR = 71,
    TR = 72,
    GDTR = 73,
    IDTR = 74,
    XCR = 75,
    MSW = 76,
    CR0 = 77,
    xmm = 78,
    xmm_m128 = 79,
    xmm_m32 = 80,
    xmm_m64 = 81,
    m64 = 82,
    r32 = 83,
    CRn = 84,
    DRn = 85,
    r64 = 86,
    mm_m64 = 87,
    r_m32 = 88,
    m128 = 89,
    mm = 90,
    MSR = 91,
    IA32_TIME_STAMP_COUNTER = 92,
    PMC = 93,
    ESP = 94,
    IA32_SYSENTER_CS = 95,
    XMM0 = 96,
    m32 = 97,
    FS = 98,
    IA32_BIOS_SIGN_ID = 99,
    GS = 100,
    m512 = 101,
    r = 102,
    r_m = 103,
 }

 fn get_instruction_index(byte1:u8, byte2:u8, byte3:u8, byte4:u8) -> Option<u32> {
    match byte1{
       0x10 => {return Some(16);}
       0x11 => {return Some(17);}
       0x12 => {return Some(18);}
       0x13 => {return Some(19);}
       0x14 => {return Some(20);}
       0x15 => {return Some(21);}
       0x16 => {return Some(22);}
       0x17 => {return Some(23);}
       0x18 => {return Some(24);}
       0x19 => {return Some(25);}
       0x20 => {return Some(32);}
       0x21 => {return Some(33);}
       0x22 => {return Some(34);}
       0x23 => {return Some(35);}
       0x24 => {return Some(36);}
       0x25 => {return Some(37);}
       0x27 => {return Some(38);}
       0x28 => {return Some(39);}
       0x29 => {return Some(40);}
       0x30 => {return Some(46);}
       0x31 => {return Some(47);}
       0x32 => {return Some(48);}
       0x33 => {return Some(49);}
       0x34 => {return Some(50);}
       0x35 => {return Some(51);}
       0x37 => {return Some(52);}
       0x38 => {return Some(53);}
       0x39 => {return Some(54);}
       0x40 => {return Some(60);}
       0x41 => {return Some(61);}
       0x42 => {return Some(62);}
       0x43 => {return Some(63);}
       0x44 => {return Some(64);}
       0x45 => {return Some(65);}
       0x46 => {return Some(66);}
       0x47 => {return Some(67);}
       0x48 => {return Some(68);}
       0x49 => {return Some(69);}
       0x50 => {return Some(76);}
       0x51 => {return Some(77);}
       0x52 => {return Some(78);}
       0x53 => {return Some(79);}
       0x54 => {return Some(80);}
       0x55 => {return Some(81);}
       0x56 => {return Some(82);}
       0x57 => {return Some(83);}
       0x58 => {return Some(84);}
       0x59 => {return Some(85);}
       0x60 => {return Some(92);}
       0x61 => {return Some(94);}
       0x62 => {return Some(96);}
       0x63 => {return Some(97);}
       0x68 => {return Some(98);}
       0x69 => {return Some(99);}
       0x70 => {return Some(108);}
       0x71 => {return Some(109);}
       0x72 => {return Some(110);}
       0x73 => {return Some(111);}
       0x74 => {return Some(112);}
       0x75 => {return Some(113);}
       0x76 => {return Some(114);}
       0x77 => {return Some(115);}
       0x78 => {return Some(116);}
       0x79 => {return Some(117);}
       0x80 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(124);}
          0x1 => {return Some(125);}
          0x2 => {return Some(126);}
          0x3 => {return Some(127);}
          0x4 => {return Some(128);}
          0x5 => {return Some(129);}
          0x6 => {return Some(130);}
          0x7 => {return Some(131);}
          _ => {return None}}}
       0x81 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(132);}
          0x1 => {return Some(133);}
          0x2 => {return Some(134);}
          0x3 => {return Some(135);}
          0x4 => {return Some(136);}
          0x5 => {return Some(137);}
          0x6 => {return Some(138);}
          0x7 => {return Some(139);}
          _ => {return None}}}
       0x82 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(140);}
          0x1 => {return Some(141);}
          0x2 => {return Some(142);}
          0x3 => {return Some(143);}
          0x4 => {return Some(144);}
          0x5 => {return Some(145);}
          0x6 => {return Some(146);}
          0x7 => {return Some(147);}
          _ => {return None}}}
       0x83 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(148);}
          0x1 => {return Some(149);}
          0x2 => {return Some(150);}
          0x3 => {return Some(151);}
          0x4 => {return Some(152);}
          0x5 => {return Some(153);}
          0x6 => {return Some(154);}
          0x7 => {return Some(155);}
          _ => {return None}}}
       0x84 => {return Some(156);}
       0x85 => {return Some(157);}
       0x86 => {return Some(158);}
       0x87 => {return Some(159);}
       0x88 => {return Some(160);}
       0x89 => {return Some(161);}
       0x90 => {return Some(168);}
       0x91 => {return Some(169);}
       0x92 => {return Some(170);}
       0x93 => {return Some(171);}
       0x94 => {return Some(172);}
       0x95 => {return Some(173);}
       0x96 => {return Some(174);}
       0x97 => {return Some(175);}
       0x98 => {return Some(177);}
       0x99 => {return Some(179);}
       0x00 => {return Some(0);}
       0x01 => {return Some(1);}
       0x02 => {return Some(2);}
       0x03 => {return Some(3);}
       0x04 => {return Some(4);}
       0x05 => {return Some(5);}
       0x06 => {return Some(6);}
       0x07 => {return Some(7);}
       0x08 => {return Some(8);}
       0x09 => {return Some(9);}
       0x0A => {return Some(10);}
       0x0B => {return Some(11);}
       0x0C => {return Some(12);}
       0x0D => {return Some(13);}
       0x0E => {return Some(14);}
       0x0F => {match byte2{
          0x10 => {return Some(522);}
          0x11 => {return Some(523);}
          0x12 => {return Some(524);}
          0x13 => {return Some(526);}
          0x14 => {return Some(527);}
          0x15 => {return Some(528);}
          0x16 => {return Some(529);}
          0x17 => {return Some(531);}
          0x18 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(532);}
             0x1 => {return Some(533);}
             0x2 => {return Some(534);}
             0x3 => {return Some(535);}
             0x4 => {return Some(536);}
             0x5 => {return Some(537);}
             0x6 => {return Some(538);}
             0x7 => {return Some(539);}
             _ => {return None}}}
          0x19 => {return Some(540);}
          0x20 => {return Some(554);}
          0x21 => {return Some(556);}
          0x22 => {return Some(558);}
          0x23 => {return Some(560);}
          0x28 => {return Some(562);}
          0x29 => {return Some(563);}
          0x30 => {return Some(570);}
          0x31 => {return Some(571);}
          0x32 => {return Some(572);}
          0x33 => {return Some(573);}
          0x34 => {return Some(574);}
          0x35 => {return Some(575);}
          0x37 => {return Some(576);}
          0x38 => {match byte3{
             0x00 => {return Some(577);}
             0x01 => {return Some(578);}
             0x02 => {return Some(579);}
             0x03 => {return Some(580);}
             0x04 => {return Some(581);}
             0x05 => {return Some(582);}
             0x06 => {return Some(583);}
             0x07 => {return Some(584);}
             0x08 => {return Some(585);}
             0x09 => {return Some(586);}
             0x0A => {return Some(587);}
             0x0B => {return Some(588);}
             0x1C => {return Some(589);}
             0x1D => {return Some(590);}
             0x1E => {return Some(591);}
             0xF0 => {return Some(592);}
             0xF1 => {return Some(593);}
             _ => {return None}}}
          0x40 => {return Some(595);}
          0x41 => {return Some(596);}
          0x42 => {return Some(597);}
          0x43 => {return Some(598);}
          0x44 => {return Some(599);}
          0x45 => {return Some(600);}
          0x46 => {return Some(601);}
          0x47 => {return Some(602);}
          0x48 => {return Some(603);}
          0x49 => {return Some(604);}
          0x50 => {return Some(611);}
          0x51 => {return Some(612);}
          0x52 => {return Some(613);}
          0x53 => {return Some(614);}
          0x54 => {return Some(615);}
          0x55 => {return Some(616);}
          0x56 => {return Some(617);}
          0x57 => {return Some(618);}
          0x58 => {return Some(619);}
          0x59 => {return Some(620);}
          0x60 => {return Some(627);}
          0x61 => {return Some(628);}
          0x62 => {return Some(629);}
          0x63 => {return Some(630);}
          0x64 => {return Some(631);}
          0x65 => {return Some(632);}
          0x66 => {return Some(633);}
          0x67 => {return Some(634);}
          0x68 => {return Some(635);}
          0x69 => {return Some(636);}
          0x70 => {return Some(641);}
          0x71 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x2 => {return Some(642);}
             0x4 => {return Some(643);}
             0x6 => {return Some(644);}
             _ => {return None}}}
          0x72 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x2 => {return Some(645);}
             0x4 => {return Some(646);}
             0x6 => {return Some(647);}
             _ => {return None}}}
          0x73 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x2 => {return Some(648);}
             0x6 => {return Some(649);}
             _ => {return None}}}
          0x74 => {return Some(650);}
          0x75 => {return Some(651);}
          0x76 => {return Some(652);}
          0x77 => {return Some(653);}
          0x78 => {return Some(654);}
          0x79 => {return Some(655);}
          0x80 => {return Some(658);}
          0x81 => {return Some(659);}
          0x82 => {return Some(660);}
          0x83 => {return Some(661);}
          0x84 => {return Some(662);}
          0x85 => {return Some(663);}
          0x86 => {return Some(664);}
          0x87 => {return Some(665);}
          0x88 => {return Some(666);}
          0x89 => {return Some(667);}
          0x90 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(674);}
             _ => {return None}}}
          0x91 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(675);}
             _ => {return None}}}
          0x92 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(676);}
             _ => {return None}}}
          0x93 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(677);}
             _ => {return None}}}
          0x94 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(678);}
             _ => {return None}}}
          0x95 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(679);}
             _ => {return None}}}
          0x96 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(680);}
             _ => {return None}}}
          0x97 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(681);}
             _ => {return None}}}
          0x98 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(682);}
             _ => {return None}}}
          0x99 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(683);}
             _ => {return None}}}
          0x00 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(493);}
             0x1 => {return Some(494);}
             0x2 => {return Some(495);}
             0x3 => {return Some(496);}
             0x4 => {return Some(497);}
             0x5 => {return Some(498);}
             _ => {return None}}}
          0x01 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {match byte3{
                0xC1 => {return Some(500);}
                0xC2 => {return Some(501);}
                0xC3 => {return Some(502);}
                0xC4 => {return Some(503);}
                _ => {return Some(499)}}}
             0x1 => {match byte3{
                0xC8 => {return Some(505);}
                0xC9 => {return Some(506);}
                _ => {return Some(504)}}}
             0x2 => {match byte3{
                0xD0 => {return Some(508);}
                0xD1 => {return Some(509);}
                _ => {return Some(507)}}}
             0x3 => {return Some(510);}
             0x4 => {return Some(511);}
             0x6 => {return Some(512);}
             0x7 => {match byte3{
                0xF9 => {return Some(514);}
                _ => {return Some(513)}}}
             _ => {return None}}}
          0x02 => {return Some(515);}
          0x03 => {return Some(516);}
          0x06 => {return Some(517);}
          0x08 => {return Some(518);}
          0x09 => {return Some(519);}
          0x0B => {return Some(520);}
          0x0D => {return Some(521);}
          0x1A => {return Some(541);}
          0x1B => {return Some(542);}
          0x1C => {return Some(543);}
          0x1D => {return Some(544);}
          0x1E => {return Some(545);}
          0x1F => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(546);}
             0x1 => {return Some(547);}
             0x2 => {return Some(548);}
             0x3 => {return Some(549);}
             0x4 => {return Some(550);}
             0x5 => {return Some(551);}
             0x6 => {return Some(552);}
             0x7 => {return Some(553);}
             _ => {return None}}}
          0x2A => {return Some(564);}
          0x2B => {return Some(565);}
          0x2C => {return Some(566);}
          0x2D => {return Some(567);}
          0x2E => {return Some(568);}
          0x2F => {return Some(569);}
          0x3A => {match byte3{
             0x0F => {return Some(594);}
             _ => {return None}}}
          0x4A => {return Some(605);}
          0x4B => {return Some(606);}
          0x4C => {return Some(607);}
          0x4D => {return Some(608);}
          0x4E => {return Some(609);}
          0x4F => {return Some(610);}
          0x5A => {return Some(621);}
          0x5B => {return Some(622);}
          0x5C => {return Some(623);}
          0x5D => {return Some(624);}
          0x5E => {return Some(625);}
          0x5F => {return Some(626);}
          0x6A => {return Some(637);}
          0x6B => {return Some(638);}
          0x6E => {return Some(639);}
          0x6F => {return Some(640);}
          0x7E => {return Some(656);}
          0x7F => {return Some(657);}
          0x8A => {return Some(668);}
          0x8B => {return Some(669);}
          0x8C => {return Some(670);}
          0x8D => {return Some(671);}
          0x8E => {return Some(672);}
          0x8F => {return Some(673);}
          0x9A => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(684);}
             _ => {return None}}}
          0x9B => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(685);}
             _ => {return None}}}
          0x9C => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(686);}
             _ => {return None}}}
          0x9D => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(687);}
             _ => {return None}}}
          0x9E => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(688);}
             _ => {return None}}}
          0x9F => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(689);}
             _ => {return None}}}
          0xA0 => {return Some(690);}
          0xA1 => {return Some(691);}
          0xA2 => {return Some(692);}
          0xA3 => {return Some(693);}
          0xA4 => {return Some(694);}
          0xA5 => {return Some(695);}
          0xA8 => {return Some(696);}
          0xA9 => {return Some(697);}
          0xAA => {return Some(698);}
          0xAB => {return Some(699);}
          0xAC => {return Some(700);}
          0xAD => {return Some(701);}
          0xAE => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x0 => {return Some(702);}
             0x1 => {return Some(703);}
             0x2 => {return Some(704);}
             0x3 => {return Some(705);}
             0x4 => {return Some(706);}
             0x5 => {return Some(707);}
             0x6 => {return Some(709);}
             0x7 => {return Some(710);}
             _ => {return None}}}
          0xAF => {return Some(712);}
          0xB0 => {return Some(713);}
          0xB1 => {return Some(714);}
          0xB2 => {return Some(715);}
          0xB3 => {return Some(716);}
          0xB4 => {return Some(717);}
          0xB5 => {return Some(718);}
          0xB6 => {return Some(719);}
          0xB7 => {return Some(720);}
          0xB9 => {return Some(721);}
          0xBA => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x4 => {return Some(722);}
             0x5 => {return Some(723);}
             0x6 => {return Some(724);}
             0x7 => {return Some(725);}
             _ => {return None}}}
          0xBB => {return Some(726);}
          0xBC => {return Some(727);}
          0xBD => {return Some(728);}
          0xBE => {return Some(729);}
          0xBF => {return Some(730);}
          0xC0 => {return Some(731);}
          0xC1 => {return Some(732);}
          0xC2 => {return Some(733);}
          0xC3 => {return Some(734);}
          0xC4 => {return Some(735);}
          0xC5 => {return Some(736);}
          0xC6 => {return Some(737);}
          0xC7 => {match r_bits!(byte3){ // RM 3bit value as opcode
             0x1 => {return Some(738);}
             0x6 => {return Some(739);}
             0x7 => {return Some(740);}
             _ => {return None}}}
          0xc8 => {return Some(741);}
          0xc9 => {return Some(742);}
          0xca => {return Some(743);}
          0xcb => {return Some(744);}
          0xcc => {return Some(745);}
          0xcd => {return Some(746);}
          0xce => {return Some(747);}
          0xcf => {return Some(748);}
          0xD1 => {return Some(749);}
          0xD2 => {return Some(750);}
          0xD3 => {return Some(751);}
          0xD4 => {return Some(752);}
          0xD5 => {return Some(753);}
          0xD7 => {return Some(754);}
          0xD8 => {return Some(755);}
          0xD9 => {return Some(756);}
          0xDA => {return Some(757);}
          0xDB => {return Some(758);}
          0xDC => {return Some(759);}
          0xDD => {return Some(760);}
          0xDE => {return Some(761);}
          0xDF => {return Some(762);}
          0xE0 => {return Some(763);}
          0xE1 => {return Some(764);}
          0xE2 => {return Some(765);}
          0xE3 => {return Some(766);}
          0xE4 => {return Some(767);}
          0xE5 => {return Some(768);}
          0xE7 => {return Some(769);}
          0xE8 => {return Some(770);}
          0xE9 => {return Some(771);}
          0xEA => {return Some(772);}
          0xEB => {return Some(773);}
          0xEC => {return Some(774);}
          0xED => {return Some(775);}
          0xEE => {return Some(776);}
          0xEF => {return Some(777);}
          0xF1 => {return Some(778);}
          0xF2 => {return Some(779);}
          0xF3 => {return Some(780);}
          0xF4 => {return Some(781);}
          0xF5 => {return Some(782);}
          0xF6 => {return Some(783);}
          0xF7 => {return Some(784);}
          0xF8 => {return Some(785);}
          0xF9 => {return Some(786);}
          0xFA => {return Some(787);}
          0xFB => {return Some(788);}
          0xFC => {return Some(789);}
          0xFD => {return Some(790);}
          0xFE => {return Some(791);}
          _ => {return None}}}
       0x1A => {return Some(26);}
       0x1B => {return Some(27);}
       0x1C => {return Some(28);}
       0x1D => {return Some(29);}
       0x1E => {return Some(30);}
       0x1F => {return Some(31);}
       0x2A => {return Some(41);}
       0x2B => {return Some(42);}
       0x2C => {return Some(43);}
       0x2D => {return Some(44);}
       0x2F => {return Some(45);}
       0x3A => {return Some(55);}
       0x3B => {return Some(56);}
       0x3C => {return Some(57);}
       0x3D => {return Some(58);}
       0x3F => {return Some(59);}
       0x4a => {return Some(70);}
       0x4b => {return Some(71);}
       0x4c => {return Some(72);}
       0x4d => {return Some(73);}
       0x4e => {return Some(74);}
       0x4f => {return Some(75);}
       0x5a => {return Some(86);}
       0x5b => {return Some(87);}
       0x5c => {return Some(88);}
       0x5d => {return Some(89);}
       0x5e => {return Some(90);}
       0x5f => {return Some(91);}
       0x6A => {return Some(100);}
       0x6B => {return Some(101);}
       0x6C => {return Some(102);}
       0x6D => {return Some(103);}
       0x6E => {return Some(105);}
       0x6F => {return Some(106);}
       0x7A => {return Some(118);}
       0x7B => {return Some(119);}
       0x7C => {return Some(120);}
       0x7D => {return Some(121);}
       0x7E => {return Some(122);}
       0x7F => {return Some(123);}
       0x8A => {return Some(162);}
       0x8B => {return Some(163);}
       0x8C => {return Some(164);}
       0x8D => {return Some(165);}
       0x8E => {return Some(166);}
       0x8F => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(167);}
          _ => {return None}}}
       0x9A => {return Some(181);}
       0x9B => {return Some(182);}
       0x9C => {return Some(183);}
       0x9D => {return Some(185);}
       0x9E => {return Some(187);}
       0x9F => {return Some(188);}
       0xA0 => {return Some(189);}
       0xA1 => {return Some(190);}
       0xA2 => {return Some(191);}
       0xA3 => {return Some(192);}
       0xA4 => {return Some(193);}
       0xA5 => {return Some(194);}
       0xA6 => {return Some(196);}
       0xA7 => {return Some(197);}
       0xA8 => {return Some(199);}
       0xA9 => {return Some(200);}
       0xAA => {return Some(201);}
       0xAB => {return Some(202);}
       0xAC => {return Some(204);}
       0xAD => {return Some(205);}
       0xAE => {return Some(207);}
       0xAF => {return Some(208);}
       0xb0 => {return Some(210);}
       0xb1 => {return Some(211);}
       0xb2 => {return Some(212);}
       0xb3 => {return Some(213);}
       0xb4 => {return Some(214);}
       0xb5 => {return Some(215);}
       0xb6 => {return Some(216);}
       0xb7 => {return Some(217);}
       0xb8 => {return Some(218);}
       0xb9 => {return Some(219);}
       0xba => {return Some(220);}
       0xbb => {return Some(221);}
       0xbc => {return Some(222);}
       0xbd => {return Some(223);}
       0xbe => {return Some(224);}
       0xbf => {return Some(225);}
       0xC0 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(226);}
          0x1 => {return Some(227);}
          0x2 => {return Some(228);}
          0x3 => {return Some(229);}
          0x4 => {return Some(230);}
          0x5 => {return Some(231);}
          0x6 => {return Some(232);}
          0x7 => {return Some(233);}
          _ => {return None}}}
       0xC1 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(234);}
          0x1 => {return Some(235);}
          0x2 => {return Some(236);}
          0x3 => {return Some(237);}
          0x4 => {return Some(238);}
          0x5 => {return Some(239);}
          0x6 => {return Some(240);}
          0x7 => {return Some(241);}
          _ => {return None}}}
       0xC2 => {return Some(242);}
       0xC3 => {return Some(243);}
       0xC4 => {return Some(244);}
       0xC5 => {return Some(245);}
       0xC6 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(246);}
          _ => {return None}}}
       0xC7 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(247);}
          _ => {return None}}}
       0xC8 => {return Some(248);}
       0xC9 => {return Some(249);}
       0xCA => {return Some(250);}
       0xCB => {return Some(251);}
       0xCC => {return Some(252);}
       0xCD => {return Some(253);}
       0xCE => {return Some(254);}
       0xCF => {return Some(255);}
       0xD0 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(257);}
          0x1 => {return Some(258);}
          0x2 => {return Some(259);}
          0x3 => {return Some(260);}
          0x4 => {return Some(261);}
          0x5 => {return Some(262);}
          0x6 => {return Some(263);}
          0x7 => {return Some(264);}
          _ => {return None}}}
       0xD1 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(265);}
          0x1 => {return Some(266);}
          0x2 => {return Some(267);}
          0x3 => {return Some(268);}
          0x4 => {return Some(269);}
          0x5 => {return Some(270);}
          0x6 => {return Some(271);}
          0x7 => {return Some(272);}
          _ => {return None}}}
       0xD2 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(273);}
          0x1 => {return Some(274);}
          0x2 => {return Some(275);}
          0x3 => {return Some(276);}
          0x4 => {return Some(277);}
          0x5 => {return Some(278);}
          0x6 => {return Some(279);}
          0x7 => {return Some(280);}
          _ => {return None}}}
       0xD3 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(281);}
          0x1 => {return Some(282);}
          0x2 => {return Some(283);}
          0x3 => {return Some(284);}
          0x4 => {return Some(285);}
          0x5 => {return Some(286);}
          0x6 => {return Some(287);}
          0x7 => {return Some(288);}
          _ => {return None}}}
       0xD4 => {match byte2{
          0x0A => {return Some(289);}
          _ => {return None}}}
       0xD5 => {match byte2{
          0x0A => {return Some(291);}
          _ => {return None}}}
       0xD6 => {return Some(293);}
       0xD7 => {return Some(295);}
       0xD8 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(296);}
          0x1 => {return Some(297);}
          0x2 => {match byte2{
             0xD1 => {return Some(299);}
             _ => {return Some(298)}}}
          0x3 => {match byte2{
             0xD9 => {return Some(301);}
             _ => {return Some(300)}}}
          0x4 => {return Some(302);}
          0x5 => {return Some(303);}
          0x6 => {return Some(304);}
          0x7 => {return Some(305);}
          _ => {return None}}}
       0xD9 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(306);}
          0x1 => {match byte2{
             0xC9 => {return Some(308);}
             _ => {return Some(307)}}}
          0x2 => {match byte2{
             0xD0 => {return Some(310);}
             _ => {return Some(309)}}}
          0x3 => {return Some(311);}
          0x4 => {match byte2{
             0xE0 => {return Some(314);}
             0xE1 => {return Some(315);}
             0xE4 => {return Some(316);}
             0xE5 => {return Some(317);}
             _ => {return Some(313)}}}
          0x5 => {match byte2{
             0xE8 => {return Some(319);}
             0xE9 => {return Some(320);}
             0xEA => {return Some(321);}
             0xEB => {return Some(322);}
             0xEC => {return Some(323);}
             0xED => {return Some(324);}
             0xEE => {return Some(325);}
             _ => {return Some(318)}}}
          0x6 => {match byte2{
             0xF0 => {return Some(327);}
             0xF1 => {return Some(328);}
             0xF2 => {return Some(329);}
             0xF3 => {return Some(330);}
             0xF4 => {return Some(331);}
             0xF5 => {return Some(332);}
             0xF6 => {return Some(333);}
             0xF7 => {return Some(334);}
             _ => {return Some(326)}}}
          0x7 => {match byte2{
             0xF8 => {return Some(336);}
             0xF9 => {return Some(337);}
             0xFA => {return Some(338);}
             0xFB => {return Some(339);}
             0xFC => {return Some(340);}
             0xFD => {return Some(341);}
             0xFE => {return Some(342);}
             0xFF => {return Some(343);}
             _ => {return Some(335)}}}
          _ => {return None}}}
       0xDA => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(344);}
          0x1 => {return Some(346);}
          0x2 => {return Some(348);}
          0x3 => {return Some(350);}
          0x4 => {return Some(352);}
          0x5 => {match byte2{
             0xE9 => {return Some(354);}
             _ => {return Some(353)}}}
          0x6 => {return Some(355);}
          0x7 => {return Some(356);}
          _ => {return None}}}
       0xDB => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(357);}
          0x1 => {return Some(359);}
          0x2 => {return Some(361);}
          0x3 => {return Some(363);}
          0x4 => {match byte2{
             0xE0 => {return Some(365);}
             0xE1 => {return Some(366);}
             0xE2 => {return Some(367);}
             0xE3 => {return Some(368);}
             0xE4 => {return Some(369);}
             _ => {return None}}}
          0x5 => {return Some(370);}
          0x6 => {return Some(372);}
          0x7 => {return Some(373);}
          _ => {return None}}}
       0xDC => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(374);}
          0x1 => {return Some(376);}
          0x2 => {return Some(378);}
          0x3 => {return Some(380);}
          0x4 => {return Some(382);}
          0x5 => {return Some(384);}
          0x6 => {return Some(386);}
          0x7 => {return Some(388);}
          _ => {return None}}}
       0xDD => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(390);}
          0x1 => {return Some(392);}
          0x2 => {return Some(394);}
          0x3 => {return Some(396);}
          0x4 => {match byte2{
             0xE1 => {return Some(400);}
             _ => {return Some(398)}}}
          0x5 => {match byte2{
             0xE9 => {return Some(402);}
             _ => {return Some(401)}}}
          0x6 => {return Some(403);}
          0x7 => {return Some(404);}
          _ => {return None}}}
       0xDE => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {match byte2{
             0xC1 => {return Some(407);}
             _ => {return Some(405)}}}
          0x1 => {match byte2{
             0xC9 => {return Some(410);}
             _ => {return Some(408)}}}
          0x2 => {return Some(411);}
          0x3 => {match byte2{
             0xD9 => {return Some(414);}
             _ => {return Some(413)}}}
          0x4 => {match byte2{
             0xE1 => {return Some(417);}
             _ => {return Some(415)}}}
          0x5 => {match byte2{
             0xE9 => {return Some(420);}
             _ => {return Some(418)}}}
          0x6 => {match byte2{
             0xF1 => {return Some(423);}
             _ => {return Some(421)}}}
          0x7 => {match byte2{
             0xF9 => {return Some(426);}
             _ => {return Some(424)}}}
          _ => {return None}}}
       0xDF => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(427);}
          0x1 => {return Some(429);}
          0x2 => {return Some(431);}
          0x3 => {return Some(433);}
          0x4 => {match byte2{
             0xE0 => {return Some(436);}
             _ => {return Some(435)}}}
          0x5 => {return Some(437);}
          0x6 => {return Some(439);}
          0x7 => {return Some(441);}
          _ => {return None}}}
       0xE0 => {return Some(442);}
       0xE1 => {return Some(443);}
       0xE2 => {return Some(444);}
       0xE3 => {return Some(445);}
       0xE4 => {return Some(446);}
       0xE5 => {return Some(447);}
       0xE6 => {return Some(448);}
       0xE7 => {return Some(449);}
       0xE8 => {return Some(450);}
       0xE9 => {return Some(451);}
       0xEA => {return Some(452);}
       0xEB => {return Some(453);}
       0xEC => {return Some(454);}
       0xED => {return Some(455);}
       0xEE => {return Some(456);}
       0xEF => {return Some(457);}
       0xF1 => {return Some(458);}
       0xF4 => {return Some(460);}
       0xF5 => {return Some(461);}
       0xF6 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(462);}
          0x1 => {return Some(463);}
          0x2 => {return Some(464);}
          0x3 => {return Some(465);}
          0x4 => {return Some(466);}
          0x5 => {return Some(467);}
          0x6 => {return Some(468);}
          0x7 => {return Some(469);}
          _ => {return None}}}
       0xF7 => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(470);}
          0x1 => {return Some(471);}
          0x2 => {return Some(472);}
          0x3 => {return Some(473);}
          0x4 => {return Some(474);}
          0x5 => {return Some(475);}
          0x6 => {return Some(476);}
          0x7 => {return Some(477);}
          _ => {return None}}}
       0xF8 => {return Some(478);}
       0xF9 => {return Some(479);}
       0xFA => {return Some(480);}
       0xFB => {return Some(481);}
       0xFC => {return Some(482);}
       0xFD => {return Some(483);}
       0xFE => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(484);}
          0x1 => {return Some(485);}
          _ => {return None}}}
       0xFF => {match r_bits!(byte2){ // RM 3bit value as opcode
          0x0 => {return Some(486);}
          0x1 => {return Some(487);}
          0x2 => {return Some(488);}
          0x3 => {return Some(489);}
          0x4 => {return Some(490);}
          0x5 => {return Some(491);}
          0x6 => {return Some(492);}
          _ => {return None}}}
       _ => {return None}
}}

lazy_static!{static ref INSTRUCTIONS:Vec<instruction> = vec![
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x00, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x01, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x02, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x03, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x04, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x05, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::ES,], rm_byte:rm_type::none, opc1:0x06, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::ES,], rm_byte:rm_type::none, opc1:0x07, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x08, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x09, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x0A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x0B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x0C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OR".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x0D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::CS,], rm_byte:rm_type::none, opc1:0x0E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x10, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x11, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x12, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x13, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x14, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADC".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x15, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::SS,], rm_byte:rm_type::none, opc1:0x16, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::SS,], rm_byte:rm_type::none, opc1:0x17, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x18, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x19, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x1A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x1B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x1C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SBB".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x1D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::DS,], rm_byte:rm_type::none, opc1:0x1E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::DS,], rm_byte:rm_type::none, opc1:0x1F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x20, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x21, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x22, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x23, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x24, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AND".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x25, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DAA".to_owned(), params:vec![operand::AL,], rm_byte:rm_type::none, opc1:0x27, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x28, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x29, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x2A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x2B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x2C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SUB".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x2D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DAS".to_owned(), params:vec![operand::AL,], rm_byte:rm_type::none, opc1:0x2F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x30, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x31, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x32, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x33, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x34, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XOR".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x35, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AAA".to_owned(), params:vec![operand::AL,operand::AH,], rm_byte:rm_type::none, opc1:0x37, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x38, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x39, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x3A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x3B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0x3C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMP".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0x3D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AAS".to_owned(), params:vec![operand::AL,operand::AH,], rm_byte:rm_type::none, opc1:0x3F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc1:0x40, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc1:0x41, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc1:0x42, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc1:0x43, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc1:0x44, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc1:0x45, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc1:0x46, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc1:0x47, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc1:0x48, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc1:0x49, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc1:0x4a, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc1:0x4b, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc1:0x4c, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc1:0x4d, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc1:0x4e, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"DEC".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc1:0x4f, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc1:0x50, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc1:0x51, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc1:0x52, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc1:0x53, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc1:0x54, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc1:0x55, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc1:0x56, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc1:0x57, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc1:0x58, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc1:0x59, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc1:0x5a, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc1:0x5b, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc1:0x5c, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc1:0x5d, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc1:0x5e, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc1:0x5f, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSHA".to_owned(), params:vec![operand::AX,operand::CX,operand::DX,], rm_byte:rm_type::none, opc1:0x60, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSHAD".to_owned(), params:vec![operand::EAX,operand::ECX,operand::EDX,], rm_byte:rm_type::none, opc1:0x60, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POPA".to_owned(), params:vec![operand::DI,operand::SI,operand::BP,], rm_byte:rm_type::none, opc1:0x61, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POPAD".to_owned(), params:vec![operand::EDI,operand::ESI,operand::EBP,], rm_byte:rm_type::none, opc1:0x61, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"BOUND".to_owned(), params:vec![operand::r16_32,operand::m16_32,operand::eFlags,], rm_byte:rm_type::available, opc1:0x62, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ARPL".to_owned(), params:vec![operand::r_m16,operand::r16,], rm_byte:rm_type::available, opc1:0x63, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::imm16_32,], rm_byte:rm_type::none, opc1:0x68, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IMUL".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::available, opc1:0x69, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSH".to_owned(), params:vec![operand::imm8,], rm_byte:rm_type::none, opc1:0x6A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IMUL".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,operand::imm8,], rm_byte:rm_type::available, opc1:0x6B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INS".to_owned(), params:vec![operand::m8,operand::DX,], rm_byte:rm_type::none, opc1:0x6C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INS".to_owned(), params:vec![operand::m16,operand::DX,], rm_byte:rm_type::none, opc1:0x6D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INS".to_owned(), params:vec![operand::m16_32,operand::DX,], rm_byte:rm_type::none, opc1:0x6D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m8,], rm_byte:rm_type::none, opc1:0x6E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m16,], rm_byte:rm_type::none, opc1:0x6F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m16_32,], rm_byte:rm_type::none, opc1:0x6F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JO".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x70, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNO".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x71, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JB".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x72, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNB".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x73, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JZ".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x74, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNZ".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x75, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JBE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x76, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNBE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x77, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JS".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x78, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNS".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x79, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JL".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNL".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JLE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JNLE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0x7F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x82, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x84, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x85, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x86, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x87, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc1:0x88, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::r_m16_32,operand::r16_32,], rm_byte:rm_type::available, opc1:0x89, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc1:0x8A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::r16_32,operand::r_m16_32,], rm_byte:rm_type::available, opc1:0x8B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::m16,operand::Sreg,], rm_byte:rm_type::available, opc1:0x8C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LEA".to_owned(), params:vec![operand::r16_32,operand::m,], rm_byte:rm_type::available, opc1:0x8D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::Sreg,operand::r_m16,], rm_byte:rm_type::available, opc1:0x8E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0x8F, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eAX,operand::eAX,], rm_byte:rm_type::none, opc1:0x90, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eCX,operand::eAX,], rm_byte:rm_type::none, opc1:0x91, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eDX,operand::eAX,], rm_byte:rm_type::none, opc1:0x92, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eBX,operand::eAX,], rm_byte:rm_type::none, opc1:0x93, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eSP,operand::eAX,], rm_byte:rm_type::none, opc1:0x94, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eBP,operand::eAX,], rm_byte:rm_type::none, opc1:0x95, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eSI,operand::eAX,], rm_byte:rm_type::none, opc1:0x96, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XCHG".to_owned(), params:vec![operand::eDI,operand::eAX,], rm_byte:rm_type::none, opc1:0x97, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"NOP".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0x90, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CBW".to_owned(), params:vec![operand::AX,operand::AL,], rm_byte:rm_type::none, opc1:0x98, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CWDE".to_owned(), params:vec![operand::EAX,operand::AX,], rm_byte:rm_type::none, opc1:0x98, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CWD".to_owned(), params:vec![operand::DX,operand::AX,], rm_byte:rm_type::none, opc1:0x99, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CDQ".to_owned(), params:vec![operand::EDX,operand::EAX,], rm_byte:rm_type::none, opc1:0x99, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CALLF".to_owned(), params:vec![operand::ptr16_16_32,], rm_byte:rm_type::none, opc1:0x9A, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"FWAIT".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0x9B, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSHF".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc1:0x9C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"PUSHFD".to_owned(), params:vec![operand::EFlags,], rm_byte:rm_type::none, opc1:0x9C, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POPF".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc1:0x9D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"POPFD".to_owned(), params:vec![operand::EFlags,], rm_byte:rm_type::none, opc1:0x9D, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SAHF".to_owned(), params:vec![operand::AH,], rm_byte:rm_type::none, opc1:0x9E, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LAHF".to_owned(), params:vec![operand::AH,], rm_byte:rm_type::none, opc1:0x9F, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::AL,operand::moffs8,], rm_byte:rm_type::none, opc1:0xA0, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eAX,operand::moffs16_32,], rm_byte:rm_type::none, opc1:0xA1, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::moffs8,operand::AL,], rm_byte:rm_type::none, opc1:0xA2, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::moffs16_32,operand::eAX,], rm_byte:rm_type::none, opc1:0xA3, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOVS".to_owned(), params:vec![operand::m8,operand::m8,], rm_byte:rm_type::none, opc1:0xA4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOVS".to_owned(), params:vec![operand::m16,operand::m16,], rm_byte:rm_type::none, opc1:0xA5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOVS".to_owned(), params:vec![operand::m16_32,operand::m16_32,], rm_byte:rm_type::none, opc1:0xA5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMPS".to_owned(), params:vec![operand::m8,operand::m8,], rm_byte:rm_type::none, opc1:0xA6, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMPS".to_owned(), params:vec![operand::m16,operand::m16,], rm_byte:rm_type::none, opc1:0xA7, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMPS".to_owned(), params:vec![operand::m16_32,operand::m16_32,], rm_byte:rm_type::none, opc1:0xA7, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"TEST".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0xA8, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"TEST".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xA9, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STOS".to_owned(), params:vec![operand::m8,operand::AL,], rm_byte:rm_type::none, opc1:0xAA, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STOS".to_owned(), params:vec![operand::m16,operand::AX,], rm_byte:rm_type::none, opc1:0xAB, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STOS".to_owned(), params:vec![operand::m16_32,operand::eAX,], rm_byte:rm_type::none, opc1:0xAB, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LODS".to_owned(), params:vec![operand::AL,operand::m8,], rm_byte:rm_type::none, opc1:0xAC, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LODS".to_owned(), params:vec![operand::AX,operand::m16,], rm_byte:rm_type::none, opc1:0xAD, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LODS".to_owned(), params:vec![operand::eAX,operand::m16_32,], rm_byte:rm_type::none, opc1:0xAD, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SCAS".to_owned(), params:vec![operand::m8,operand::AL,], rm_byte:rm_type::none, opc1:0xAE, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SCAS".to_owned(), params:vec![operand::m16,operand::AX,], rm_byte:rm_type::none, opc1:0xAF, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SCAS".to_owned(), params:vec![operand::m16_32,operand::eAX,], rm_byte:rm_type::none, opc1:0xAF, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0xb0, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::CL,operand::imm8,], rm_byte:rm_type::none, opc1:0xb1, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::DL,operand::imm8,], rm_byte:rm_type::none, opc1:0xb2, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::BL,operand::imm8,], rm_byte:rm_type::none, opc1:0xb3, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::AH,operand::imm8,], rm_byte:rm_type::none, opc1:0xb4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::CH,operand::imm8,], rm_byte:rm_type::none, opc1:0xb5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::DH,operand::imm8,], rm_byte:rm_type::none, opc1:0xb6, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::BH,operand::imm8,], rm_byte:rm_type::none, opc1:0xb7, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eAX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xb8, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eCX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xb9, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eDX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xba, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eBX,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xbb, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eSP,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xbc, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eBP,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xbd, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eSI,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xbe, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::eDI,operand::imm16_32,], rm_byte:rm_type::none, opc1:0xbf, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"RETN".to_owned(), params:vec![operand::imm16,], rm_byte:rm_type::none, opc1:0xC2, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"RETN".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xC3, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LES".to_owned(), params:vec![operand::ES,operand::r16_32,operand::m16_16_32,], rm_byte:rm_type::available, opc1:0xC4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LDS".to_owned(), params:vec![operand::DS,operand::r16_32,operand::m16_16_32,], rm_byte:rm_type::available, opc1:0xC5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"MOV".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xC6, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"MOV".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0xC7, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ENTER".to_owned(), params:vec![operand::eBP,operand::imm16,operand::imm8,], rm_byte:rm_type::none, opc1:0xC8, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LEAVE".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc1:0xC9, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"RETF".to_owned(), params:vec![operand::imm16,], rm_byte:rm_type::none, opc1:0xCA, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"RETF".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xCB, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INT".to_owned(), params:vec![operand::_3,operand::eFlags,], rm_byte:rm_type::none, opc1:0xCC, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INT".to_owned(), params:vec![operand::imm8,operand::eFlags,], rm_byte:rm_type::none, opc1:0xCD, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INTO".to_owned(), params:vec![operand::eFlags,], rm_byte:rm_type::none, opc1:0xCE, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IRET".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc1:0xCF, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IRETD".to_owned(), params:vec![operand::EFlags,], rm_byte:rm_type::none, opc1:0xCF, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32,operand::_1,], rm_byte:rm_type::reg_opcode, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32,operand::CL,], rm_byte:rm_type::reg_opcode, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"AAM".to_owned(), params:vec![operand::AL,operand::AH,], rm_byte:rm_type::none, opc1:0xD4, opc2:Some(0x0A), opc3:None, opc4_reg:None},
    instruction{name:"AMX".to_owned(), params:vec![operand::AL,operand::AH,operand::imm8,], rm_byte:rm_type::none, opc1:0xD4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"AAD".to_owned(), params:vec![operand::AL,operand::AH,], rm_byte:rm_type::none, opc1:0xD5, opc2:Some(0x0A), opc3:None, opc4_reg:None},
    instruction{name:"ADX".to_owned(), params:vec![operand::AL,operand::AH,operand::imm8,], rm_byte:rm_type::none, opc1:0xD5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"undefined".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xD6, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"SALC".to_owned(), params:vec![operand::AL,], rm_byte:rm_type::none, opc1:0xD6, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"XLAT".to_owned(), params:vec![operand::AL,operand::m8,], rm_byte:rm_type::none, opc1:0xD7, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"FADD".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FMUL".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD8, opc2:Some(0xD1), opc3:None, opc4_reg:None},
    instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD8, opc2:Some(0xD9), opc3:None, opc4_reg:None},
    instruction{name:"FSUB".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FSUBR".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FDIV".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FDIVR".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FXCH".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FXCH".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xC9), opc3:None, opc4_reg:None},
    instruction{name:"FST".to_owned(), params:vec![operand::m32real,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FNOP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xD0), opc3:None, opc4_reg:None},
    instruction{name:"FSTP".to_owned(), params:vec![operand::m32real,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FSTP1".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FLDENV".to_owned(), params:vec![operand::m14_28,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FCHS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE0), opc3:None, opc4_reg:None},
    instruction{name:"FABS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE1), opc3:None, opc4_reg:None},
    instruction{name:"FTST".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE4), opc3:None, opc4_reg:None},
    instruction{name:"FXAM".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE5), opc3:None, opc4_reg:None},
    instruction{name:"FLDCW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FLD1".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE8), opc3:None, opc4_reg:None},
    instruction{name:"FLDL2T".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xE9), opc3:None, opc4_reg:None},
    instruction{name:"FLDL2E".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xEA), opc3:None, opc4_reg:None},
    instruction{name:"FLDPI".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xEB), opc3:None, opc4_reg:None},
    instruction{name:"FLDLG2".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xEC), opc3:None, opc4_reg:None},
    instruction{name:"FLDLN2".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xED), opc3:None, opc4_reg:None},
    instruction{name:"FLDZ".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xEE), opc3:None, opc4_reg:None},
    instruction{name:"FNSTENV".to_owned(), params:vec![operand::m14_28,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"F2XM1".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF0), opc3:None, opc4_reg:None},
    instruction{name:"FYL2X".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF1), opc3:None, opc4_reg:None},
    instruction{name:"FPTAN".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF2), opc3:None, opc4_reg:None},
    instruction{name:"FPATAN".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF3), opc3:None, opc4_reg:None},
    instruction{name:"FXTRACT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF4), opc3:None, opc4_reg:None},
    instruction{name:"FPREM1".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF5), opc3:None, opc4_reg:None},
    instruction{name:"FDECSTP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF6), opc3:None, opc4_reg:None},
    instruction{name:"FINCSTP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF7), opc3:None, opc4_reg:None},
    instruction{name:"FNSTCW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FPREM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF8), opc3:None, opc4_reg:None},
    instruction{name:"FYL2XP1".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xF9), opc3:None, opc4_reg:None},
    instruction{name:"FSQRT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFA), opc3:None, opc4_reg:None},
    instruction{name:"FSINCOS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFB), opc3:None, opc4_reg:None},
    instruction{name:"FRNDINT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFC), opc3:None, opc4_reg:None},
    instruction{name:"FSCALE".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFD), opc3:None, opc4_reg:None},
    instruction{name:"FSIN".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFE), opc3:None, opc4_reg:None},
    instruction{name:"FCOS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xD9, opc2:Some(0xFF), opc3:None, opc4_reg:None},
    instruction{name:"FIADD".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FCMOVB".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FIMUL".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FCMOVE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FICOM".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCMOVBE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FICOMP".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FCMOVU".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FISUB".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FISUBR".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FUCOMPP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xDA, opc2:Some(0xE9), opc3:None, opc4_reg:None},
    instruction{name:"FIDIV".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FIDIVR".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FCMOVNB".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FISTTP".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FCMOVNE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FIST".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCMOVNBE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FISTP".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FCMOVNU".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FNENI nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xDB, opc2:Some(0xE0), opc3:None, opc4_reg:None},
    instruction{name:"FNDISI nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xDB, opc2:Some(0xE1), opc3:None, opc4_reg:None},
    instruction{name:"FNCLEX".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xDB, opc2:Some(0xE2), opc3:None, opc4_reg:None},
    instruction{name:"FNINIT".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xDB, opc2:Some(0xE3), opc3:None, opc4_reg:None},
    instruction{name:"FNSETPM nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc1:0xDB, opc2:Some(0xE4), opc3:None, opc4_reg:None},
    instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::m80real,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FUCOMI".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FCOMI".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FSTP".to_owned(), params:vec![operand::m80real,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FADD".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FADD".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FMUL".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FMUL".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCOM2".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FCOMP3".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FSUB".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FSUBR".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FSUBR".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FSUB".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FDIV".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FDIVR".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FDIVR".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FDIV".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FFREE".to_owned(), params:vec![operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FISTTP".to_owned(), params:vec![operand::m64int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FXCH4".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FST".to_owned(), params:vec![operand::m64real,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FST".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FSTP".to_owned(), params:vec![operand::m64real,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FSTP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FRSTOR".to_owned(), params:vec![operand::ST,operand::ST1,operand::ST2,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FUCOM".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FUCOM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xDD, opc2:Some(0xE1), opc3:None, opc4_reg:None},
    instruction{name:"FUCOMP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FUCOMP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xDD, opc2:Some(0xE9), opc3:None, opc4_reg:None},
    instruction{name:"FNSAVE".to_owned(), params:vec![operand::m94_108,operand::ST,operand::ST1,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FNSTSW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FIADD".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FADDP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FADDP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xC1), opc3:None, opc4_reg:None},
    instruction{name:"FIMUL".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FMULP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FMULP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xC9), opc3:None, opc4_reg:None},
    instruction{name:"FICOM".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FCOMP5".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FICOMP".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FCOMPP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xD9), opc3:None, opc4_reg:None},
    instruction{name:"FISUB".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FSUBRP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FSUBRP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xE1), opc3:None, opc4_reg:None},
    instruction{name:"FISUBR".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FSUBP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FSUBP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xE9), opc3:None, opc4_reg:None},
    instruction{name:"FIDIV".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FDIVRP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FDIVRP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xF1), opc3:None, opc4_reg:None},
    instruction{name:"FIDIVR".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FDIVP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"FDIVP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc1:0xDE, opc2:Some(0xF9), opc3:None, opc4_reg:None},
    instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FFREEP".to_owned(), params:vec![operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"FISTTP".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FXCH7".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"FIST".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FSTP8".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"FISTP".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FSTP9".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"FBLD".to_owned(), params:vec![operand::ST,operand::m80dec,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"FNSTSW".to_owned(), params:vec![operand::AX,], rm_byte:rm_type::full_opcode, opc1:0xDF, opc2:Some(0xE0), opc3:None, opc4_reg:None},
    instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m64int,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FUCOMIP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"FBSTP".to_owned(), params:vec![operand::m80dec,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FCOMIP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"FISTP".to_owned(), params:vec![operand::m64int,operand::ST,], rm_byte:rm_type::reg_opcode, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"LOOPNZ".to_owned(), params:vec![operand::eCX,operand::rel8,], rm_byte:rm_type::none, opc1:0xE0, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LOOPZ".to_owned(), params:vec![operand::eCX,operand::rel8,], rm_byte:rm_type::none, opc1:0xE1, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"LOOP".to_owned(), params:vec![operand::eCX,operand::rel8,], rm_byte:rm_type::none, opc1:0xE2, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JCXZ".to_owned(), params:vec![operand::rel8,operand::CX,], rm_byte:rm_type::none, opc1:0xE3, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IN".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc1:0xE4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IN".to_owned(), params:vec![operand::eAX,operand::imm8,], rm_byte:rm_type::none, opc1:0xE5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUT".to_owned(), params:vec![operand::imm8,operand::AL,], rm_byte:rm_type::none, opc1:0xE6, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUT".to_owned(), params:vec![operand::imm8,operand::eAX,], rm_byte:rm_type::none, opc1:0xE7, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CALL".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc1:0xE8, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JMP".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc1:0xE9, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JMPF".to_owned(), params:vec![operand::ptr16_16_32,], rm_byte:rm_type::none, opc1:0xEA, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"JMP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc1:0xEB, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IN".to_owned(), params:vec![operand::AL,operand::DX,], rm_byte:rm_type::none, opc1:0xEC, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"IN".to_owned(), params:vec![operand::eAX,operand::DX,], rm_byte:rm_type::none, opc1:0xED, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUT".to_owned(), params:vec![operand::DX,operand::AL,], rm_byte:rm_type::none, opc1:0xEE, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"OUT".to_owned(), params:vec![operand::DX,operand::eAX,], rm_byte:rm_type::none, opc1:0xEF, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"undefined".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xF1, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INT1".to_owned(), params:vec![operand::eFlags,], rm_byte:rm_type::none, opc1:0xF1, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"HLT".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xF4, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CMC".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xF5, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"NOT".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"NEG".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"MUL".to_owned(), params:vec![operand::AX,operand::AL,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"IMUL".to_owned(), params:vec![operand::AX,operand::AL,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"DIV".to_owned(), params:vec![operand::AL,operand::AH,operand::AX,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"IDIV".to_owned(), params:vec![operand::AL,operand::AH,operand::AX,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"NOT".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"NEG".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"MUL".to_owned(), params:vec![operand::eDX,operand::eAX,operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"IMUL".to_owned(), params:vec![operand::eDX,operand::eAX,operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"DIV".to_owned(), params:vec![operand::eDX,operand::eAX,operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x6)},
    instruction{name:"IDIV".to_owned(), params:vec![operand::eDX,operand::eAX,operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x7)},
    instruction{name:"CLC".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xF8, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STC".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xF9, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CLI".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xFA, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STI".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xFB, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"CLD".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xFC, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"STD".to_owned(), params:vec![], rm_byte:rm_type::none, opc1:0xFD, opc2:None, opc3:None, opc4_reg:None},
    instruction{name:"INC".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xFE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"DEC".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc1:0xFE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"INC".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
    instruction{name:"DEC".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
    instruction{name:"CALL".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
    instruction{name:"CALLF".to_owned(), params:vec![operand::m16_16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
    instruction{name:"JMP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x4)},
    instruction{name:"JMPF".to_owned(), params:vec![operand::m16_16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
    instruction{name:"PUSH".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
];}