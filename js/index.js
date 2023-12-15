
import("../pkg/index.js").catch(console.error);
import { redraw_canvas, key_input, window_resized } from '../pkg/index_bg.js';


document.addEventListener('keydown', function(event) {
    key_input(event.key);
});

var canvas = document.getElementById('canvas');
window.addEventListener('resize', function(event) {
    // update canvas to be correct size
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    window_resized();
});