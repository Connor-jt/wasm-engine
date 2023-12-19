
import("../pkg/index.js").catch(console.error);
import { redraw_canvas, key_input, tick, window_resized, file_added } from '../pkg/index_bg.js';


document.addEventListener('keydown', function(event) {
    key_input(event.key);
});

var canvas = document.getElementById('canvas');
window.addEventListener('resize', resize);
function resize(){
    // update canvas to be correct size
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    try{window_resized();} catch(error) {} // again, we load too soon so this doesn't call to thingo
}

var time_span = document.getElementById('time_span');
function animate() {
    requestAnimationFrame(animate);
    var start = performance.now();
    try{ // wasm isn't defined??? somehow we're loading too soon even though the module SHOULD be loaded already??? 
        if (tick() == true){
            var end = performance.now();
            time_span.innerText = (end - start).toFixed().toString() + " ms";
        }
    } catch(ex){}
}

//window.onload = function() {
window.addEventListener('load', function() {
    resize();
    animate();
});
  



var files_box = document.getElementById('file');
files_box.onchange = e => { 
    let file = e.target.files[0]; 
    if (file == null){ return;} // dont do anything when we clear the file
    console.log(file); 
    files_box.value = null; // then clear the value because we might want to upload the same file twice
    (async () => {
        let test = await file.arrayBuffer();
        console.log(test); 
        file_added(file.name, new Uint8Array(test));
    })();
};
