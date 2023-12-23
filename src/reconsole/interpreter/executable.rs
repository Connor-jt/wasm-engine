

pub struct exe{
    dos_header:Option<IMAGE_DOS_HEADER>,
    file_header:Option<IMAGE_DOS_HEADER>,
    optional_header:Option<IMAGE_DOS_HEADER>,
    section_header:Option<Vec<IMAGE_DOS_HEADER>>,
}

pub fn construct_exe(data:&vec<u8>){

}

enum file_header_machine_type{ // SHOULD BE u16
    IMAGE_FILE_MACHINE_UNKNOWN = 0x0, //The content of this field is assumed to be applicable to any machine type
    IMAGE_FILE_MACHINE_ALPHA = 0x184, //Alpha AXP, 32-bit address space
    IMAGE_FILE_MACHINE_ALPHA64 = 0x284, //Alpha 64, 64-bit address space
    IMAGE_FILE_MACHINE_AM33 = 0x1d3, //Matsushita AM33
    IMAGE_FILE_MACHINE_AMD64 = 0x8664, //x64
    IMAGE_FILE_MACHINE_ARM = 0x1c0, //ARM little endian
    IMAGE_FILE_MACHINE_ARM64 = 0xaa64, //ARM64 little endian
    IMAGE_FILE_MACHINE_ARMNT = 0x1c4, //ARM Thumb-2 little endian
    IMAGE_FILE_MACHINE_AXP64 = 0x284, //AXP 64 (Same as Alpha 64)
    IMAGE_FILE_MACHINE_EBC = 0xebc, //EFI byte code
    IMAGE_FILE_MACHINE_I386 = 0x14c, //Intel 386 or later processors and compatible processors
    IMAGE_FILE_MACHINE_IA64 = 0x200, //Intel Itanium processor family
    IMAGE_FILE_MACHINE_LOONGARCH32 = 0x6232, //LoongArch 32-bit processor family
    IMAGE_FILE_MACHINE_LOONGARCH64 = 0x6264, //LoongArch 64-bit processor family
    IMAGE_FILE_MACHINE_M32R = 0x9041, //Mitsubishi M32R little endian
    IMAGE_FILE_MACHINE_MIPS16 = 0x266, //MIPS16
    IMAGE_FILE_MACHINE_MIPSFPU = 0x366, //MIPS with FPU
    IMAGE_FILE_MACHINE_MIPSFPU16 = 0x466, //MIPS16 with FPU
    IMAGE_FILE_MACHINE_POWERPC = 0x1f0, //Power PC little endian
    IMAGE_FILE_MACHINE_POWERPCFP = 0x1f1, //Power PC with floating point support
    IMAGE_FILE_MACHINE_R4000 = 0x166, //MIPS little endian
    IMAGE_FILE_MACHINE_RISCV32 = 0x5032, //RISC-V 32-bit address space
    IMAGE_FILE_MACHINE_RISCV64 = 0x5064, //RISC-V 64-bit address space
    IMAGE_FILE_MACHINE_RISCV128 = 0x5128, //RISC-V 128-bit address space
    IMAGE_FILE_MACHINE_SH3 = 0x1a2, //Hitachi SH3
    IMAGE_FILE_MACHINE_SH3DSP = 0x1a3, //Hitachi SH3 DSP
    IMAGE_FILE_MACHINE_SH4 = 0x1a6, //Hitachi SH4
    IMAGE_FILE_MACHINE_SH5 = 0x1a8, //Hitachi SH5
    IMAGE_FILE_MACHINE_THUMB = 0x1c2, //Thumb
    IMAGE_FILE_MACHINE_WCEMIPSV2 = 0x169, //
}
enum file_header_characteristics { // SHOULD BE u16
    IMAGE_FILE_RELOCS_STRIPPED = 0x0001, //Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
    IMAGE_FILE_EXECUTABLE_IMAGE = 0x0002, //Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
    IMAGE_FILE_LINE_NUMS_STRIPPED = 0x0004, //COFF line numbers have been removed. This flag is deprecated and should be zero.
    IMAGE_FILE_LOCAL_SYMS_STRIPPED = 0x0008, //COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
    IMAGE_FILE_AGGRESSIVE_WS_TRIM = 0x0010, //Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
    IMAGE_FILE_LARGE_ADDRESS_AWARE = 0x0020, //Application can handle > 2-GB addresses.
    IMAGE_FILE_RESERVED = 0x0040, //This flag is reserved for future use.
    IMAGE_FILE_BYTES_REVERSED_LO = 0x0080, //Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
    IMAGE_FILE_32BIT_MACHINE = 0x0100, //Machine is based on a 32-bit-word architecture.
    IMAGE_FILE_DEBUG_STRIPPED = 0x0200, //Debugging information is removed from the image file.
    IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP = 0x0400, //If the image is on removable media, fully load it and copy it to the swap file.
    IMAGE_FILE_NET_RUN_FROM_SWAP = 0x0800, //If the image is on network media, fully load it and copy it to the swap file.
    IMAGE_FILE_SYSTEM = 0x1000, //The image file is a system file, not a user program.
    IMAGE_FILE_DLL = 0x2000, //The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
    IMAGE_FILE_UP_SYSTEM_ONLY = 0x4000, //The file should be run only on a uniprocessor machine.
    IMAGE_FILE_BYTES_REVERSED_HI = 0x8000, //Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
}

#[repr(C, packed)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,      // Magic number
    pub e_cblp: u16,       // Bytes on last page of file
    pub e_cp: u16,         // Pages in file
    pub e_crlc: u16,       // Relocations
    pub e_cparhdr: u16,    // Size of header in paragraphs
    pub e_minalloc: u16,   // Minimum extra paragraphs needed
    pub e_maxalloc: u16,   // Maximum extra paragraphs needed
    pub e_ss: u16,         // Initial (relative) SS value
    pub e_sp: u16,         // Initial SP value
    pub e_csum: u16,       // Checksum
    pub e_ip: u16,         // Initial IP value
    pub e_cs: u16,         // Initial (relative) CS value
    pub e_lfarlc: u16,     // File address of relocation table
    pub e_ovno: u16,       // Overlay number
    pub e_res: [u16; 4],   // Reserved words
    pub e_oemid: u16,      // OEM identifier (for e_oeminfo)
    pub e_oeminfo: u16,    // OEM information; e_oemid specific
    pub e_res2: [u16; 10], // Reserved words
    pub e_lfanew: u32,     // File address of new exe header
} // 64 bytes 

// then DOS stub, 192 bytes

#[repr(C, packed)]
pub struct IMAGE_FILE_HEADER {
    pub magic: u32,
    pub machine: file_header_machine_type, // u16
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: file_header_characteristics, // u16
} // 24 bytes

#[repr(C, packed)]
pub struct IMAGE_OPTIONAL_HEADER {
    pub magic: u16, // 0x10 = 32bit, 0x20 = 64bit 
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    // THERES MORE DATA HERE,
} // 96 bytes

#[repr(C, packed)]
pub struct IMAGE_SECTION_HEADER {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
} // 40 bytes