
use once_cell::sync::Lazy;

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
            y += 1;
        }
        if y + CHAR_HEIGHT >= height{
            break; // this shouldn't happen?
        }

        // dummy code for drawing character
        for dummy_x in 0..CHAR_WIDTH{
            for dummy_y in 0..CHAR_HEIGHT{
                if (dummy_x + (dummy_y & 1)) % 2 == 1{
                    let pixel_offset = ((x + dummy_x) + ((y + dummy_y) * width) * 4) as usize;
                    screen[pixel_offset] = 255;
                    screen[pixel_offset+1] = 255;
                    screen[pixel_offset+2] = 255;
                    screen[pixel_offset+3] = 255;
                }
            }
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