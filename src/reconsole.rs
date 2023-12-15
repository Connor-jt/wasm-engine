


// GLOBAL VARS

static mut PREV_ENTRIES:Vec<String> = vec![];
static mut LINE:Vec<char> = vec![]; // i hate you rust

const CHAR_WIDTH:u32 = 8;
const CHAR_HEIGHT:u32 = 16;

const a: &[u32] = &[8,9,10,11,23,27,38,44,54,60,70,76,87,91,102,103,104,105,106,107,108,];
const b: &[u32] = &[3,4,5,6,7,8,9,10,11,23,28,38,44,54,60,70,76,87,91,104,105,106,];
const c: &[u32] = &[8,9,10,23,27,38,44,54,60,70,76,86,92,103,107,];

const NULL: &[u32] = &[];

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
        
        // turn this into its own function so we can regnerate easier!!
        let char_data: &[u32];
        match char {
            'a' => char_data = a,
            'b' => char_data = b,
            'c' => char_data = c,
            _ => char_data = NULL,
        }

        for value in char_data.iter(){
            let offset_x:u32 = (value >> 4) & 0b111;
            let offset_y:u32 = value & 0b1111;

            let pixel_offset = (((x + offset_x)*4) + ((y + offset_y) * width*4)) as usize;
            screen[pixel_offset] = 255;
            screen[pixel_offset+1] = 255;
            screen[pixel_offset+2] = 255;
            screen[pixel_offset+3] = 255;
        }

        // dummy code for drawing character
        // for dummy_x in 0..CHAR_WIDTH{
        //     for dummy_y in 0..CHAR_HEIGHT{
        //         if (dummy_x + (dummy_y & 1)) % 2 == 1{
        //             let pixel_offset = (((x + dummy_x)*4) + ((y + dummy_y) * width*4)) as usize;
        //             screen[pixel_offset] = 255;
        //             screen[pixel_offset+1] = 255;
        //             screen[pixel_offset+2] = 255;
        //             screen[pixel_offset+3] = 255;
        //         }
        //     }
        // }


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