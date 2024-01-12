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
pub fn get_instruction(byte1:u8, byte2:u8, byte3:u8) -> Option<&'static instruction>{
    let opcode_index = get_instruction_index(byte1, byte2, byte3);
    if opcode_index.is_none(){return None};
    return Some(&INSTRUCTIONS[opcode_index.unwrap() as usize]);
}

pub struct instruction{
   pub name:String,
   pub params:Vec<operand>,
    //func:u64, // placeholder function reference
   pub rm_byte:rm_type,

    // debug data
   pub opc_length:u8,
   pub opc1:u8,
   pub opc2:Option<u8>, 
   pub opc3:Option<u8>,
   pub opc4_reg:Option<u8>, // the 3 or 5 bits used in the r/m byte // 0-4, 0-7 // i dont think its just 5 bits, it could be the whole byte?
    // we also need the prefix byte, but i dont think we're going to support that for this implementation
}

enum rm_type{
    none = 0,
    available = 1,
    reg_opcode = 2,
    full_opcode = 3
}

pub enum prefixes{ // u16
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

pub enum operand{
   r_m8 = 0,
   r8 = 1,
   r_m16_32_64 = 2,
   r16_32_64 = 3,
   AL = 4,
   imm8 = 5,
   rAX = 6,
   imm16_32 = 7,
   eAX = 8,
   eCX = 9,
   eDX = 10,
   eBX = 11,
   eSP = 12,
   eBP = 13,
   eSI = 14,
   eDI = 15,
   r32_64 = 16,
   r_m32 = 17,
   m8 = 18,
   DX = 19,
   m16 = 20,
   m16_32 = 21,
   rel8 = 22,
   Sreg = 23,
   m = 24,
   r_m16 = 25,
   r_m16_32 = 26,
   r_m64_16 = 27,
   AX = 28,
   Flags = 29,
   AH = 30,
   moffs8 = 31,
   moffs16_32_64 = 32,
   m16_32_64 = 33,
   CL = 34,
   DL = 35,
   BL = 36,
   CH = 37,
   DH = 38,
   BH = 39,
   imm16_32_64 = 40,
   imm16 = 41,
   rBP = 42,
   _3 = 43,
   eFlags = 44,
   _1 = 45,
   ST = 46,
   m32real = 47,
   STi_m32real = 48,
   ST1 = 49,
   STi = 50,
   m14_28 = 51,
   m32int = 52,
   m80real = 53,
   m64real = 54,
   m64int = 55,
   ST2 = 56,
   m94_108 = 57,
   m16int = 58,
   m80dec = 59,
   rCX = 60,
   ECX = 61,
   rel16_32 = 62,
   rDX = 63,
   r_m64 = 64,
   m16_16_32_64 = 65,
   LDTR = 66,
   TR = 67,
   GDTR = 68,
   IDTR = 69,
   EDX = 70,
   EAX = 71,
   XCR = 72,
   MSW = 73,
   GS = 74,
   IA32_KERNEL_GSBASE = 75,
   RCX = 76,
   R11 = 77,
   SS = 78,
   CR0 = 79,
   EFlags = 80,
   xmm = 81,
   xmm_m128 = 82,
   xmm_m32 = 83,
   xmm_m64 = 84,
   m64 = 85,
   r64 = 86,
   CRn = 87,
   DRn = 88,
   mm_m64 = 89,
   r_m32_64 = 90,
   m128 = 91,
   mm = 92,
   MSR = 93,
   IA32_TIME_STAMP_COUNTER = 94,
   PMC = 95,
   RSP = 96,
   IA32_SYSENTER_CS = 97,
   m32 = 98,
   XMM0 = 99,
   FS = 100,
   IA32_BIOS_SIGN_ID = 101,
   m512 = 102,
   r = 103,
   r_m = 104,
   m32_64 = 105,
}

fn get_instruction_index(byte1:u8, byte2:u8, byte3:u8) -> Option<u32>{
   match byte1{
      0x10 => {return Some(13);}
      0x11 => {return Some(14);}
      0x12 => {return Some(15);}
      0x13 => {return Some(16);}
      0x14 => {return Some(17);}
      0x15 => {return Some(18);}
      0x18 => {return Some(19);}
      0x19 => {return Some(20);}
      0x20 => {return Some(25);}
      0x21 => {return Some(26);}
      0x22 => {return Some(27);}
      0x23 => {return Some(28);}
      0x24 => {return Some(29);}
      0x25 => {return Some(30);}
      0x28 => {return Some(31);}
      0x29 => {return Some(32);}
      0x30 => {return Some(37);}
      0x31 => {return Some(38);}
      0x32 => {return Some(39);}
      0x33 => {return Some(40);}
      0x34 => {return Some(41);}
      0x35 => {return Some(42);}
      0x38 => {return Some(43);}
      0x39 => {return Some(44);}
      0x50 => {return Some(49);}
      0x51 => {return Some(50);}
      0x52 => {return Some(51);}
      0x53 => {return Some(52);}
      0x54 => {return Some(53);}
      0x55 => {return Some(54);}
      0x56 => {return Some(55);}
      0x57 => {return Some(56);}
      0x58 => {return Some(57);}
      0x59 => {return Some(58);}
      0x63 => {return Some(65);}
      0x68 => {return Some(66);}
      0x69 => {return Some(67);}
      0x70 => {return Some(76);}
      0x71 => {return Some(77);}
      0x72 => {return Some(78);}
      0x73 => {return Some(79);}
      0x74 => {return Some(80);}
      0x75 => {return Some(81);}
      0x76 => {return Some(82);}
      0x77 => {return Some(83);}
      0x78 => {return Some(84);}
      0x79 => {return Some(85);}
      0x80 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(92);}
         0x1 => {return Some(93);}
         0x2 => {return Some(94);}
         0x3 => {return Some(95);}
         0x4 => {return Some(96);}
         0x5 => {return Some(97);}
         0x6 => {return Some(98);}
         0x7 => {return Some(99);}
         _ => {return None}}}
      0x81 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(100);}
         0x1 => {return Some(101);}
         0x2 => {return Some(102);}
         0x3 => {return Some(103);}
         0x4 => {return Some(104);}
         0x5 => {return Some(105);}
         0x6 => {return Some(106);}
         0x7 => {return Some(107);}
         _ => {return None}}}
      0x83 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(108);}
         0x1 => {return Some(109);}
         0x2 => {return Some(110);}
         0x3 => {return Some(111);}
         0x4 => {return Some(112);}
         0x5 => {return Some(113);}
         0x6 => {return Some(114);}
         0x7 => {return Some(115);}
         _ => {return None}}}
      0x84 => {return Some(116);}
      0x85 => {return Some(117);}
      0x86 => {return Some(118);}
      0x87 => {return Some(119);}
      0x88 => {return Some(120);}
      0x89 => {return Some(121);}
      0x90 => {return Some(129);}
      0x91 => {return Some(130);}
      0x92 => {return Some(131);}
      0x93 => {return Some(132);}
      0x94 => {return Some(133);}
      0x95 => {return Some(134);}
      0x96 => {return Some(135);}
      0x97 => {return Some(136);}
      0x98 => {return Some(138);}
      0x99 => {return Some(139);}
      0x00 => {return Some(0);}
      0x01 => {return Some(1);}
      0x02 => {return Some(2);}
      0x03 => {return Some(3);}
      0x04 => {return Some(4);}
      0x05 => {return Some(5);}
      0x08 => {return Some(6);}
      0x09 => {return Some(7);}
      0x0A => {return Some(8);}
      0x0B => {return Some(9);}
      0x0C => {return Some(10);}
      0x0D => {return Some(11);}
      0x0F => {match byte2{
         0x10 => {return Some(464);}
         0x11 => {return Some(465);}
         0x12 => {return Some(466);}
         0x13 => {return Some(468);}
         0x14 => {return Some(469);}
         0x15 => {return Some(470);}
         0x16 => {return Some(471);}
         0x17 => {return Some(473);}
         0x18 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(474);}
            0x1 => {return Some(475);}
            0x2 => {return Some(476);}
            0x3 => {return Some(477);}
            0x4 => {return Some(478);}
            0x5 => {return Some(479);}
            0x6 => {return Some(480);}
            0x7 => {return Some(481);}
            _ => {return None}}}
         0x19 => {return Some(482);}
         0x20 => {return Some(496);}
         0x21 => {return Some(498);}
         0x22 => {return Some(500);}
         0x23 => {return Some(502);}
         0x28 => {return Some(504);}
         0x29 => {return Some(505);}
         0x30 => {return Some(512);}
         0x31 => {return Some(513);}
         0x32 => {return Some(514);}
         0x33 => {return Some(515);}
         0x37 => {return Some(516);}
         0x38 => {match byte3{
            0xF0 => {return Some(517);}
            0xF1 => {return Some(518);}
            _ => {return None}}}
         0x40 => {return Some(520);}
         0x41 => {return Some(521);}
         0x42 => {return Some(522);}
         0x43 => {return Some(523);}
         0x44 => {return Some(524);}
         0x45 => {return Some(525);}
         0x46 => {return Some(526);}
         0x47 => {return Some(527);}
         0x48 => {return Some(528);}
         0x49 => {return Some(529);}
         0x50 => {return Some(536);}
         0x51 => {return Some(537);}
         0x52 => {return Some(538);}
         0x53 => {return Some(539);}
         0x54 => {return Some(540);}
         0x55 => {return Some(541);}
         0x56 => {return Some(542);}
         0x57 => {return Some(543);}
         0x58 => {return Some(544);}
         0x59 => {return Some(545);}
         0x60 => {return Some(552);}
         0x61 => {return Some(553);}
         0x62 => {return Some(554);}
         0x63 => {return Some(555);}
         0x64 => {return Some(556);}
         0x65 => {return Some(557);}
         0x66 => {return Some(558);}
         0x67 => {return Some(559);}
         0x68 => {return Some(560);}
         0x69 => {return Some(561);}
         0x70 => {return Some(566);}
         0x71 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x2 => {return Some(567);}
            0x4 => {return Some(568);}
            0x6 => {return Some(569);}
            _ => {return None}}}
         0x72 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x2 => {return Some(570);}
            0x4 => {return Some(571);}
            0x6 => {return Some(572);}
            _ => {return None}}}
         0x73 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x2 => {return Some(573);}
            0x6 => {return Some(574);}
            _ => {return None}}}
         0x74 => {return Some(575);}
         0x75 => {return Some(576);}
         0x76 => {return Some(577);}
         0x77 => {return Some(578);}
         0x78 => {return Some(579);}
         0x79 => {return Some(580);}
         0x80 => {return Some(583);}
         0x81 => {return Some(584);}
         0x82 => {return Some(585);}
         0x83 => {return Some(586);}
         0x84 => {return Some(587);}
         0x85 => {return Some(588);}
         0x86 => {return Some(589);}
         0x87 => {return Some(590);}
         0x88 => {return Some(591);}
         0x89 => {return Some(592);}
         0x90 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(599);}
            _ => {return None}}}
         0x91 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(600);}
            _ => {return None}}}
         0x92 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(601);}
            _ => {return None}}}
         0x93 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(602);}
            _ => {return None}}}
         0x94 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(603);}
            _ => {return None}}}
         0x95 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(604);}
            _ => {return None}}}
         0x96 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(605);}
            _ => {return None}}}
         0x97 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(606);}
            _ => {return None}}}
         0x98 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(607);}
            _ => {return None}}}
         0x99 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(608);}
            _ => {return None}}}
         0x00 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(435);}
            0x1 => {return Some(436);}
            0x2 => {return Some(437);}
            0x3 => {return Some(438);}
            0x4 => {return Some(439);}
            0x5 => {return Some(440);}
            _ => {return None}}}
         0x01 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {match byte3{
               0xC1 => {return Some(442);}
               0xC2 => {return Some(443);}
               0xC3 => {return Some(444);}
               0xC4 => {return Some(445);}
               _ => {return Some(441)}}}
            0x1 => {match byte3{
               0xC8 => {return Some(447);}
               0xC9 => {return Some(448);}
               _ => {return Some(446)}}}
            0x2 => {match byte3{
               0xD0 => {return Some(450);}
               0xD1 => {return Some(451);}
               _ => {return Some(449)}}}
            0x3 => {return Some(452);}
            0x4 => {return Some(453);}
            0x6 => {return Some(454);}
            0x7 => {match byte3{
               0xF8 => {return Some(456);}
               _ => {return Some(455)}}}
            _ => {return None}}}
         0x02 => {return Some(457);}
         0x03 => {return Some(458);}
         0x06 => {return Some(459);}
         0x08 => {return Some(460);}
         0x09 => {return Some(461);}
         0x0B => {return Some(462);}
         0x0D => {return Some(463);}
         0x1A => {return Some(483);}
         0x1B => {return Some(484);}
         0x1C => {return Some(485);}
         0x1D => {return Some(486);}
         0x1E => {return Some(487);}
         0x1F => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(488);}
            0x1 => {return Some(489);}
            0x2 => {return Some(490);}
            0x3 => {return Some(491);}
            0x4 => {return Some(492);}
            0x5 => {return Some(493);}
            0x6 => {return Some(494);}
            0x7 => {return Some(495);}
            _ => {return None}}}
         0x2A => {return Some(506);}
         0x2B => {return Some(507);}
         0x2C => {return Some(508);}
         0x2D => {return Some(509);}
         0x2E => {return Some(510);}
         0x2F => {return Some(511);}
         0x3A => {match byte3{
            0x0F => {return Some(519);}
            _ => {return None}}}
         0x4A => {return Some(530);}
         0x4B => {return Some(531);}
         0x4C => {return Some(532);}
         0x4D => {return Some(533);}
         0x4E => {return Some(534);}
         0x4F => {return Some(535);}
         0x5A => {return Some(546);}
         0x5B => {return Some(547);}
         0x5C => {return Some(548);}
         0x5D => {return Some(549);}
         0x5E => {return Some(550);}
         0x5F => {return Some(551);}
         0x6A => {return Some(562);}
         0x6B => {return Some(563);}
         0x6E => {return Some(564);}
         0x6F => {return Some(565);}
         0x7E => {return Some(581);}
         0x7F => {return Some(582);}
         0x8A => {return Some(593);}
         0x8B => {return Some(594);}
         0x8C => {return Some(595);}
         0x8D => {return Some(596);}
         0x8E => {return Some(597);}
         0x8F => {return Some(598);}
         0x9A => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(609);}
            _ => {return None}}}
         0x9B => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(610);}
            _ => {return None}}}
         0x9C => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(611);}
            _ => {return None}}}
         0x9D => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(612);}
            _ => {return None}}}
         0x9E => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(613);}
            _ => {return None}}}
         0x9F => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x0 => {return Some(614);}
            _ => {return None}}}
         0xA0 => {return Some(615);}
         0xA1 => {return Some(616);}
         0xA3 => {return Some(617);}
         0xA4 => {return Some(618);}
         0xA5 => {return Some(619);}
         0xA8 => {return Some(620);}
         0xA9 => {return Some(621);}
         0xAA => {return Some(622);}
         0xAB => {return Some(623);}
         0xAC => {return Some(624);}
         0xAD => {return Some(625);}
         0xAE => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x2 => {return Some(626);}
            0x3 => {return Some(627);}
            0x5 => {return Some(628);}
            0x6 => {return Some(629);}
            0x7 => {return Some(630);}
            _ => {return None}}}
         0xAF => {return Some(632);}
         0xB0 => {return Some(633);}
         0xB1 => {return Some(634);}
         0xB2 => {return Some(635);}
         0xB3 => {return Some(636);}
         0xB4 => {return Some(637);}
         0xB5 => {return Some(638);}
         0xB6 => {return Some(639);}
         0xB7 => {return Some(640);}
         0xB9 => {return Some(641);}
         0xBA => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x4 => {return Some(642);}
            0x5 => {return Some(643);}
            0x6 => {return Some(644);}
            0x7 => {return Some(645);}
            _ => {return None}}}
         0xBB => {return Some(646);}
         0xBC => {return Some(647);}
         0xBD => {return Some(648);}
         0xBE => {return Some(649);}
         0xBF => {return Some(650);}
         0xC0 => {return Some(651);}
         0xC1 => {return Some(652);}
         0xC2 => {return Some(653);}
         0xC3 => {return Some(654);}
         0xC4 => {return Some(655);}
         0xC5 => {return Some(656);}
         0xC6 => {return Some(657);}
         0xC7 => {match r_bits!(byte3){ // RM 3bit value as opcode
            0x6 => {return Some(658);}
            0x7 => {return Some(659);}
            _ => {return None}}}
         0xc8 => {return Some(660);}
         0xc9 => {return Some(661);}
         0xca => {return Some(662);}
         0xcb => {return Some(663);}
         0xcc => {return Some(664);}
         0xcd => {return Some(665);}
         0xce => {return Some(666);}
         0xcf => {return Some(667);}
         0xD1 => {return Some(668);}
         0xD2 => {return Some(669);}
         0xD3 => {return Some(670);}
         0xD4 => {return Some(671);}
         0xD5 => {return Some(672);}
         0xD7 => {return Some(673);}
         0xD8 => {return Some(674);}
         0xD9 => {return Some(675);}
         0xDA => {return Some(676);}
         0xDB => {return Some(677);}
         0xDC => {return Some(678);}
         0xDD => {return Some(679);}
         0xDE => {return Some(680);}
         0xDF => {return Some(681);}
         0xE0 => {return Some(682);}
         0xE1 => {return Some(683);}
         0xE2 => {return Some(684);}
         0xE3 => {return Some(685);}
         0xE4 => {return Some(686);}
         0xE5 => {return Some(687);}
         0xE7 => {return Some(688);}
         0xE8 => {return Some(689);}
         0xE9 => {return Some(690);}
         0xEA => {return Some(691);}
         0xEB => {return Some(692);}
         0xEC => {return Some(693);}
         0xED => {return Some(694);}
         0xEE => {return Some(695);}
         0xEF => {return Some(696);}
         0xF1 => {return Some(697);}
         0xF2 => {return Some(698);}
         0xF3 => {return Some(699);}
         0xF4 => {return Some(700);}
         0xF5 => {return Some(701);}
         0xF6 => {return Some(702);}
         0xF7 => {return Some(703);}
         0xF8 => {return Some(704);}
         0xF9 => {return Some(705);}
         0xFA => {return Some(706);}
         0xFB => {return Some(707);}
         0xFC => {return Some(708);}
         0xFD => {return Some(709);}
         0xFE => {return Some(710);}
         _ => {return None}}}
      0x1A => {return Some(21);}
      0x1B => {return Some(22);}
      0x1C => {return Some(23);}
      0x1D => {return Some(24);}
      0x2A => {return Some(33);}
      0x2B => {return Some(34);}
      0x2C => {return Some(35);}
      0x2D => {return Some(36);}
      0x3A => {return Some(45);}
      0x3B => {return Some(46);}
      0x3C => {return Some(47);}
      0x3D => {return Some(48);}
      0x5a => {return Some(59);}
      0x5b => {return Some(60);}
      0x5c => {return Some(61);}
      0x5d => {return Some(62);}
      0x5e => {return Some(63);}
      0x5f => {return Some(64);}
      0x6A => {return Some(68);}
      0x6B => {return Some(69);}
      0x6C => {return Some(70);}
      0x6D => {return Some(71);}
      0x6E => {return Some(73);}
      0x6F => {return Some(74);}
      0x7A => {return Some(86);}
      0x7B => {return Some(87);}
      0x7C => {return Some(88);}
      0x7D => {return Some(89);}
      0x7E => {return Some(90);}
      0x7F => {return Some(91);}
      0x8A => {return Some(122);}
      0x8B => {return Some(123);}
      0x8C => {return Some(124);}
      0x8D => {return Some(125);}
      0x8E => {return Some(126);}
      0x8F => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(127);}
         _ => {return None}}}
      0x9B => {return Some(140);}
      0x9C => {return Some(141);}
      0x9D => {return Some(142);}
      0x9E => {return Some(143);}
      0x9F => {return Some(144);}
      0xA0 => {return Some(145);}
      0xA1 => {return Some(146);}
      0xA2 => {return Some(147);}
      0xA3 => {return Some(148);}
      0xA4 => {return Some(149);}
      0xA5 => {return Some(150);}
      0xA6 => {return Some(151);}
      0xA7 => {return Some(152);}
      0xA8 => {return Some(153);}
      0xA9 => {return Some(154);}
      0xAA => {return Some(155);}
      0xAB => {return Some(156);}
      0xAC => {return Some(157);}
      0xAD => {return Some(158);}
      0xAE => {return Some(159);}
      0xAF => {return Some(160);}
      0xb0 => {return Some(161);}
      0xb1 => {return Some(162);}
      0xb2 => {return Some(163);}
      0xb3 => {return Some(164);}
      0xb4 => {return Some(165);}
      0xb5 => {return Some(166);}
      0xb6 => {return Some(167);}
      0xb7 => {return Some(168);}
      0xb8 => {return Some(169);}
      0xb9 => {return Some(170);}
      0xba => {return Some(171);}
      0xbb => {return Some(172);}
      0xbc => {return Some(173);}
      0xbd => {return Some(174);}
      0xbe => {return Some(175);}
      0xbf => {return Some(176);}
      0xC0 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(177);}
         0x1 => {return Some(178);}
         0x2 => {return Some(179);}
         0x3 => {return Some(180);}
         0x4 => {return Some(181);}
         0x5 => {return Some(182);}
         0x6 => {return Some(183);}
         0x7 => {return Some(184);}
         _ => {return None}}}
      0xC1 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(185);}
         0x1 => {return Some(186);}
         0x2 => {return Some(187);}
         0x3 => {return Some(188);}
         0x4 => {return Some(189);}
         0x5 => {return Some(190);}
         0x6 => {return Some(191);}
         0x7 => {return Some(192);}
         _ => {return None}}}
      0xC2 => {return Some(193);}
      0xC3 => {return Some(194);}
      0xC6 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(195);}
         _ => {return None}}}
      0xC7 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(196);}
         _ => {return None}}}
      0xC8 => {return Some(197);}
      0xC9 => {return Some(198);}
      0xCA => {return Some(199);}
      0xCB => {return Some(200);}
      0xCC => {return Some(201);}
      0xCD => {return Some(202);}
      0xCE => {return Some(203);}
      0xCF => {return Some(204);}
      0xD0 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(205);}
         0x1 => {return Some(206);}
         0x2 => {return Some(207);}
         0x3 => {return Some(208);}
         0x4 => {return Some(209);}
         0x5 => {return Some(210);}
         0x6 => {return Some(211);}
         0x7 => {return Some(212);}
         _ => {return None}}}
      0xD1 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(213);}
         0x1 => {return Some(214);}
         0x2 => {return Some(215);}
         0x3 => {return Some(216);}
         0x4 => {return Some(217);}
         0x5 => {return Some(218);}
         0x6 => {return Some(219);}
         0x7 => {return Some(220);}
         _ => {return None}}}
      0xD2 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(221);}
         0x1 => {return Some(222);}
         0x2 => {return Some(223);}
         0x3 => {return Some(224);}
         0x4 => {return Some(225);}
         0x5 => {return Some(226);}
         0x6 => {return Some(227);}
         0x7 => {return Some(228);}
         _ => {return None}}}
      0xD3 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(229);}
         0x1 => {return Some(230);}
         0x2 => {return Some(231);}
         0x3 => {return Some(232);}
         0x4 => {return Some(233);}
         0x5 => {return Some(234);}
         0x6 => {return Some(235);}
         0x7 => {return Some(236);}
         _ => {return None}}}
      0xD7 => {return Some(237);}
      0xD8 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(238);}
         0x1 => {return Some(239);}
         0x2 => {match byte2{
            0xD1 => {return Some(241);}
            _ => {return Some(240)}}}
         0x3 => {match byte2{
            0xD9 => {return Some(243);}
            _ => {return Some(242)}}}
         0x4 => {return Some(244);}
         0x5 => {return Some(245);}
         0x6 => {return Some(246);}
         0x7 => {return Some(247);}
         _ => {return None}}}
      0xD9 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(248);}
         0x1 => {match byte2{
            0xC9 => {return Some(250);}
            _ => {return Some(249)}}}
         0x2 => {match byte2{
            0xD0 => {return Some(252);}
            _ => {return Some(251)}}}
         0x3 => {return Some(253);}
         0x4 => {match byte2{
            0xE0 => {return Some(256);}
            0xE1 => {return Some(257);}
            0xE4 => {return Some(258);}
            0xE5 => {return Some(259);}
            _ => {return Some(255)}}}
         0x5 => {match byte2{
            0xE8 => {return Some(261);}
            0xE9 => {return Some(262);}
            0xEA => {return Some(263);}
            0xEB => {return Some(264);}
            0xEC => {return Some(265);}
            0xED => {return Some(266);}
            0xEE => {return Some(267);}
            _ => {return Some(260)}}}
         0x6 => {match byte2{
            0xF0 => {return Some(269);}
            0xF1 => {return Some(270);}
            0xF2 => {return Some(271);}
            0xF3 => {return Some(272);}
            0xF4 => {return Some(273);}
            0xF5 => {return Some(274);}
            0xF6 => {return Some(275);}
            0xF7 => {return Some(276);}
            _ => {return Some(268)}}}
         0x7 => {match byte2{
            0xF8 => {return Some(278);}
            0xF9 => {return Some(279);}
            0xFA => {return Some(280);}
            0xFB => {return Some(281);}
            0xFC => {return Some(282);}
            0xFD => {return Some(283);}
            0xFE => {return Some(284);}
            0xFF => {return Some(285);}
            _ => {return Some(277)}}}
         _ => {return None}}}
      0xDA => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(286);}
         0x1 => {return Some(288);}
         0x2 => {return Some(290);}
         0x3 => {return Some(292);}
         0x4 => {return Some(294);}
         0x5 => {match byte2{
            0xE9 => {return Some(296);}
            _ => {return Some(295)}}}
         0x6 => {return Some(297);}
         0x7 => {return Some(298);}
         _ => {return None}}}
      0xDB => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(299);}
         0x1 => {return Some(301);}
         0x2 => {return Some(303);}
         0x3 => {return Some(305);}
         0x4 => {match byte2{
            0xE0 => {return Some(307);}
            0xE1 => {return Some(308);}
            0xE2 => {return Some(309);}
            0xE3 => {return Some(310);}
            0xE4 => {return Some(311);}
            _ => {return None}}}
         0x5 => {return Some(312);}
         0x6 => {return Some(314);}
         0x7 => {return Some(315);}
         _ => {return None}}}
      0xDC => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(316);}
         0x1 => {return Some(318);}
         0x2 => {return Some(320);}
         0x3 => {return Some(322);}
         0x4 => {return Some(324);}
         0x5 => {return Some(326);}
         0x6 => {return Some(328);}
         0x7 => {return Some(330);}
         _ => {return None}}}
      0xDD => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(332);}
         0x1 => {return Some(334);}
         0x2 => {return Some(336);}
         0x3 => {return Some(338);}
         0x4 => {match byte2{
            0xE1 => {return Some(341);}
            _ => {return Some(340)}}}
         0x5 => {match byte2{
            0xE9 => {return Some(343);}
            _ => {return Some(342)}}}
         0x7 => {return Some(344);}
         _ => {return None}}}
      0xDE => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {match byte2{
            0xC1 => {return Some(347);}
            _ => {return Some(345)}}}
         0x1 => {match byte2{
            0xC9 => {return Some(350);}
            _ => {return Some(348)}}}
         0x2 => {return Some(351);}
         0x3 => {match byte2{
            0xD9 => {return Some(354);}
            _ => {return Some(353)}}}
         0x4 => {match byte2{
            0xE1 => {return Some(357);}
            _ => {return Some(355)}}}
         0x5 => {match byte2{
            0xE9 => {return Some(360);}
            _ => {return Some(358)}}}
         0x6 => {match byte2{
            0xF1 => {return Some(363);}
            _ => {return Some(361)}}}
         0x7 => {match byte2{
            0xF9 => {return Some(366);}
            _ => {return Some(364)}}}
         _ => {return None}}}
      0xDF => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(367);}
         0x1 => {return Some(369);}
         0x2 => {return Some(371);}
         0x3 => {return Some(373);}
         0x4 => {match byte2{
            0xE0 => {return Some(376);}
            _ => {return Some(375)}}}
         0x5 => {return Some(377);}
         0x6 => {return Some(379);}
         0x7 => {return Some(381);}
         _ => {return None}}}
      0xE0 => {return Some(382);}
      0xE1 => {return Some(383);}
      0xE2 => {return Some(384);}
      0xE3 => {return Some(385);}
      0xE4 => {return Some(386);}
      0xE5 => {return Some(387);}
      0xE6 => {return Some(388);}
      0xE7 => {return Some(389);}
      0xE8 => {return Some(390);}
      0xE9 => {return Some(391);}
      0xEB => {return Some(392);}
      0xEC => {return Some(393);}
      0xED => {return Some(394);}
      0xEE => {return Some(395);}
      0xEF => {return Some(396);}
      0xF1 => {return Some(397);}
      0xF4 => {return Some(399);}
      0xF5 => {return Some(400);}
      0xF6 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(401);}
         0x1 => {return Some(402);}
         0x2 => {return Some(403);}
         0x3 => {return Some(404);}
         0x4 => {return Some(405);}
         0x5 => {return Some(406);}
         0x6 => {return Some(407);}
         0x7 => {return Some(408);}
         _ => {return None}}}
      0xF7 => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(409);}
         0x1 => {return Some(410);}
         0x2 => {return Some(411);}
         0x3 => {return Some(412);}
         0x4 => {return Some(413);}
         0x5 => {return Some(414);}
         0x6 => {return Some(415);}
         0x7 => {return Some(416);}
         _ => {return None}}}
      0xF8 => {return Some(417);}
      0xF9 => {return Some(418);}
      0xFA => {return Some(419);}
      0xFB => {return Some(420);}
      0xFC => {return Some(421);}
      0xFD => {return Some(422);}
      0xFE => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(423);}
         0x1 => {return Some(424);}
         _ => {return None}}}
      0xFF => {match r_bits!(byte2){ // RM 3bit value as opcode
         0x0 => {return Some(425);}
         0x1 => {return Some(426);}
         0x2 => {return Some(427);}
         0x3 => {return Some(429);}
         0x4 => {return Some(430);}
         0x5 => {return Some(432);}
         0x6 => {return Some(433);}
         _ => {return None}}}
      _ => {return None}}}
//
lazy_static!{static ref INSTRUCTIONS:Vec<instruction> = vec![
   instruction{name:"ADD".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x00, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x01, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x02, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x03, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x04, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x05, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x08, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x09, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x0A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x0B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x0C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OR".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x0D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"_0F".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0x0F, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x10, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x11, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x12, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x13, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x14, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADC".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x15, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x18, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x19, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x1A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x1B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x1C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SBB".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x1D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x20, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x21, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x22, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x23, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x24, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"AND".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x25, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x28, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x29, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x2A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x2B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x2C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SUB".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x2D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x30, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x31, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x32, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x33, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x34, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XOR".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x35, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x38, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x39, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x3A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x3B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x3C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMP".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x3D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x50, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc_length:1, opc1:0x51, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc_length:1, opc1:0x52, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc_length:1, opc1:0x53, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc_length:1, opc1:0x54, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc_length:1, opc1:0x55, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc_length:1, opc1:0x56, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc_length:1, opc1:0x57, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x58, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc_length:1, opc1:0x59, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc_length:1, opc1:0x5a, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc_length:1, opc1:0x5b, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc_length:1, opc1:0x5c, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc_length:1, opc1:0x5d, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc_length:1, opc1:0x5e, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc_length:1, opc1:0x5f, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOVSXD".to_owned(), params:vec![operand::r32_64,operand::r_m32,], rm_byte:rm_type::available, opc_length:2, opc1:0x63, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x68, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IMUL".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::available, opc_length:2, opc1:0x69, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0x6A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IMUL".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::available, opc_length:2, opc1:0x6B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INS".to_owned(), params:vec![operand::m8,operand::DX,], rm_byte:rm_type::none, opc_length:1, opc1:0x6C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INS".to_owned(), params:vec![operand::m16,operand::DX,], rm_byte:rm_type::none, opc_length:1, opc1:0x6D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INS".to_owned(), params:vec![operand::m16_32,operand::DX,], rm_byte:rm_type::none, opc_length:1, opc1:0x6D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m8,], rm_byte:rm_type::none, opc_length:1, opc1:0x6E, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m16,], rm_byte:rm_type::none, opc_length:1, opc1:0x6F, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUTS".to_owned(), params:vec![operand::DX,operand::m16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0x6F, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JO".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x70, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNO".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x71, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JB".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x72, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNB".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x73, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JZ".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x74, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNZ".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x75, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JBE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x76, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNBE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x77, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JS".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x78, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNS".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x79, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JL".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNL".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JLE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7E, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JNLE".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0x7F, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ADD".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"OR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"ADC".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"SBB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"AND".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SUB".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"XOR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"CMP".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x80, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x81, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ADD".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"OR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"ADC".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"SBB".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"AND".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SUB".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"XOR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"CMP".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x83, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x84, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x85, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x86, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x87, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:2, opc1:0x88, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x89, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r8,operand::r_m8,], rm_byte:rm_type::available, opc_length:2, opc1:0x8A, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:2, opc1:0x8B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::m16,operand::Sreg,], rm_byte:rm_type::available, opc_length:2, opc1:0x8C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LEA".to_owned(), params:vec![operand::r16_32_64,operand::m,], rm_byte:rm_type::available, opc_length:2, opc1:0x8D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::Sreg,operand::r_m16,], rm_byte:rm_type::available, opc_length:2, opc1:0x8E, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x8F, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"POP".to_owned(), params:vec![operand::r_m64_16,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0x8F, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eAX,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x90, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eCX,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x91, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eDX,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x92, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eBX,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x93, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eSP,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x94, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eBP,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x95, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eSI,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x96, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"XCHG".to_owned(), params:vec![operand::eDI,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0x97, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"NOP".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0x90, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CBW".to_owned(), params:vec![operand::AX,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0x98, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CWD".to_owned(), params:vec![operand::DX,operand::AX,], rm_byte:rm_type::none, opc_length:1, opc1:0x99, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"FWAIT".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0x9B, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"PUSHF".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc_length:1, opc1:0x9C, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"POPF".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc_length:1, opc1:0x9D, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SAHF".to_owned(), params:vec![operand::AH,], rm_byte:rm_type::none, opc_length:1, opc1:0x9E, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LAHF".to_owned(), params:vec![operand::AH,], rm_byte:rm_type::none, opc_length:1, opc1:0x9F, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::AL,operand::moffs8,], rm_byte:rm_type::none, opc_length:1, opc1:0xA0, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::rAX,operand::moffs16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xA1, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::moffs8,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0xA2, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::moffs16_32_64,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0xA3, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOVS".to_owned(), params:vec![operand::m8,operand::m8,], rm_byte:rm_type::none, opc_length:1, opc1:0xA4, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOVS".to_owned(), params:vec![operand::m16_32_64,operand::m16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xA5, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMPS".to_owned(), params:vec![operand::m8,operand::m8,], rm_byte:rm_type::none, opc_length:1, opc1:0xA6, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMPS".to_owned(), params:vec![operand::m16_32_64,operand::m16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xA7, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"TEST".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xA8, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"TEST".to_owned(), params:vec![operand::rAX,operand::imm16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0xA9, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"STOS".to_owned(), params:vec![operand::m8,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0xAA, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"STOS".to_owned(), params:vec![operand::m16_32_64,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0xAB, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LODS".to_owned(), params:vec![operand::AL,operand::m8,], rm_byte:rm_type::none, opc_length:1, opc1:0xAC, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LODS".to_owned(), params:vec![operand::rAX,operand::m16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xAD, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SCAS".to_owned(), params:vec![operand::m8,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0xAE, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"SCAS".to_owned(), params:vec![operand::m16_32_64,operand::rAX,], rm_byte:rm_type::none, opc_length:1, opc1:0xAF, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb0, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::CL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb1, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::DL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb2, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::BL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb3, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::AH,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb4, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::CH,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb5, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::DH,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb6, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::BH,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xb7, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eAX,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xb8, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eCX,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xb9, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eDX,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xba, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eBX,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xbb, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eSP,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xbc, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eBP,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xbd, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eSI,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xbe, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::eDI,operand::imm16_32_64,], rm_byte:rm_type::none, opc_length:1, opc1:0xbf, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC0, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC1, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"RETN".to_owned(), params:vec![operand::imm16,], rm_byte:rm_type::none, opc_length:1, opc1:0xC2, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"RETN".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xC3, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC6, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"MOV".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xC7, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ENTER".to_owned(), params:vec![operand::rBP,operand::imm16,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xC8, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LEAVE".to_owned(), params:vec![operand::rBP,], rm_byte:rm_type::none, opc_length:1, opc1:0xC9, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"RETF".to_owned(), params:vec![operand::imm16,], rm_byte:rm_type::none, opc_length:1, opc1:0xCA, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"RETF".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xCB, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INT".to_owned(), params:vec![operand::_3,operand::eFlags,], rm_byte:rm_type::none, opc_length:1, opc1:0xCC, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INT".to_owned(), params:vec![operand::imm8,operand::eFlags,], rm_byte:rm_type::none, opc_length:1, opc1:0xCD, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INTO".to_owned(), params:vec![operand::eFlags,], rm_byte:rm_type::none, opc_length:1, opc1:0xCE, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IRET".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc_length:1, opc1:0xCF, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD0, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32_64,operand::_1,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD1, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m8,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD2, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"ROL".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"ROR".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"RCL".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"RCR".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SHL".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"SHR".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SAL".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SAR".to_owned(), params:vec![operand::r_m16_32_64,operand::CL,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD3, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"XLAT".to_owned(), params:vec![operand::AL,operand::m8,], rm_byte:rm_type::none, opc_length:1, opc1:0xD7, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"FADD".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FMUL".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD8, opc2:Some(0xD1), opc3:None, opc4_reg:None},
   instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD8, opc2:Some(0xD9), opc3:None, opc4_reg:None},
   instruction{name:"FSUB".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FSUBR".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FDIV".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FDIVR".to_owned(), params:vec![operand::ST,operand::m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD8, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::STi_m32real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FXCH".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FXCH".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xC9), opc3:None, opc4_reg:None},
   instruction{name:"FST".to_owned(), params:vec![operand::m32real,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FNOP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xD0), opc3:None, opc4_reg:None},
   instruction{name:"FSTP".to_owned(), params:vec![operand::m32real,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FSTP1".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FLDENV".to_owned(), params:vec![operand::m14_28,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FCHS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE0), opc3:None, opc4_reg:None},
   instruction{name:"FABS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE1), opc3:None, opc4_reg:None},
   instruction{name:"FTST".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE4), opc3:None, opc4_reg:None},
   instruction{name:"FXAM".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE5), opc3:None, opc4_reg:None},
   instruction{name:"FLDCW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FLD1".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE8), opc3:None, opc4_reg:None},
   instruction{name:"FLDL2T".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xE9), opc3:None, opc4_reg:None},
   instruction{name:"FLDL2E".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xEA), opc3:None, opc4_reg:None},
   instruction{name:"FLDPI".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xEB), opc3:None, opc4_reg:None},
   instruction{name:"FLDLG2".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xEC), opc3:None, opc4_reg:None},
   instruction{name:"FLDLN2".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xED), opc3:None, opc4_reg:None},
   instruction{name:"FLDZ".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xEE), opc3:None, opc4_reg:None},
   instruction{name:"FNSTENV".to_owned(), params:vec![operand::m14_28,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"F2XM1".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF0), opc3:None, opc4_reg:None},
   instruction{name:"FYL2X".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF1), opc3:None, opc4_reg:None},
   instruction{name:"FPTAN".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF2), opc3:None, opc4_reg:None},
   instruction{name:"FPATAN".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF3), opc3:None, opc4_reg:None},
   instruction{name:"FXTRACT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF4), opc3:None, opc4_reg:None},
   instruction{name:"FPREM1".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF5), opc3:None, opc4_reg:None},
   instruction{name:"FDECSTP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF6), opc3:None, opc4_reg:None},
   instruction{name:"FINCSTP".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF7), opc3:None, opc4_reg:None},
   instruction{name:"FNSTCW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xD9, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FPREM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF8), opc3:None, opc4_reg:None},
   instruction{name:"FYL2XP1".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xF9), opc3:None, opc4_reg:None},
   instruction{name:"FSQRT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFA), opc3:None, opc4_reg:None},
   instruction{name:"FSINCOS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFB), opc3:None, opc4_reg:None},
   instruction{name:"FRNDINT".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFC), opc3:None, opc4_reg:None},
   instruction{name:"FSCALE".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFD), opc3:None, opc4_reg:None},
   instruction{name:"FSIN".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFE), opc3:None, opc4_reg:None},
   instruction{name:"FCOS".to_owned(), params:vec![operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xD9, opc2:Some(0xFF), opc3:None, opc4_reg:None},
   instruction{name:"FIADD".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FCMOVB".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FIMUL".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FCMOVE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FICOM".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCMOVBE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FICOMP".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FCMOVU".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FISUB".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FISUBR".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FUCOMPP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDA, opc2:Some(0xE9), opc3:None, opc4_reg:None},
   instruction{name:"FIDIV".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FIDIVR".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDA, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m32int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FCMOVNB".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FISTTP".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FCMOVNE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FIST".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCMOVNBE".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FISTP".to_owned(), params:vec![operand::m32int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FCMOVNU".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FNENI nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDB, opc2:Some(0xE0), opc3:None, opc4_reg:None},
   instruction{name:"FNDISI nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDB, opc2:Some(0xE1), opc3:None, opc4_reg:None},
   instruction{name:"FNCLEX".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDB, opc2:Some(0xE2), opc3:None, opc4_reg:None},
   instruction{name:"FNINIT".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDB, opc2:Some(0xE3), opc3:None, opc4_reg:None},
   instruction{name:"FNSETPM nop".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDB, opc2:Some(0xE4), opc3:None, opc4_reg:None},
   instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::m80real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FUCOMI".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FCOMI".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FSTP".to_owned(), params:vec![operand::m80real,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDB, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FADD".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FADD".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FMUL".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FMUL".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FCOM".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCOM2".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCOMP".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FCOMP3".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FSUB".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FSUBR".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FSUBR".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FSUB".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FDIV".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FDIVR".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FDIVR".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FDIV".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDC, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FLD".to_owned(), params:vec![operand::ST,operand::m64real,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FFREE".to_owned(), params:vec![operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FISTTP".to_owned(), params:vec![operand::m64int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FXCH4".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FST".to_owned(), params:vec![operand::m64real,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FST".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FSTP".to_owned(), params:vec![operand::m64real,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FSTP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FUCOM".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FUCOM".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDD, opc2:Some(0xE1), opc3:None, opc4_reg:None},
   instruction{name:"FUCOMP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FUCOMP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDD, opc2:Some(0xE9), opc3:None, opc4_reg:None},
   instruction{name:"FNSTSW".to_owned(), params:vec![operand::m16,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDD, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FIADD".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FADDP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FADDP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xC1), opc3:None, opc4_reg:None},
   instruction{name:"FIMUL".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FMULP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FMULP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xC9), opc3:None, opc4_reg:None},
   instruction{name:"FICOM".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FCOMP5".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FICOMP".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FCOMPP".to_owned(), params:vec![operand::ST,operand::ST1,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xD9), opc3:None, opc4_reg:None},
   instruction{name:"FISUB".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FSUBRP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FSUBRP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xE1), opc3:None, opc4_reg:None},
   instruction{name:"FISUBR".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FSUBP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FSUBP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xE9), opc3:None, opc4_reg:None},
   instruction{name:"FIDIV".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FDIVRP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FDIVRP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xF1), opc3:None, opc4_reg:None},
   instruction{name:"FIDIVR".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FDIVP".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDE, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"FDIVP".to_owned(), params:vec![operand::ST1,operand::ST,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDE, opc2:Some(0xF9), opc3:None, opc4_reg:None},
   instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m16int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FFREEP".to_owned(), params:vec![operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"FISTTP".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FXCH7".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"FIST".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FSTP8".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"FISTP".to_owned(), params:vec![operand::m16int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FSTP9".to_owned(), params:vec![operand::STi,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"FBLD".to_owned(), params:vec![operand::ST,operand::m80dec,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"FNSTSW".to_owned(), params:vec![operand::AX,], rm_byte:rm_type::full_opcode, opc_length:2, opc1:0xDF, opc2:Some(0xE0), opc3:None, opc4_reg:None},
   instruction{name:"FILD".to_owned(), params:vec![operand::ST,operand::m64int,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FUCOMIP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"FBSTP".to_owned(), params:vec![operand::m80dec,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FCOMIP".to_owned(), params:vec![operand::ST,operand::STi,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"FISTP".to_owned(), params:vec![operand::m64int,operand::ST,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xDF, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"LOOPNZ".to_owned(), params:vec![operand::rCX,operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0xE0, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LOOPZ".to_owned(), params:vec![operand::rCX,operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0xE1, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"LOOP".to_owned(), params:vec![operand::rCX,operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0xE2, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JECXZ".to_owned(), params:vec![operand::rel8,operand::ECX,], rm_byte:rm_type::none, opc_length:1, opc1:0xE3, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IN".to_owned(), params:vec![operand::AL,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xE4, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IN".to_owned(), params:vec![operand::eAX,operand::imm8,], rm_byte:rm_type::none, opc_length:1, opc1:0xE5, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUT".to_owned(), params:vec![operand::imm8,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0xE6, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUT".to_owned(), params:vec![operand::imm8,operand::eAX,], rm_byte:rm_type::none, opc_length:1, opc1:0xE7, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CALL".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0xE8, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JMP".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:1, opc1:0xE9, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"JMP".to_owned(), params:vec![operand::rel8,], rm_byte:rm_type::none, opc_length:1, opc1:0xEB, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IN".to_owned(), params:vec![operand::AL,operand::DX,], rm_byte:rm_type::none, opc_length:1, opc1:0xEC, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"IN".to_owned(), params:vec![operand::eAX,operand::DX,], rm_byte:rm_type::none, opc_length:1, opc1:0xED, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUT".to_owned(), params:vec![operand::DX,operand::AL,], rm_byte:rm_type::none, opc_length:1, opc1:0xEE, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"OUT".to_owned(), params:vec![operand::DX,operand::eAX,], rm_byte:rm_type::none, opc_length:1, opc1:0xEF, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"undefined".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xF1, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INT1".to_owned(), params:vec![operand::eFlags,], rm_byte:rm_type::none, opc_length:1, opc1:0xF1, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"HLT".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xF4, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CMC".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xF5, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m8,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"NOT".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"NEG".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"MUL".to_owned(), params:vec![operand::AX,operand::AL,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"IMUL".to_owned(), params:vec![operand::AX,operand::AL,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"DIV".to_owned(), params:vec![operand::AL,operand::AH,operand::AX,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"IDIV".to_owned(), params:vec![operand::AL,operand::AH,operand::AX,operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF6, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"TEST".to_owned(), params:vec![operand::r_m16_32_64,operand::imm16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"NOT".to_owned(), params:vec![operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"NEG".to_owned(), params:vec![operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"MUL".to_owned(), params:vec![operand::rDX,operand::rAX,operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"IMUL".to_owned(), params:vec![operand::rDX,operand::rAX,operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"DIV".to_owned(), params:vec![operand::rDX,operand::rAX,operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"IDIV".to_owned(), params:vec![operand::rDX,operand::rAX,operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xF7, opc2:None, opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"CLC".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xF8, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"STC".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xF9, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CLI".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xFA, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"STI".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xFB, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"CLD".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xFC, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"STD".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:1, opc1:0xFD, opc2:None, opc3:None, opc4_reg:None},
   instruction{name:"INC".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFE, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"DEC".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFE, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"INC".to_owned(), params:vec![operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"DEC".to_owned(), params:vec![operand::r_m16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"CALL".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"CALL".to_owned(), params:vec![operand::r_m64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"CALLF".to_owned(), params:vec![operand::m16_16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"JMP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"JMP".to_owned(), params:vec![operand::r_m64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"JMPF".to_owned(), params:vec![operand::m16_16_32_64,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"PUSH".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"PUSH".to_owned(), params:vec![operand::r_m64_16,], rm_byte:rm_type::reg_opcode, opc_length:1, opc1:0xFF, opc2:None, opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SLDT".to_owned(), params:vec![operand::m16,operand::LDTR,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"STR".to_owned(), params:vec![operand::m16,operand::TR,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"LLDT".to_owned(), params:vec![operand::LDTR,operand::r_m16,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"LTR".to_owned(), params:vec![operand::TR,operand::r_m16,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"VERR".to_owned(), params:vec![operand::r_m16,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"VERW".to_owned(), params:vec![operand::r_m16,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x00), opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"SGDT".to_owned(), params:vec![operand::m,operand::GDTR,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"VMCALL".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC1), opc4_reg:None},
   instruction{name:"VMLAUNCH".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC2), opc4_reg:None},
   instruction{name:"VMRESUME".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC3), opc4_reg:None},
   instruction{name:"VMXOFF".to_owned(), params:vec![], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC4), opc4_reg:None},
   instruction{name:"SIDT".to_owned(), params:vec![operand::m,operand::IDTR,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"MONITOR".to_owned(), params:vec![operand::m8,operand::ECX,operand::EDX,], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC8), opc4_reg:None},
   instruction{name:"MWAIT".to_owned(), params:vec![operand::EAX,operand::ECX,], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xC9), opc4_reg:None},
   instruction{name:"LGDT".to_owned(), params:vec![operand::GDTR,operand::m,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"XGETBV".to_owned(), params:vec![operand::EDX,operand::EAX,operand::ECX,operand::XCR,], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xD0), opc4_reg:None},
   instruction{name:"XSETBV".to_owned(), params:vec![operand::XCR,operand::ECX,operand::EDX,operand::EAX,], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xD1), opc4_reg:None},
   instruction{name:"LIDT".to_owned(), params:vec![operand::IDTR,operand::m,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"SMSW".to_owned(), params:vec![operand::m16,operand::MSW,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"LMSW".to_owned(), params:vec![operand::MSW,operand::r_m16,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"INVLPG".to_owned(), params:vec![operand::m,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x01), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"SWAPGS".to_owned(), params:vec![operand::GS,operand::IA32_KERNEL_GSBASE,], rm_byte:rm_type::full_opcode, opc_length:3, opc1:0x0F, opc2:Some(0x01), opc3:Some(0xF8), opc4_reg:None},
   instruction{name:"LAR".to_owned(), params:vec![operand::r16_32_64,operand::m16,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x02), opc3:None, opc4_reg:None},
   instruction{name:"LSL".to_owned(), params:vec![operand::r16_32_64,operand::m16,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x03), opc3:None, opc4_reg:None},
   instruction{name:"CLTS".to_owned(), params:vec![operand::CR0,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x06), opc3:None, opc4_reg:None},
   instruction{name:"INVD".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x08), opc3:None, opc4_reg:None},
   instruction{name:"WBINVD".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x09), opc3:None, opc4_reg:None},
   instruction{name:"UD2".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x0B), opc3:None, opc4_reg:None},
   instruction{name:"NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x0D), opc3:None, opc4_reg:None},
   instruction{name:"MOVUPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x10), opc3:None, opc4_reg:None},
   instruction{name:"MOVUPS".to_owned(), params:vec![operand::xmm_m128,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x11), opc3:None, opc4_reg:None},
   instruction{name:"MOVHLPS".to_owned(), params:vec![operand::xmm,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x12), opc3:None, opc4_reg:None},
   instruction{name:"MOVLPS".to_owned(), params:vec![operand::xmm,operand::m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x12), opc3:None, opc4_reg:None},
   instruction{name:"MOVLPS".to_owned(), params:vec![operand::m64,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x13), opc3:None, opc4_reg:None},
   instruction{name:"UNPCKLPS".to_owned(), params:vec![operand::xmm,operand::xmm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x14), opc3:None, opc4_reg:None},
   instruction{name:"UNPCKHPS".to_owned(), params:vec![operand::xmm,operand::xmm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x15), opc3:None, opc4_reg:None},
   instruction{name:"MOVLHPS".to_owned(), params:vec![operand::xmm,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x16), opc3:None, opc4_reg:None},
   instruction{name:"MOVHPS".to_owned(), params:vec![operand::xmm,operand::m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x16), opc3:None, opc4_reg:None},
   instruction{name:"MOVHPS".to_owned(), params:vec![operand::m64,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x17), opc3:None, opc4_reg:None},
   instruction{name:"PREFETCHNTA".to_owned(), params:vec![operand::m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"PREFETCHT0".to_owned(), params:vec![operand::m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"PREFETCHT1".to_owned(), params:vec![operand::m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"PREFETCHT2".to_owned(), params:vec![operand::m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x18), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x19), opc3:None, opc4_reg:None},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x1A), opc3:None, opc4_reg:None},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x1B), opc3:None, opc4_reg:None},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x1C), opc3:None, opc4_reg:None},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x1D), opc3:None, opc4_reg:None},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x1E), opc3:None, opc4_reg:None},
   instruction{name:"NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x1)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"HINT_NOP".to_owned(), params:vec![operand::r_m16_32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x1F), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"MOV".to_owned(), params:vec![operand::r64,operand::CRn,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x20), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r64,operand::CRn,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x20), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r64,operand::DRn,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x21), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::r64,operand::DRn,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x21), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::CRn,operand::r64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x22), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::CRn,operand::r64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x22), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::DRn,operand::r64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x23), opc3:None, opc4_reg:None},
   instruction{name:"MOV".to_owned(), params:vec![operand::DRn,operand::r64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x23), opc3:None, opc4_reg:None},
   instruction{name:"MOVAPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x28), opc3:None, opc4_reg:None},
   instruction{name:"MOVAPS".to_owned(), params:vec![operand::xmm_m128,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x29), opc3:None, opc4_reg:None},
   instruction{name:"CVTPI2PS".to_owned(), params:vec![operand::xmm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2A), opc3:None, opc4_reg:None},
   instruction{name:"MOVNTPS".to_owned(), params:vec![operand::m128,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2B), opc3:None, opc4_reg:None},
   instruction{name:"CVTTPS2PI".to_owned(), params:vec![operand::mm,operand::xmm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2C), opc3:None, opc4_reg:None},
   instruction{name:"CVTPS2PI".to_owned(), params:vec![operand::mm,operand::xmm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2D), opc3:None, opc4_reg:None},
   instruction{name:"UCOMISS".to_owned(), params:vec![operand::xmm,operand::xmm_m32,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2E), opc3:None, opc4_reg:None},
   instruction{name:"COMISS".to_owned(), params:vec![operand::xmm,operand::xmm_m32,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x2F), opc3:None, opc4_reg:None},
   instruction{name:"WRMSR".to_owned(), params:vec![operand::MSR,operand::rCX,operand::rAX,operand::rDX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x30), opc3:None, opc4_reg:None},
   instruction{name:"RDTSC".to_owned(), params:vec![operand::EAX,operand::EDX,operand::IA32_TIME_STAMP_COUNTER,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x31), opc3:None, opc4_reg:None},
   instruction{name:"RDMSR".to_owned(), params:vec![operand::rAX,operand::rDX,operand::rCX,operand::MSR,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x32), opc3:None, opc4_reg:None},
   instruction{name:"RDPMC".to_owned(), params:vec![operand::EAX,operand::EDX,operand::PMC,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x33), opc3:None, opc4_reg:None},
   instruction{name:"GETSEC".to_owned(), params:vec![operand::EAX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x37), opc3:None, opc4_reg:None},
   instruction{name:"MOVBE".to_owned(), params:vec![operand::r16_32_64,operand::m16_32_64,], rm_byte:rm_type::available, opc_length:4, opc1:0x0F, opc2:Some(0x38), opc3:Some(0xF0), opc4_reg:None},
   instruction{name:"MOVBE".to_owned(), params:vec![operand::m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:4, opc1:0x0F, opc2:Some(0x38), opc3:Some(0xF1), opc4_reg:None},
   instruction{name:"PALIGNR".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:4, opc1:0x0F, opc2:Some(0x3A), opc3:Some(0x0F), opc4_reg:None},
   instruction{name:"CMOVO".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x40), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNO".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x41), opc3:None, opc4_reg:None},
   instruction{name:"CMOVB".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x42), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNB".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x43), opc3:None, opc4_reg:None},
   instruction{name:"CMOVZ".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x44), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNZ".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x45), opc3:None, opc4_reg:None},
   instruction{name:"CMOVBE".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x46), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNBE".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x47), opc3:None, opc4_reg:None},
   instruction{name:"CMOVS".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x48), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNS".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x49), opc3:None, opc4_reg:None},
   instruction{name:"CMOVP".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4A), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNP".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4B), opc3:None, opc4_reg:None},
   instruction{name:"CMOVL".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4C), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNL".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4D), opc3:None, opc4_reg:None},
   instruction{name:"CMOVLE".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4E), opc3:None, opc4_reg:None},
   instruction{name:"CMOVNLE".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x4F), opc3:None, opc4_reg:None},
   instruction{name:"MOVMSKPS".to_owned(), params:vec![operand::r32_64,operand::xmm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x50), opc3:None, opc4_reg:None},
   instruction{name:"SQRTPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x51), opc3:None, opc4_reg:None},
   instruction{name:"RSQRTPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x52), opc3:None, opc4_reg:None},
   instruction{name:"RCPPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x53), opc3:None, opc4_reg:None},
   instruction{name:"ANDPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x54), opc3:None, opc4_reg:None},
   instruction{name:"ANDNPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x55), opc3:None, opc4_reg:None},
   instruction{name:"ORPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x56), opc3:None, opc4_reg:None},
   instruction{name:"XORPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x57), opc3:None, opc4_reg:None},
   instruction{name:"ADDPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x58), opc3:None, opc4_reg:None},
   instruction{name:"MULPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x59), opc3:None, opc4_reg:None},
   instruction{name:"CVTPS2PD".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5A), opc3:None, opc4_reg:None},
   instruction{name:"CVTDQ2PS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5B), opc3:None, opc4_reg:None},
   instruction{name:"SUBPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5C), opc3:None, opc4_reg:None},
   instruction{name:"MINPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5D), opc3:None, opc4_reg:None},
   instruction{name:"DIVPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5E), opc3:None, opc4_reg:None},
   instruction{name:"MAXPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x5F), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKLBW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x60), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKLWD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x61), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKLDQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x62), opc3:None, opc4_reg:None},
   instruction{name:"PACKSSWB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x63), opc3:None, opc4_reg:None},
   instruction{name:"PCMPGTB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x64), opc3:None, opc4_reg:None},
   instruction{name:"PCMPGTW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x65), opc3:None, opc4_reg:None},
   instruction{name:"PCMPGTD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x66), opc3:None, opc4_reg:None},
   instruction{name:"PACKUSWB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x67), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKHBW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x68), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKHWD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x69), opc3:None, opc4_reg:None},
   instruction{name:"PUNPCKHDQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x6A), opc3:None, opc4_reg:None},
   instruction{name:"PACKSSDW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x6B), opc3:None, opc4_reg:None},
   instruction{name:"MOVD".to_owned(), params:vec![operand::mm,operand::r_m32,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x6E), opc3:None, opc4_reg:None},
   instruction{name:"MOVQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x6F), opc3:None, opc4_reg:None},
   instruction{name:"PSHUFW".to_owned(), params:vec![operand::mm,operand::mm_m64,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x70), opc3:None, opc4_reg:None},
   instruction{name:"PSRLW".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x71), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"PSRAW".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x71), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"PSLLW".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x71), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"PSRLD".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x72), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"PSRAD".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x72), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"PSLLD".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x72), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"PSRLQ".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x73), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"PSLLQ".to_owned(), params:vec![operand::mm,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x73), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"PCMPEQB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x74), opc3:None, opc4_reg:None},
   instruction{name:"PCMPEQW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x75), opc3:None, opc4_reg:None},
   instruction{name:"PCMPEQD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x76), opc3:None, opc4_reg:None},
   instruction{name:"EMMS".to_owned(), params:vec![], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x77), opc3:None, opc4_reg:None},
   instruction{name:"VMREAD".to_owned(), params:vec![operand::r_m64,operand::r64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x78), opc3:None, opc4_reg:None},
   instruction{name:"VMWRITE".to_owned(), params:vec![operand::r64,operand::r_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x79), opc3:None, opc4_reg:None},
   instruction{name:"MOVD".to_owned(), params:vec![operand::r_m32,operand::mm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x7E), opc3:None, opc4_reg:None},
   instruction{name:"MOVQ".to_owned(), params:vec![operand::mm_m64,operand::mm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0x7F), opc3:None, opc4_reg:None},
   instruction{name:"JO".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x80), opc3:None, opc4_reg:None},
   instruction{name:"JNO".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x81), opc3:None, opc4_reg:None},
   instruction{name:"JB".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x82), opc3:None, opc4_reg:None},
   instruction{name:"JNB".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x83), opc3:None, opc4_reg:None},
   instruction{name:"JZ".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x84), opc3:None, opc4_reg:None},
   instruction{name:"JNZ".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x85), opc3:None, opc4_reg:None},
   instruction{name:"JBE".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x86), opc3:None, opc4_reg:None},
   instruction{name:"JNBE".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x87), opc3:None, opc4_reg:None},
   instruction{name:"JS".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x88), opc3:None, opc4_reg:None},
   instruction{name:"JNS".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x89), opc3:None, opc4_reg:None},
   instruction{name:"JP".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8A), opc3:None, opc4_reg:None},
   instruction{name:"JNP".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8B), opc3:None, opc4_reg:None},
   instruction{name:"JL".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8C), opc3:None, opc4_reg:None},
   instruction{name:"JNL".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8D), opc3:None, opc4_reg:None},
   instruction{name:"JLE".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8E), opc3:None, opc4_reg:None},
   instruction{name:"JNLE".to_owned(), params:vec![operand::rel16_32,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0x8F), opc3:None, opc4_reg:None},
   instruction{name:"SETO".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x90), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNO".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x91), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETB".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x92), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNB".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x93), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETZ".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x94), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNZ".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x95), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETBE".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x96), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNBE".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x97), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETS".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x98), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNS".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x99), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETP".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9A), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNP".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9B), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETL".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9C), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNL".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9D), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETLE".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9E), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"SETNLE".to_owned(), params:vec![operand::r_m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0x9F), opc3:None, opc4_reg:Some(0x0)},
   instruction{name:"PUSH".to_owned(), params:vec![operand::FS,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xA0), opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::FS,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xA1), opc3:None, opc4_reg:None},
   instruction{name:"BT".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xA3), opc3:None, opc4_reg:None},
   instruction{name:"SHLD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xA4), opc3:None, opc4_reg:None},
   instruction{name:"SHLD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,operand::CL,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xA5), opc3:None, opc4_reg:None},
   instruction{name:"PUSH".to_owned(), params:vec![operand::GS,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xA8), opc3:None, opc4_reg:None},
   instruction{name:"POP".to_owned(), params:vec![operand::GS,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xA9), opc3:None, opc4_reg:None},
   instruction{name:"RSM".to_owned(), params:vec![operand::Flags,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xAA), opc3:None, opc4_reg:None},
   instruction{name:"BTS".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xAB), opc3:None, opc4_reg:None},
   instruction{name:"SHRD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xAC), opc3:None, opc4_reg:None},
   instruction{name:"SHRD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,operand::CL,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xAD), opc3:None, opc4_reg:None},
   instruction{name:"LDMXCSR".to_owned(), params:vec![operand::m32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x2)},
   instruction{name:"STMXCSR".to_owned(), params:vec![operand::m32,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x3)},
   instruction{name:"LFENCE".to_owned(), params:vec![], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"MFENCE".to_owned(), params:vec![], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"SFENCE".to_owned(), params:vec![], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"CLFLUSH".to_owned(), params:vec![operand::m8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xAE), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"IMUL".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xAF), opc3:None, opc4_reg:None},
   instruction{name:"CMPXCHG".to_owned(), params:vec![operand::r_m8,operand::AL,operand::r8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB0), opc3:None, opc4_reg:None},
   instruction{name:"CMPXCHG".to_owned(), params:vec![operand::r_m16_32_64,operand::rAX,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB1), opc3:None, opc4_reg:None},
   instruction{name:"LSS".to_owned(), params:vec![operand::SS,operand::r16_32_64,operand::m16_16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB2), opc3:None, opc4_reg:None},
   instruction{name:"BTR".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB3), opc3:None, opc4_reg:None},
   instruction{name:"LFS".to_owned(), params:vec![operand::FS,operand::r16_32_64,operand::m16_16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB4), opc3:None, opc4_reg:None},
   instruction{name:"LGS".to_owned(), params:vec![operand::GS,operand::r16_32_64,operand::m16_16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB5), opc3:None, opc4_reg:None},
   instruction{name:"MOVZX".to_owned(), params:vec![operand::r16_32_64,operand::r_m8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB6), opc3:None, opc4_reg:None},
   instruction{name:"MOVZX".to_owned(), params:vec![operand::r16_32_64,operand::r_m16,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB7), opc3:None, opc4_reg:None},
   instruction{name:"UD".to_owned(), params:vec![operand::r,operand::r_m,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xB9), opc3:None, opc4_reg:None},
   instruction{name:"BT".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xBA), opc3:None, opc4_reg:Some(0x4)},
   instruction{name:"BTS".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xBA), opc3:None, opc4_reg:Some(0x5)},
   instruction{name:"BTR".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xBA), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"BTC".to_owned(), params:vec![operand::r_m16_32_64,operand::imm8,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xBA), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"BTC".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xBB), opc3:None, opc4_reg:None},
   instruction{name:"BSF".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xBC), opc3:None, opc4_reg:None},
   instruction{name:"BSR".to_owned(), params:vec![operand::r16_32_64,operand::r_m16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xBD), opc3:None, opc4_reg:None},
   instruction{name:"MOVSX".to_owned(), params:vec![operand::r16_32_64,operand::r_m8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xBE), opc3:None, opc4_reg:None},
   instruction{name:"MOVSX".to_owned(), params:vec![operand::r16_32_64,operand::r_m16,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xBF), opc3:None, opc4_reg:None},
   instruction{name:"XADD".to_owned(), params:vec![operand::r_m8,operand::r8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC0), opc3:None, opc4_reg:None},
   instruction{name:"XADD".to_owned(), params:vec![operand::r_m16_32_64,operand::r16_32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC1), opc3:None, opc4_reg:None},
   instruction{name:"CMPPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC2), opc3:None, opc4_reg:None},
   instruction{name:"MOVNTI".to_owned(), params:vec![operand::m32_64,operand::r32_64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC3), opc3:None, opc4_reg:None},
   instruction{name:"PINSRW".to_owned(), params:vec![operand::mm,operand::r32_64,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC4), opc3:None, opc4_reg:None},
   instruction{name:"PEXTRW".to_owned(), params:vec![operand::r32_64,operand::mm,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC5), opc3:None, opc4_reg:None},
   instruction{name:"SHUFPS".to_owned(), params:vec![operand::xmm,operand::xmm_m128,operand::imm8,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xC6), opc3:None, opc4_reg:None},
   instruction{name:"VMPTRLD".to_owned(), params:vec![operand::m64,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xC7), opc3:None, opc4_reg:Some(0x6)},
   instruction{name:"VMPTRST".to_owned(), params:vec![operand::m64,], rm_byte:rm_type::reg_opcode, opc_length:2, opc1:0x0F, opc2:Some(0xC7), opc3:None, opc4_reg:Some(0x7)},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eAX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xc8), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eCX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xc9), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eDX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xca), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eBX,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xcb), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eSP,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xcc), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eBP,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xcd), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eSI,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xce), opc3:None, opc4_reg:None},
   instruction{name:"BSWAP".to_owned(), params:vec![operand::eDI,], rm_byte:rm_type::none, opc_length:2, opc1:0x0F, opc2:Some(0xcf), opc3:None, opc4_reg:None},
   instruction{name:"PSRLW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD1), opc3:None, opc4_reg:None},
   instruction{name:"PSRLD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD2), opc3:None, opc4_reg:None},
   instruction{name:"PSRLQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD3), opc3:None, opc4_reg:None},
   instruction{name:"PADDQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD4), opc3:None, opc4_reg:None},
   instruction{name:"PMULLW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD5), opc3:None, opc4_reg:None},
   instruction{name:"PMOVMSKB".to_owned(), params:vec![operand::r32_64,operand::mm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD7), opc3:None, opc4_reg:None},
   instruction{name:"PSUBUSB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD8), opc3:None, opc4_reg:None},
   instruction{name:"PSUBUSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xD9), opc3:None, opc4_reg:None},
   instruction{name:"PMINUB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDA), opc3:None, opc4_reg:None},
   instruction{name:"PAND".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDB), opc3:None, opc4_reg:None},
   instruction{name:"PADDUSB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDC), opc3:None, opc4_reg:None},
   instruction{name:"PADDUSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDD), opc3:None, opc4_reg:None},
   instruction{name:"PMAXUB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDE), opc3:None, opc4_reg:None},
   instruction{name:"PANDN".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xDF), opc3:None, opc4_reg:None},
   instruction{name:"PAVGB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE0), opc3:None, opc4_reg:None},
   instruction{name:"PSRAW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE1), opc3:None, opc4_reg:None},
   instruction{name:"PSRAD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE2), opc3:None, opc4_reg:None},
   instruction{name:"PAVGW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE3), opc3:None, opc4_reg:None},
   instruction{name:"PMULHUW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE4), opc3:None, opc4_reg:None},
   instruction{name:"PMULHW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE5), opc3:None, opc4_reg:None},
   instruction{name:"MOVNTQ".to_owned(), params:vec![operand::m64,operand::mm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE7), opc3:None, opc4_reg:None},
   instruction{name:"PSUBSB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE8), opc3:None, opc4_reg:None},
   instruction{name:"PSUBSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xE9), opc3:None, opc4_reg:None},
   instruction{name:"PMINSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xEA), opc3:None, opc4_reg:None},
   instruction{name:"POR".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xEB), opc3:None, opc4_reg:None},
   instruction{name:"PADDSB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xEC), opc3:None, opc4_reg:None},
   instruction{name:"PADDSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xED), opc3:None, opc4_reg:None},
   instruction{name:"PMAXSW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xEE), opc3:None, opc4_reg:None},
   instruction{name:"PXOR".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xEF), opc3:None, opc4_reg:None},
   instruction{name:"PSLLW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF1), opc3:None, opc4_reg:None},
   instruction{name:"PSLLD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF2), opc3:None, opc4_reg:None},
   instruction{name:"PSLLQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF3), opc3:None, opc4_reg:None},
   instruction{name:"PMULUDQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF4), opc3:None, opc4_reg:None},
   instruction{name:"PMADDWD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF5), opc3:None, opc4_reg:None},
   instruction{name:"PSADBW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF6), opc3:None, opc4_reg:None},
   instruction{name:"MASKMOVQ".to_owned(), params:vec![operand::m64,operand::mm,operand::mm,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF7), opc3:None, opc4_reg:None},
   instruction{name:"PSUBB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF8), opc3:None, opc4_reg:None},
   instruction{name:"PSUBW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xF9), opc3:None, opc4_reg:None},
   instruction{name:"PSUBD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xFA), opc3:None, opc4_reg:None},
   instruction{name:"PSUBQ".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xFB), opc3:None, opc4_reg:None},
   instruction{name:"PADDB".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xFC), opc3:None, opc4_reg:None},
   instruction{name:"PADDW".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xFD), opc3:None, opc4_reg:None},
   instruction{name:"PADDD".to_owned(), params:vec![operand::mm,operand::mm_m64,], rm_byte:rm_type::available, opc_length:3, opc1:0x0F, opc2:Some(0xFE), opc3:None, opc4_reg:None},
];}