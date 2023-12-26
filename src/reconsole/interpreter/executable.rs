
use std::{mem, option, ops::Deref};

pub struct loaded_exe{
    pub runtime_data:Vec<u8>,
    pub header: exe_header,
}

pub struct exe_header{
    pub dos_header:IMAGE_DOS_HEADER,
    pub file_header:IMAGE_FILE_HEADER,
    pub optional_header:IMAGE_OPTIONAL_HEADER_64,
    pub optional_header_rva:IMAGE_OPTIONAL_HEADER_RVA,
    pub section_headers:Vec<IMAGE_SECTION_HEADER>,
}
// for some reason these sizes aren't correct so we have to use consts. thanks john rust
const SIZE_OF_IMAGE_DOS_HEADER:u32 = 64;
const SIZE_OF_IMAGE_FILE_HEADER:u32 = 24;
const SIZE_OF_IMAGE_OPTIONAL_HEADER_64:u32 = 112;
const SIZE_OF_IMAGE_DATA_DIRECTORY:u32 = 8;
const SIZE_OF_IMAGE_SECTION_HEADER:u32 = 40;

pub unsafe fn load_exe(data:&Vec<u8>) -> Result<loaded_exe,String>{

    // load header
    let header = cast_exe_header(data)?;


    // find furthest code section?
    let mut largest_offset:u32 = 0;
    for section in header.section_headers.iter(){
        let curr_offset = section.virtual_address + section.virtual_size;
        if curr_offset > largest_offset{ 
            largest_offset = curr_offset; 
        }
    }
    //let mut result = loaded_exe{runtime_data: vec![0; largest_offset as usize], header:None}; // screw you dumb option wrapping pooop system
    //let mut result = loaded_exe{runtime_data: &vec![], header:None};
    // then reallocate all data
    let mut reallocated_data:Vec<u8> = vec![0; largest_offset as usize];// Vec::with_capacity(largest_offset as usize);
    //return Err(format!("size {}", reallocated_data.len()));

    // copy over the header aswell
    reallocated_data.splice(0..header.optional_header.size_of_headers as usize, (&data[0..header.optional_header.size_of_headers as usize]).iter().cloned());

    for section in header.section_headers.iter(){
        let target_data = &data[section.pointer_to_raw_data as usize..(section.pointer_to_raw_data + section.virtual_size) as usize];
        reallocated_data.splice(section.virtual_address as usize.. (section.virtual_address + section.virtual_size) as usize, target_data.iter().cloned());
    }
    


    let new_header = cast_exe_header(&reallocated_data);
    if new_header.is_err(){
        return Err(format!("{}{}", "issue with new header: ".to_owned(), new_header.err().unwrap()));}
   //result.header = Some(new_header.unwrap());

    //return Err("test".to_owned());
    //return Ok(result);
    return Ok(loaded_exe{runtime_data: reallocated_data, header: new_header.unwrap()});
    
}


pub unsafe fn cast_exe_header(data:&Vec<u8>) -> Result<exe_header,String>{
    let mut curr_offset:u32 = 0;
    if (data.len() as u32) < SIZE_OF_IMAGE_DOS_HEADER {return Err("file not large enough to contain dos header".to_owned());}

    let dos_header:IMAGE_DOS_HEADER = cast_ref(data, 0);
    curr_offset += dos_header.e_lfanew;

    if dos_header.e_magic != 23117 {return Err("file does not contain exe signature".to_owned());}

    if data.len() as u32 - curr_offset < SIZE_OF_IMAGE_FILE_HEADER{return Err("file not large enough to contain pe file header".to_owned());}
    
    let file_header:IMAGE_FILE_HEADER = cast_ref(data, curr_offset as usize);
    curr_offset += SIZE_OF_IMAGE_FILE_HEADER;
    
    if data.len() as u32 - curr_offset < SIZE_OF_IMAGE_FILE_HEADER{return Err("file not large enough to contain optional header".to_owned());}
    
    // we should have a condition here that lets us skip this if the size of the optional header is 0, however i believe this section is mandatory for exe & dll
    let optional_header:IMAGE_OPTIONAL_HEADER_64 = cast_ref(data, curr_offset as usize);
    curr_offset += SIZE_OF_IMAGE_OPTIONAL_HEADER_64 as u32;
    
    if optional_header.magic != 0x20B{return Err("optional header specified format other than 64 bit".to_owned());}
    if SIZE_OF_IMAGE_OPTIONAL_HEADER_64 + (optional_header.number_of_rva_and_sizes as u32 * SIZE_OF_IMAGE_DATA_DIRECTORY) != file_header.size_of_optional_header as u32 {
        return Err("optional header did not match expected size".to_owned());}
    if optional_header.number_of_rva_and_sizes > 16 {return Err("optional header had unexpected rva count".to_owned());}
    
    let mut optional_header_rva:IMAGE_OPTIONAL_HEADER_RVA = IMAGE_OPTIONAL_HEADER_RVA{export_table: None, import_table: None, resource_table: None, exception_table: None, certificate_table: None, base_relocation_table: None, debug: None, architecture: None, global_ptr: None, tls_table: None, load_config_table: None, bound_import: None, iat: None, delay_import_descriptor: None, clr_runtime_header: None, reserved_must_be_zero: None};
    // switch on all the things & iterate
    for index in 0..optional_header.number_of_rva_and_sizes{
        match index{
            0 => {optional_header_rva.export_table = Some(cast_ref(data, curr_offset as usize));},
            1 => {optional_header_rva.import_table = Some(cast_ref(data, curr_offset as usize));},
            2 => {optional_header_rva.resource_table = Some(cast_ref(data, curr_offset as usize));},
            3 => {optional_header_rva.exception_table = Some(cast_ref(data, curr_offset as usize));},
            4 => {optional_header_rva.certificate_table = Some(cast_ref(data, curr_offset as usize));},
            5 => {optional_header_rva.base_relocation_table = Some(cast_ref(data, curr_offset as usize));},
            6 => {optional_header_rva.debug = Some(cast_ref(data, curr_offset as usize));},
            7 => {optional_header_rva.architecture = Some(cast_ref(data, curr_offset as usize));},
            8 => {optional_header_rva.global_ptr = Some(cast_ref(data, curr_offset as usize));},
            9 => {optional_header_rva.tls_table = Some(cast_ref(data, curr_offset as usize));},
            10 => {optional_header_rva.load_config_table = Some(cast_ref(data, curr_offset as usize));},
            11 => {optional_header_rva.bound_import = Some(cast_ref(data, curr_offset as usize));},
            12 => {optional_header_rva.iat = Some(cast_ref(data, curr_offset as usize));},
            13 => {optional_header_rva.delay_import_descriptor = Some(cast_ref(data, curr_offset as usize));},
            14 => {optional_header_rva.clr_runtime_header = Some(cast_ref(data, curr_offset as usize));},
            15 => {optional_header_rva.reserved_must_be_zero = Some(cast_ref(data, curr_offset as usize));},
            _ => {return Err("optional header unexpected rva index? redundant error".to_owned());}
        }
        curr_offset += SIZE_OF_IMAGE_DATA_DIRECTORY;
    }
    
    if data.len() as u32 - curr_offset < SIZE_OF_IMAGE_SECTION_HEADER * file_header.number_of_sections as u32{return Err("file not large enough to contain section headers".to_owned());}

    let mut section_headers:Vec<IMAGE_SECTION_HEADER> = vec![];
    for index in 0..file_header.number_of_sections{
        section_headers.push(cast_ref(data, curr_offset as usize));
        curr_offset += SIZE_OF_IMAGE_SECTION_HEADER;
    }
    
    return Ok(exe_header{ dos_header: dos_header, file_header: file_header, optional_header:optional_header, optional_header_rva:optional_header_rva, section_headers:section_headers});
}

unsafe fn cast_ref<T>(bytes: &Vec<u8>, offset:usize) -> T {
    // assert correct endianness somehow
    assert!(bytes.len() - offset >= mem::size_of::<T>());
    let ptr: *const u8 = bytes.as_ptr().wrapping_add(offset);
    //assert_eq!(ptr.align_offset(mem::align_of::<T>()), 0);
    let test = ptr.cast::<T>().read();
    return test;
}

pub enum file_header_machine_type{ // SHOULD BE u16
    IMAGE_FILE_MACHINE_UNKNOWN = 0x0, //The content of this field is assumed to be applicable to any machine type
    IMAGE_FILE_MACHINE_ALPHA = 0x184, //Alpha AXP, 32-bit address space
    IMAGE_FILE_MACHINE_ALPHA64 = 0x284, //Alpha 64, 64-bit address space
    //IMAGE_FILE_MACHINE_AXP64 = 0x284, //AXP 64 (Same as Alpha 64)
    IMAGE_FILE_MACHINE_AM33 = 0x1d3, //Matsushita AM33
    IMAGE_FILE_MACHINE_AMD64 = 0x8664, //x64
    IMAGE_FILE_MACHINE_ARM = 0x1c0, //ARM little endian
    IMAGE_FILE_MACHINE_ARM64 = 0xaa64, //ARM64 little endian
    IMAGE_FILE_MACHINE_ARMNT = 0x1c4, //ARM Thumb-2 little endian
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
pub enum file_header_characteristics { // SHOULD BE u16
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

pub enum optional_header_windows_subsytem{ // SHOULD BE u16
	IMAGE_SUBSYSTEM_UNKNOWN = 0, //An unknown subsystem
	IMAGE_SUBSYSTEM_NATIVE = 1, //Device drivers and native Windows processes
	IMAGE_SUBSYSTEM_WINDOWS_GUI = 2, //The Windows graphical user interface (GUI) subsystem
	IMAGE_SUBSYSTEM_WINDOWS_CUI = 3, //The Windows character subsystem
	IMAGE_SUBSYSTEM_OS2_CUI = 5, //The OS/2 character subsystem
	IMAGE_SUBSYSTEM_POSIX_CUI = 7, //The Posix character subsystem
	IMAGE_SUBSYSTEM_NATIVE_WINDOWS = 8, //Native Win9x driver
	IMAGE_SUBSYSTEM_WINDOWS_CE_GUI = 9, //Windows CE
	IMAGE_SUBSYSTEM_EFI_APPLICATION = 10, //An Extensible Firmware Interface (EFI) application
	IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER = 11, //An EFI driver with boot services
	IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER = 12, //An EFI driver with run-time services
	IMAGE_SUBSYSTEM_EFI_ROM = 13, //An EFI ROM image
	IMAGE_SUBSYSTEM_XBOX = 14, //XBOX
	IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION = 16, //Windows boot application.
    padding = 0xffff
}
pub enum optional_header_dll_characteristics{ // SHOULD BE u16
    IMAGE_DLLCHARACTERISTICS_RESERVED1 = 0x0001, //Reserved, must be zero.
    IMAGE_DLLCHARACTERISTICS_RESERVED2 = 0x0002, //Reserved, must be zero.
    IMAGE_DLLCHARACTERISTICS_RESERVED3 = 0x0004, //Reserved, must be zero.
    IMAGE_DLLCHARACTERISTICS_RESERVED4 = 0x0008, //Reserved, must be zero.
    IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA = 0x0020, //Image can handle a high entropy 64-bit virtual address space.
    IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE = 0x0040, //DLL can be relocated at load time.
    IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY = 0x0080, //Code Integrity checks are enforced.
    IMAGE_DLLCHARACTERISTICS_NX_COMPAT = 0x0100, //Image is NX compatible.
    IMAGE_DLLCHARACTERISTICS_NO_ISOLATION = 0x0200, //Isolation aware, but do not isolate the image.
    IMAGE_DLLCHARACTERISTICS_NO_SEH = 0x0400, //Does not use structured exception (SE) handling. No SE handler may be called in this image.
    IMAGE_DLLCHARACTERISTICS_NO_BIND = 0x0800, //Do not bind the image.
    IMAGE_DLLCHARACTERISTICS_APPCONTAINER = 0x1000, //Image must execute in an AppContainer.
    IMAGE_DLLCHARACTERISTICS_WDM_DRIVER = 0x2000, //A WDM driver.
    IMAGE_DLLCHARACTERISTICS_GUARD_CF = 0x4000, //Image supports Control Flow Guard.
    IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE = 0x8000, //Terminal Server aware.
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
pub struct IMAGE_OPTIONAL_HEADER_64 {
    pub magic: u16, // 0x10B = 32bit, 0x20B = 64bit 
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    //pub base_of_data: u32,
    pub image_base: u64, 
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
    pub subsystem: optional_header_windows_subsytem, // u16
    pub dll_characteristics: optional_header_dll_characteristics, // u16
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
} // 112 bytes

#[repr(C, packed)]
pub struct IMAGE_OPTIONAL_HEADER_RVA{ // supposedly these can be nullable?
    pub export_table:Option<IMAGE_DATA_DIRECTORY>, //The export table address and size. For more information see .edata Section (Image Only).
    pub import_table:Option<IMAGE_DATA_DIRECTORY>, //The import table address and size. For more information, see The .idata Section.
    pub resource_table:Option<IMAGE_DATA_DIRECTORY>, //The resource table address and size. For more information, see The .rsrc Section.
    pub exception_table:Option<IMAGE_DATA_DIRECTORY>, //The exception table address and size. For more information, see The .pdata Section.
    pub certificate_table:Option<IMAGE_DATA_DIRECTORY>, //The attribute certificate table address and size. For more information, see The Attribute Certificate Table (Image Only).
    pub base_relocation_table:Option<IMAGE_DATA_DIRECTORY>, //The base relocation table address and size. For more information, see The .reloc Section (Image Only).
    pub debug:Option<IMAGE_DATA_DIRECTORY>, //The debug data starting address and size. For more information, see The .debug Section.
    pub architecture:Option<IMAGE_DATA_DIRECTORY>, //Reserved, must be 0
    pub global_ptr:Option<IMAGE_DATA_DIRECTORY>, //The RVA of the value to be stored in the global pointer register. The size member of this structure must be set to zero.
    pub tls_table:Option<IMAGE_DATA_DIRECTORY>, //The thread local storage (TLS) table address and size. For more information, see The .tls Section.
    pub load_config_table:Option<IMAGE_DATA_DIRECTORY>, //The load configuration table address and size. For more information, see The Load Configuration Structure (Image Only).
    pub bound_import:Option<IMAGE_DATA_DIRECTORY>, //The bound import table address and size.
    pub iat:Option<IMAGE_DATA_DIRECTORY>, //The import address table address and size. For more information, see Import Address Table.
    pub delay_import_descriptor:Option<IMAGE_DATA_DIRECTORY>, //The delay import descriptor address and size. For more information, see Delay-Load Import Tables (Image Only).
    pub clr_runtime_header:Option<IMAGE_DATA_DIRECTORY>, //The CLR runtime header address and size. For more information, see The .cormeta Section (Object Only).
    pub reserved_must_be_zero:Option<IMAGE_DATA_DIRECTORY>, //
} // 128 bytes

#[repr(C, packed)]
pub struct IMAGE_DATA_DIRECTORY {
    pub virtual_address:u32,
    pub size:u32
} // 8 bytes

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