use wasm_bindgen::prelude::*;
use web_sys::{console, ImageData};
use wasm_bindgen::Clamped;
use std::mem::MaybeUninit;

#[macro_use]
extern crate lazy_static;

mod reconsole;

// GLOBALS
static mut CANVAS: MaybeUninit<web_sys::HtmlCanvasElement> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<web_sys::CanvasRenderingContext2d> = MaybeUninit::uninit();
static mut requesting_screen_update:bool = true;


// INIT FUNCTION
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Hello world from rust!"));
    // init global variables
    let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
    let document: web_sys::Document = window.document().expect("should have a document on window");
    unsafe{
        CANVAS = MaybeUninit::new(document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap());
        CONTEXT = MaybeUninit::new(CANVAS.assume_init_ref().get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap());}
    Ok(())
}

// RENDER FUNCTION
#[wasm_bindgen]
pub fn redraw_canvas(){
    let canvas: &web_sys::HtmlCanvasElement;
    let context: &web_sys::CanvasRenderingContext2d;
    unsafe{ // verify both variables are initialized & assign
        if CANVAS.as_mut_ptr().is_null() || CONTEXT.as_mut_ptr().is_null(){
            return console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));}
        canvas = CANVAS.assume_init_ref(); context = CONTEXT.assume_init_ref();
    }

    let mut new_pixels: Vec<u8> = vec![0; (canvas.width() * canvas.height() * 4) as usize];
    // draw stuff
    unsafe {
        reconsole::draw(&mut new_pixels, canvas.width(), canvas.height());
    }

    let edited_img:ImageData = ImageData::new_with_u8_clamped_array(Clamped(&new_pixels[..]), canvas.width()).unwrap();
    let _ = context.put_image_data(&edited_img, 0.0, 0.0);
}

// INPUT HANDLER FUNCTION
#[wasm_bindgen]
pub unsafe fn key_input(input:&str){
    // check if this input wants us to restart/close or something?
    //console::log_1(&JsValue::from_str(format!("key pressed! {}", input).as_str()));

    // pass input to current process
    if reconsole::input(input){
        requesting_screen_update = true;
    }
}

// TICK INPUT FUNCTION ????
#[wasm_bindgen]
pub fn tick() -> bool{ // returns whether an update was performed or not
    unsafe {
        if requesting_screen_update{
            redraw_canvas();
            requesting_screen_update = false;
            return true;
    }}
    return false;
}




// WINDOW RESIZE FUNCTION 
#[wasm_bindgen]
pub unsafe fn window_resized(){
    requesting_screen_update = true;
    //redraw_canvas();
    // let canvas: &web_sys::HtmlCanvasElement;
    // unsafe{ // verify both variables are initialized & assign
    //     if CANVAS.as_mut_ptr().is_null(){return console::log_1(&JsValue::from_str("canvas/context uninitalized when drawing!!"));}
    //     canvas = CANVAS.assume_init_ref();
    // }

    // canvas.set_width(canvas)
}
#[wasm_bindgen]
pub unsafe fn file_added(file:String, data:Vec<u8>){
    let test = format!("{}, {}bytes", file, data.len());
    console::log_1(&JsValue::from_str(&test));
    
    reconsole::add_file(file, data);
    requesting_screen_update = true;
}