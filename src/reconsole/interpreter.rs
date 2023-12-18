

use std::fs::File;
use std::io::Read;
use rfd::AsyncFileDialog;
use futures::executor;

static mut loaded_exe: Vec<u8> = vec![];

// we have to return the error message or if there was no error then we return null somehow
pub unsafe fn run_file() -> Option<String> {
    
    let future = async {load_file().await};
    let file_output = executor::block_on(future);

    //if file_output.is_none(){return Some("failed to open file".to_owned())}



    return Some("success ??".to_owned());
}
async unsafe fn load_file() -> Option<Vec<u8>>{
    let file = AsyncFileDialog::new().add_filter("executable", &["exe", "dll"]).set_directory("/").pick_file().await;
    if file.is_none(){ return None}; 
    let data = file.unwrap().read().await;
    return Some(data);
}

// have the fun ction here that runs the code, we have to run all of the code in a single go, idk how programs can run over the span of more than 1 tick

// we will have to write our own functions for a lot of the built in windows functions i think

// also we need to get the entire list of all aseembly x64 instructions and build another layer of interpreter i think