mod declarations;

// GLOBAL VARS

static mut PREV_ENTRIES:Vec<String> = vec![];
static mut LINE:Vec<char> = vec![]; // i hate you rust

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
        if y + CHAR_HEIGHT >= height{
            break; // this shouldn't happen?
        }
        
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

    // iterate characters in output?





}


pub fn input(input:&str) -> bool{
    if input.len() > 1 {return false;} // then its a control key

    unsafe{LINE.push(input.chars().next().unwrap());}
    return true;
}