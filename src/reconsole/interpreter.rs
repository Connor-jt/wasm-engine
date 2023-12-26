

use std::fs::File;
use std::io::Read;
use std::thread::yield_now;
use rfd::AsyncFileDialog;
use futures::executor;

use executable::loaded_exe;

mod registers;
mod executable;

struct running_process{
    

}

pub struct loaded_file{
    pub name:String,
    pub data:Vec<u8>,
    //pub process:running_process // this contains all the data to run the process
}
impl Iterator for loaded_file{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = Some(self.data[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

// we have to return the error message or if there was no error then we return null somehow
pub unsafe fn run_file(file:&loaded_file) -> Option<String> {

    let var = executable::load_exe(&file.data);
    if var.is_err(){return Some(var.err().unwrap());}
    let unwrapped = var.unwrap();

    let poopy1 = unwrapped.header.optional_header.address_of_entry_point;
    let poopy2 = unwrapped.header.optional_header.base_of_code;
    let poopy3 = unwrapped.header.optional_header.size_of_code;
    let poopy4 = unwrapped.header.optional_header.image_base;
    return Some(format!("{} bytes, {} base address, {} code base, {} code size, {} image base",
        unwrapped.runtime_data.len(), 
        poopy1, poopy2, poopy3, poopy4
    ));

    // run code loop here
    loop{



        break;
    }
    return Some("program success".to_owned());
}


// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think