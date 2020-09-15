import init, { Application, Point, Key } from './pkg/mandelbrot_wasm.js';

async function run() {
    const wasm = await init();
    const memory = wasm.memory;
    
    let app = Application.new();
    const width = app.width();
    const height = app.height();
   
    const canvas = document.getElementById("mandelbrot-canvas");
    canvas.height = height;
    canvas.width = width;

    const ctx = canvas.getContext('2d');
    const drawImage = () => {
        const imgPtr = app.image_buffer();
        const img_raw = new Uint8Array(memory.buffer, imgPtr, width * height * 4);
        const img = ctx.createImageData(width, height);
        for (let k=0; k<4*height*width; k++) {
            img.data[k] = img_raw[k];
        }
        ctx.putImageData(img, 0, 0);
    }
    
    app.update();
    drawImage();

    async function zoom(event, out) {
        const boundingRect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;
        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasBottom = (event.clientY - boundingRect.top) * scaleY;
        const point = Point.new(canvasLeft, canvasBottom);
        app.zoom(point, out);
    }

    canvas.addEventListener("click", event => {
        zoom(event, false);
        drawImage();
    });

    canvas.addEventListener("contextmenu", event => {
        zoom(event, true);
        drawImage();
        event.preventDefault();
        return false;
    });

    async function shift(direction) {
        app.shift(direction);
        drawImage();
    }

    const up = document.getElementsByClassName("up")[0];
    const down = document.getElementsByClassName("down")[0];
    const left = document.getElementsByClassName("left")[0];
    const right = document.getElementsByClassName("right")[0];
    up.onclick = () => { shift(Key.Up) };
    down.onclick = () => { shift(Key.Down) };
    left.onclick = () => { shift(Key.Left) };
    right.onclick = () => { shift(Key.Right) };

    const reset = document.getElementById("reset");
    reset.onclick = () => { app.reset(); drawImage(); };

    save = document.getElementById("save");
    save.addEventListener('click', function (e) {
        save.href = canvas.toDataURL('image/png');
    });

}

run();
