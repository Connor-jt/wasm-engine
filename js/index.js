
import("../pkg/index.js").catch(console.error);
import { redraw_canvas, key_input } from '../pkg/index_bg.js';


document.addEventListener('keydown', function(event) {
    key_input(event.key);
});
