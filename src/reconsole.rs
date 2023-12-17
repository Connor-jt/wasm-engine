mod declarations;

// GLOBAL VARS

static mut PREV_ENTRIES:Vec<line_struct> = vec![];

static mut LINE_POS:i32 = 0; // signed so we dont get any overflow errors
static mut LINE:Vec<char> = vec![]; // i hate you rust
static mut LINE_CONTEXT:[char; 4] = ['c', 'm', 'd', '>']; 

struct line_struct{
    R:u8,
    G:u8,
    B:u8,
    Line:String
}

const CHAR_WIDTH:u32 = 8;
const CHAR_HEIGHT:u32 = 16;
pub unsafe fn draw_char(screen:&mut Vec<u8>, width:u32, height:u32, char:char, x:u32, y:u32, R:u8, G:u8, B:u8){
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

fn output(message:String, red:u8, green:u8, blue:u8){
    let test:line_struct = line_struct{ Line:message, R:red, G:green, B:blue};
    unsafe {PREV_ENTRIES.push(test);}
}

unsafe fn commit_line(){
    // convert chars into string
    let curr_line: String = LINE.iter().collect();
    LINE.clear(); // empty the line

    // then match string with possible commands?
    // for now we're going to pretend that every combination just returns an error

    let error_message = format!("error! cmd '{}' does not exist!", curr_line);
    output(error_message, 255,0,0);
    verify_LINE_POS();
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