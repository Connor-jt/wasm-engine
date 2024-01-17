mod declarations;
mod interpreter;
use crate::reconsole::interpreter::loaded_file;
use crate::reconsole::interpreter::running_process;

// GLOBAL VARS
static mut AVAILABLE_FILES:Vec<loaded_file> = vec![];
static mut AVAILABLE_PROCESSES:Vec<running_process> = vec![];

static mut PREV_ENTRIES:Vec<line_struct> = vec![];

static mut LINE_POS:i32 = 0; // signed so we dont get any overflow errors
static mut LINE:Vec<char> = vec![]; // i hate you rust
static mut LINE_CONTEXT:[char; 4] = ['c', 'm', 'd', '>']; 

static mut input_blocked:bool = false;

struct line_struct{
    R:u8,
    G:u8,
    B:u8,
    Line:String
}
unsafe fn file_from_available(file:&str) -> Option<&loaded_file>{
    for loaded in AVAILABLE_FILES.iter(){
        if loaded.name == file{
            return Some(loaded);
    }}
    return None;
}
unsafe fn verify_filename(requested_name:String) -> String{ // this just gives the file a new name if we already have a file with that name
    let mut curr_name = requested_name.clone();
    let mut iteration_count = 0;
    while true {
        for file in AVAILABLE_FILES.iter(){
            if file.name == curr_name{
                iteration_count += 1;
                curr_name = format!("{}{}", requested_name, format!("_{}", iteration_count)); // bruh just lemme += strings for the love of god
                continue;
        }}
        break; // name not found, clear to use current name
    }
    return curr_name;
}   
fn filesize_to_string(size: usize) -> String{
    if size < 1000{ return format!("{}b", size);} 
    else if size < 1000000{return format!("{}kb", size/1000);}
    else if size < 1000000000{return format!("{}mb", size/1000000);}
    else {return format!("{}gb", size/1000000000);}
}

const CHAR_WIDTH:u32 = 8;
const CHAR_HEIGHT:u32 = 16;
unsafe fn draw_char(screen:&mut Vec<u8>, width:u32, height:u32, char:char, x:u32, y:u32, R:u8, G:u8, B:u8){
    // do a check here to see if base coords are out of bounds, for small perf increase?
    for value in declarations::get_char_data(char).iter(){
        let offset_x:u32 = ((value >> 4) & 0b111) + x;
        let offset_y:u32 = (value & 0b1111) + y;
        if offset_x >= width || offset_y >= height{continue;} // verify this pixel is on the thingo

        let pixel_offset = ((offset_x*4) + (offset_y*width*4)) as usize;
        screen[pixel_offset] = R;
        screen[pixel_offset+1] = G;
        screen[pixel_offset+2] = B;
        screen[pixel_offset+3] = 255;
    }
}

pub unsafe fn draw(screen:&mut Vec<u8>, width:u32, height:u32){
    let mut y:u32 = 0;
    // first we need to iterate previous lines
    for string in PREV_ENTRIES.iter(){
        let mut x:u32 = 0;
        for char in string.Line.chars(){
            draw_char(screen, width, height, char, x,y, string.R,string.G,string.B);
            x += CHAR_WIDTH;
        }
        y += CHAR_HEIGHT;
    }
    if input_blocked {return} // dont render line if input is currently blocked

    let mut x:u32 = 0;
    // draw context to line
    for char in LINE_CONTEXT.iter(){
        draw_char(screen, width, height, *char, x,y, 215,215,215);
        x += CHAR_WIDTH;
    }
    let post_context_x = x;

    // iterate characters in current line
    for char in LINE.iter(){
        // draw thingo
        draw_char(screen, width, height, *char, x,y, 215,215,215);
        // update draw position
        x += CHAR_WIDTH;
    }
    // then we have to draw our line position indicator (doing it this way does not allow us to have a command span multiple lines)
    let indi_x = (verify_LINE_POS() as u32 * CHAR_WIDTH) + post_context_x;
    draw_char(screen, width, height, 'A', indi_x,y, 255,255,255); // this draws the line indicator
}
pub unsafe fn add_file(input:String, data:Vec<u8>){
    let verified_name = verify_filename(input);
    AVAILABLE_FILES.push(loaded_file{name: verified_name.clone(), data: data});
    output(format!("file '{}' is now accessible", verified_name), 0,255,0);
}

pub unsafe fn input(input:&str) -> bool{
    if input.len() > 1 { // then its a control key
        match input{
            "Backspace" => {
                let index = (verify_LINE_POS()-1) as usize;
                if index >= 0 && LINE.len() > index {
                    LINE.remove(index);
                    LINE_POS -= 1; verify_LINE_POS(); // subtract and verify
            }},
            "Delete" => {
                let index = (verify_LINE_POS()) as usize;
                if LINE.len() > index {
                    LINE.remove(index); // line does not change, do not verify pos
            }},
            "ArrowLeft" => {
                LINE_POS -= 1; verify_LINE_POS(); 
            },
            "ArrowRight" => {
                LINE_POS += 1; verify_LINE_POS(); 
            },
            "Enter" => {
                commit_line(); 
            },
            _ => return false
        } // 0 g1 2
        return true; // control key did something, screen needs update
    } 
    // otherwise its a regular key
    LINE.insert(verify_LINE_POS() as usize, input.chars().next().unwrap().to_ascii_lowercase());
    LINE_POS += 1; //verify_LINE_POS(); // line wont exceed max from doing this, do not need to verify
    return true;
}

unsafe fn verify_LINE_POS() ->i32{
    if LINE_POS != 0{
        if LINE_POS < 0{LINE_POS = 0; }
        else if LINE_POS > LINE.len() as i32 {LINE_POS = (LINE.len() as i32);}}
    return LINE_POS;
}


fn output(message:String, red:u8, green:u8, blue:u8){
    let test:line_struct = line_struct{ Line:message, R:red, G:green, B:blue};
    unsafe {PREV_ENTRIES.push(test);}
}

unsafe fn commit_line(){
    if input_blocked{return;} // cant enter if input is blocked

    // convert chars into string
    let curr_line: String = LINE.iter().collect();
    LINE.clear(); // empty the line
    verify_LINE_POS();
    

    // then match string with possible commands?
    // for now we're going to pretend that every combination just returns an error

    let prev_line:String = LINE_CONTEXT.iter().collect();
    output(prev_line + &curr_line, 160,160,160);

    process_command(curr_line);
}

unsafe fn process_command(line:String){
    // break line down into words + get the first word which will be the command
    let args:Vec<&str> = line.split(" ").collect();
    if args.len() == 0{ return;} // return if empty
    input_blocked = true;

    match args[0]{
        "hello" => {
            output("hello world!!".to_owned(), 0,255,0);
        },
        "load"  => 'test: { // filename
            // validate args
            if args.len() < 2{output("too few arguments!".to_owned(), 255,0,0); break 'test;}
            let target_file = file_from_available(args[1]);
            if target_file.is_none(){
                output(format!("error! file '{}' is not available!", args[1]), 255,0,0);break 'test;}
            //
            let file_output = target_file.unwrap().load_file();
            if file_output.is_err(){
                output(format!("file error: {}", file_output.err().unwrap()), 255,0,0);break 'test;}
            let mut process = file_output.unwrap();
            output(format!("process {} id:{} is now operational", process.name, process.id), 0,255,0);
            // load process into running process list
            AVAILABLE_PROCESSES.push(process);
        },
        "run"  => 'test1: { // id, instruction count [default:1]
            // validate args
            if args.len() < 2{output("too few arguments!".to_owned(), 255,0,0); break 'test1;}
            
            let target_proc = get_proc(args[1]);
            if target_proc.is_none(){
                output(format!("process by id:{} was not found!", args[1]), 255,0,0); break 'test1;}
            let process:&mut running_process = target_proc.unwrap();

            let mut instruction_count:u32 = 1;
            if args.len() > 2{
                let instr_cnt = args[2].parse();
                if instr_cnt.is_err(){
                    output(format!("instructions count:{} was not valid!", args[2]), 255,0,0); break 'test1;}
                instruction_count = instr_cnt.unwrap();}
            
            // then  we just run the instructions
            for i in 0..instruction_count{
                //process.curr_address = 0; 
                let test = process.run_single();
                if test.is_none(){
                    output("failed instruction".to_owned(), 255,0,0);break 'test1;}
                output(test.unwrap().to_lowercase(), 200, 200,200);
            }
        }
        "rip" => 'test: { // id, new_rip_address
            if args.len() < 3{output("too few arguments!".to_owned(), 255,0,0); break 'test;}

            // verify id
            let target_proc = get_proc(args[1]);
            if target_proc.is_none(){
                output(format!("process by id:{} was not found!", args[1]), 255,0,0); 
                break 'test;}
            let process:&mut running_process = target_proc.unwrap();

            // verify new_rip_address
            // if starts with 0x then its hex, otherwise a number
            let offset_result:Result<u64, std::num::ParseIntError>;
            if args[2].starts_with("0x"){
                  offset_result = u64::from_str_radix(&args[2].trim_start_matches("0x"), 16);} 
            else{ offset_result = args[2].parse::<u64>();}
            if offset_result.is_err(){
                output(format!("arg '{}' was not a valid address!", args[2]), 255,0,0); break 'test;}
            
            process.curr_address = offset_result.unwrap();
            output(format!("process id:{} address updated to: 0x{:x}",process.id, process.curr_address), 0,255,0); break 'test;
        }
        "offs" => 'test: { // id, rip_offset
            if args.len() < 3{output("too few arguments!".to_owned(), 255,0,0); break 'test;}

            // verify id
            let target_proc = get_proc(args[1]);
            if target_proc.is_none(){
                output(format!("process by id:{} was not found!", args[1]), 255,0,0); 
                break 'test;}
            let process:&mut running_process = target_proc.unwrap();

            // verify offset value
            // if starts with 0x then its hex, otherwise a number
            let offset_result:Result<i64, std::num::ParseIntError>;
            let mut is_negative = false;
            if args[2].starts_with("0x"){
                  offset_result = i64::from_str_radix(&args[2].trim_start_matches("0x"), 16);} 
            else if args[2].starts_with("-0x"){ // as we support negative offsets
                  offset_result = i64::from_str_radix(&args[2].trim_start_matches("-0x"), 16);
                  is_negative = true;}
            else{ offset_result = args[2].parse::<i64>();} // else regular number
            if offset_result.is_err(){
                output(format!("arg '{}' was not a valid address!", args[2]), 255,0,0); break 'test;}

            let mut target_offset = offset_result.unwrap();
            if is_negative {target_offset *= -1;}
            // this is so cursed & epic & bad
            if target_offset < 0 {process.curr_address -= (target_offset.abs() as u64);}
            else {process.curr_address += target_offset as u64;}
            output(format!("process id:{} address updated to: 0x{:x}",process.id, process.curr_address), 0,255,0); break 'test;
        }
        "read" => 'test:{ // address, count
            // read bytes at address for x amount
            // get address 
            let offset_result:Result<u64, std::num::ParseIntError>;
            if args[1].starts_with("0x"){
                  offset_result = u64::from_str_radix(&args[1].trim_start_matches("0x"), 16);} 
            else{ offset_result = args[1].parse::<u64>();}
            if offset_result.is_err(){
                output(format!("arg '{}' was not a valid address!", args[1]), 255,0,0); break 'test;}
            let offset = offset_result.unwrap();
            // get count
            let count_result = args[2].parse::<u64>();
            if count_result.is_err(){
                output(format!("arg '{}' was not a valid count!", args[2]), 255,0,0); break 'test;}
            let count = count_result.unwrap();

            let mut con_output = "".to_owned();
            for i in 0..count{
                con_output += &format!("{:0>2} ", format!("{:x}", *((offset+i) as *const u8)));}
            output(con_output, 200, 200,200);
        }
        "dir" => {
            for file in AVAILABLE_FILES.iter(){
                output(format!("{} {}", file.name.to_owned(), filesize_to_string(file.data.len())), 200, 200,200);
            }
        },
        "proc" => {
            for proc in AVAILABLE_PROCESSES.iter(){
                output(format!("id: {}, {}, rip: 0x{:x}", proc.id, proc.name, proc.curr_address), 200, 200,200);
            }
        },
        "clear" => {
            unsafe {PREV_ENTRIES.clear();}
        },
        _ => {
            output(format!("error! cmd '{}' does not exist!", line), 255,0,0);
        }
    }


    input_blocked = false;
}
unsafe fn get_proc(id:&str) -> Option<&'static mut running_process>{
    for proc in AVAILABLE_PROCESSES.iter_mut(){
        if proc.id.to_string() == id{
            return Some(proc);}}
    return None;
}