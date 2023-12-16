mod declarations;

// GLOBAL VARS

static mut PREV_ENTRIES:Vec<String> = vec![];
static mut LINE_POS:i32 = 0; // signed so we dont get any overflow errors
static mut LINE:Vec<char> = vec![]; // i hate you rust
struct line_struct{

}

const CHAR_WIDTH:u32 = 8;
const CHAR_HEIGHT:u32 = 16;

pub unsafe fn draw(screen:&mut Vec<u8>, width:u32, height:u32){
    let mut x:u32 = 0;
    let mut y:u32 = 0;
    // iterate characters in current line
    for char in LINE.iter(){

        if x + CHAR_WIDTH >= width{
            x = 0;
            y += CHAR_HEIGHT;
        }
        if y + CHAR_HEIGHT >= height{break;}
        
        // draw thingo
        for value in declarations::get_char_data(*char).iter(){
            let offset_x:u32 = (value >> 4) & 0b111;
            let offset_y:u32 = value & 0b1111;

            let pixel_offset = (((x + offset_x)*4) + ((y + offset_y) * width*4)) as usize;
            screen[pixel_offset] = 255;
            screen[pixel_offset+1] = 255;
            screen[pixel_offset+2] = 255;
            screen[pixel_offset+3] = 255;
        }


        // update draw position
        x += CHAR_WIDTH
    }
    // then we have to draw our line position indicator??

    // iterate characters in output?





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
            _ => return false
        }
        return true; // control key did something, screen needs update
    } 

    // otherwise its a regular key
    LINE.insert(verify_LINE_POS() as usize, input.chars().next().unwrap());
    LINE_POS += 1; //verify_LINE_POS(); // line wont exceed max from doing this, do not need to verify
    return true;
}
unsafe fn verify_LINE_POS() ->i32{
    if LINE_POS != 0{
        if LINE_POS < 0{LINE_POS = 0; }
        else if LINE_POS > LINE.len() as i32 {LINE_POS = (LINE.len() as i32);}}
    return LINE_POS;
}