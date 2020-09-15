# mandelbrot-wasm
Explorable [Mandelbrot Fractal](https://en.wikipedia.org/wiki/Mandelbrot_set) - written in Rust, compiled to WASM

![mandelbrot-rs](https://github.com/christiankuhl/mandelbrot-wasm/raw/master/screenshot.png "mandelbrot-wasm")

## Installation and usage

```
git clone https://github.com/christiankuhl/mandelbrot-wasm.git
cd mandelbrot-wasm
python3 devserver.py
```
serves a barebones webpage `index.html` which loads the compiled WASM and necessary JS glue code.

## Example
The applet can be tested on [https://www.musicofreason,de/mandelbrot].