use num::Complex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MAX_ITERATIONS: u32 = 512;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const START_RANGE: PlotRange = PlotRange { top_left: Complex {re: -2.0, im: 1.25},
                                           bottom_right: Complex {re: 1.0, im: -1.25}};
const ZOOM: f64 = 2.0;
const STEP_SIZE: f64 = 0.05;

fn escape_time(c: &Complex<f64>, settings: &ApplicationSettings) -> Option<f64> {
    if in_mandelbrot_set(&c) {
        return None;
    }
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..settings.max_iterations {
        z = z * z + c;
        if z.norm_sqr() > (1 << 16) as f64 {
            let shade = 1.0 - 0.01 * (z.norm_sqr().log2() / 2.0).log2();
            return Some((i as f64) + shade)
        }
        // return Some(500.0)
    }
    None
}

fn in_mandelbrot_set(c: &Complex<f64>) -> bool {
    (c - Complex::new(-1., 0.)).norm_sqr() < 0.0625 || {
    let z = c / c.norm_sqr().sqrt();
    c.norm_sqr() < (z / 2. - (z * z) / 4.).norm_sqr()
   }
}

#[wasm_bindgen]
pub enum Key {
    Up,
    Down,
    Left,
    Right
}

#[wasm_bindgen]
pub struct Point (f64, f64);

#[wasm_bindgen]
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point(x, y)
    }
}


#[wasm_bindgen]
pub struct ApplicationSettings {
    zoom: f64,
    max_iterations: u32,
}

#[wasm_bindgen]
pub struct Application {
   plot_range: PlotRange,
   settings: ApplicationSettings,
   buffer: Vec<u32>
}

#[wasm_bindgen]
impl Application {
    pub fn height(&self) -> u32 {
        HEIGHT.to_owned() as u32 
    }
    pub fn width(&self) -> u32 {
        WIDTH.to_owned() as u32
    }
    pub fn new() -> Application {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let settings = ApplicationSettings {zoom: ZOOM, max_iterations: MAX_ITERATIONS};
        Application { plot_range: START_RANGE,
                      settings: settings,
                      buffer: buffer }
    }
    pub fn update(&mut self) {
        for (index, value) in self.buffer.iter_mut().enumerate() {
            let z = self.plot_range.index_to_point(index);
            *value = match escape_time(&z, &self.settings) {
                Some(tau) => PALETTE[tau.floor() as usize % 1024],
                None => 255 << 24
            };
        }
    }
    pub fn reset(&mut self) {
        self.settings = ApplicationSettings {zoom: ZOOM, max_iterations: MAX_ITERATIONS};
        self.plot_range = START_RANGE;
        self.buffer = vec![0; WIDTH * HEIGHT];
        self.update();
    }
    pub fn zoom(&mut self, point: Point, out: bool) {
            self.plot_range.zoom(point, out, &mut self.settings);
            self.update();
    }
    pub fn shift(&mut self, direction: Key){
        self.plot_range.shift(direction);
        self.update();
    }
    pub fn image_buffer(&self) -> *const u32 {
            self.buffer.as_ptr()
    }
}

struct PlotRange {
    top_left: Complex<f64>,
    bottom_right: Complex<f64>
}

impl PlotRange {
    pub fn index_to_point(&self, index: usize) -> Complex<f64> {
        Complex {re: ((index % WIDTH) as f64) / (WIDTH as f64)
                        * self.width() + self.top_left.re,
                 im: (((index / WIDTH) as f64).floor()) / (HEIGHT as f64)
                         * self.height() + self.top_left.im}
    }
    pub fn zoom(&mut self, point: Point, out: bool, settings: &mut ApplicationSettings) {
        let h = self.height();
        let w = self.width();
        let mut z = settings.zoom;
        if out {
            z = 1.0 / z;
            settings.max_iterations -= 5;
        } else {
            settings.max_iterations += 5;
        }
        let mid_x = point.0 / (WIDTH as f64) * w + self.top_left.re;
        let mid_y = point.1 / (HEIGHT as f64) * h + self.top_left.im;
        self.top_left = Complex {re: mid_x - w / (2.0 * z),
                                 im: mid_y - h / (2.0 * z)};
        self.bottom_right = Complex {re: mid_x + w / (2.0 * z),
                                     im: mid_y + h / (2.0 * z)};
    }
    pub fn shift(&mut self, direction: Key) {
        let w = self.width() * STEP_SIZE;
        let delta = match direction {
            Key::Left => Complex {re: -w, im: 0.0},
            Key::Right => Complex {re: w, im: 0.0},
            Key::Up => Complex {re: 0.0, im: w},
            Key::Down => Complex {re: 0.0, im: -w},
        };
        self.top_left += delta;
        self.bottom_right += delta;
    }
    pub fn height(&self) -> f64 {
        self.bottom_right.im - self.top_left.im
    }
    pub fn width(&self) -> f64 {
        self.bottom_right.re - self.top_left.re
    }
}

const PALETTE: [u32; 1024] = [0xff640700, 0xff640700, 0xff650800, 0xff660800, 0xff670900, 0xff670a00, 0xff680a00, 0xff690b00, 
                                0xff6a0c00, 0xff6b0c00, 0xff6b0d00, 0xff6c0d00, 0xff6d0e00, 0xff6e0f00, 0xff6f0f00, 0xff6f1000, 
                                0xff701100, 0xff711100, 0xff721200, 0xff731200, 0xff731300, 0xff741400, 0xff751400, 0xff761500, 
                                0xff761600, 0xff771601, 0xff781701, 0xff791701, 0xff7a1801, 0xff7a1901, 0xff7b1901, 0xff7c1a01, 
                                0xff7d1b01, 0xff7d1b01, 0xff7e1c01, 0xff7f1c02, 0xff801d02, 0xff801e02, 0xff811e02, 0xff821f02, 
                                0xff832002, 0xff842002, 0xff842102, 0xff852102, 0xff862203, 0xff872303, 0xff872303, 0xff882403, 
                                0xff892503, 0xff8a2503, 0xff8a2603, 0xff8b2604, 0xff8c2704, 0xff8d2804, 0xff8d2804, 0xff8e2904, 
                                0xff8f2a04, 0xff8f2a05, 0xff902b05, 0xff912b05, 0xff922c05, 0xff922d05, 0xff932d05, 0xff942e06, 
                                0xff942e06, 0xff952f06, 0xff963006, 0xff973006, 0xff973107, 0xff983207, 0xff993207, 0xff993307, 
                                0xff9a3307, 0xff9b3408, 0xff9c3508, 0xff9c3508, 0xff9d3608, 0xff9e3608, 0xff9e3709, 0xff9f3809, 
                                0xffa03809, 0xffa03909, 0xffa13a09, 0xffa23a0a, 0xffa23b0a, 0xffa33b0a, 0xffa43c0a, 0xffa43d0b, 
                                0xffa53d0b, 0xffa63e0b, 0xffa63e0b, 0xffa73f0b, 0xffa7400c, 0xffa8400c, 0xffa9410c, 0xffa9410c, 
                                0xffaa420d, 0xffab430d, 0xffab430d, 0xffac440d, 0xffac440e, 0xffad450e, 0xffae460e, 0xffae460e, 
                                0xffaf470f, 0xffaf480f, 0xffb0480f, 0xffb1490f, 0xffb14910, 0xffb24a10, 0xffb24b10, 0xffb34b10, 
                                0xffb34c11, 0xffb44c11, 0xffb54d11, 0xffb54e12, 0xffb64e12, 0xffb64f12, 0xffb74f12, 0xffb75013, 
                                0xffb85113, 0xffb85113, 0xffb95213, 0xffb95214, 0xffba5314, 0xffba5414, 0xffbb5415, 0xffbb5515, 
                                0xffbc5515, 0xffbc5615, 0xffbd5716, 0xffbd5716, 0xffbe5816, 0xffbe5816, 0xffbf5917, 0xffbf5917, 
                                0xffc05a17, 0xffc05b18, 0xffc15b18, 0xffc15c18, 0xffc25c18, 0xffc25d19, 0xffc25e19, 0xffc35e19, 
                                0xffc35f1a, 0xffc45f1a, 0xffc4601a, 0xffc4611b, 0xffc5611b, 0xffc5621b, 0xffc6621b, 0xffc6631c, 
                                0xffc6641c, 0xffc7641c, 0xffc7651d, 0xffc8651d, 0xffc8661d, 0xffc8661d, 0xffc9671e, 0xffc9681e, 
                                0xffc9681e, 0xffca691f, 0xffca691f, 0xffca6a1f, 0xffcb6b20, 0xffcb6b20, 0xffcb6c20, 0xffcb6c20, 
                                0xffcc6d21, 0xffcc6e21, 0xffcc6e22, 0xffcd6f22, 0xffcd6f22, 0xffcd7023, 0xffce7123, 0xffce7124, 
                                0xffce7224, 0xffcf7224, 0xffcf7325, 0xffcf7425, 0xffcf7426, 0xffd07526, 0xffd07627, 0xffd07627, 
                                0xffd17728, 0xffd17828, 0xffd17829, 0xffd27929, 0xffd27a2a, 0xffd27a2a, 0xffd37b2b, 0xffd37c2c, 
                                0xffd37c2c, 0xffd37d2d, 0xffd47e2d, 0xffd47e2e, 0xffd47f2f, 0xffd5802f, 0xffd58030, 0xffd58131, 
                                0xffd68231, 0xffd68232, 0xffd68333, 0xffd68433, 0xffd78434, 0xffd78535, 0xffd78635, 0xffd88736, 
                                0xffd88737, 0xffd88838, 0xffd88938, 0xffd98939, 0xffd98a3a, 0xffd98b3b, 0xffda8c3b, 0xffda8c3c, 
                                0xffda8d3d, 0xffda8e3e, 0xffdb8e3e, 0xffdb8f3f, 0xffdb9040, 0xffdc9141, 0xffdc9142, 0xffdc9243, 
                                0xffdc9343, 0xffdd9444, 0xffdd9445, 0xffdd9546, 0xffde9647, 0xffde9648, 0xffde9749, 0xffde9849, 
                                0xffdf994a, 0xffdf994b, 0xffdf9a4c, 0xffe09b4d, 0xffe09c4e, 0xffe09c4f, 0xffe09d50, 0xffe19e51, 
                                0xffe19f52, 0xffe19f53, 0xffe1a053, 0xffe2a154, 0xffe2a255, 0xffe2a256, 0xffe3a357, 0xffe3a458, 
                                0xffe3a559, 0xffe3a55a, 0xffe4a65b, 0xffe4a75c, 0xffe4a75d, 0xffe4a85e, 0xffe5a95f, 0xffe5aa60, 
                                0xffe5aa61, 0xffe5ab62, 0xffe6ac63, 0xffe6ad64, 0xffe6ad65, 0xffe6ae66, 0xffe7af67, 0xffe7b068, 
                                0xffe7b069, 0xffe7b16a, 0xffe8b26b, 0xffe8b36c, 0xffe8b36d, 0xffe8b46e, 0xffe9b56f, 0xffe9b570, 
                                0xffe9b671, 0xffe9b772, 0xffeab873, 0xffeab874, 0xffeab975, 0xffeaba76, 0xffebbb77, 0xffebbb78, 
                                0xffebbc79, 0xffebbd7b, 0xffecbd7c, 0xffecbe7d, 0xffecbf7e, 0xffecc07f, 0xffecc080, 0xffedc181, 
                                0xffedc282, 0xffedc283, 0xffedc384, 0xffeec485, 0xffeec486, 0xffeec587, 0xffeec688, 0xffeec789, 
                                0xffefc78a, 0xffefc88b, 0xffefc98c, 0xffefc98d, 0xffefca8e, 0xfff0cb8f, 0xfff0cb90, 0xfff0cc92, 
                                0xfff0cd93, 0xfff1cd94, 0xfff1ce95, 0xfff1cf96, 0xfff1cf97, 0xfff1d098, 0xfff2d199, 0xfff2d19a, 
                                0xfff2d29b, 0xfff2d39c, 0xfff2d39d, 0xfff3d49e, 0xfff3d59f, 0xfff3d5a0, 0xfff3d6a1, 0xfff3d6a2, 
                                0xfff3d7a3, 0xfff4d8a4, 0xfff4d8a5, 0xfff4d9a6, 0xfff4daa7, 0xfff4daa8, 0xfff5dba9, 0xfff5dbaa, 
                                0xfff5dcab, 0xfff5ddac, 0xfff5ddad, 0xfff5deae, 0xfff6deaf, 0xfff6dfb0, 0xfff6e0b1, 0xfff6e0b2, 
                                0xfff6e1b3, 0xfff6e1b4, 0xfff7e2b5, 0xfff7e2b6, 0xfff7e3b6, 0xfff7e3b7, 0xfff7e4b8, 0xfff7e5b9, 
                                0xfff8e5ba, 0xfff8e6bb, 0xfff8e6bc, 0xfff8e7bd, 0xfff8e7be, 0xfff8e8bf, 0xfff8e8c0, 0xfff9e9c0, 
                                0xfff9e9c1, 0xfff9eac2, 0xfff9eac3, 0xfff9ebc4, 0xfff9ebc5, 0xfff9ecc6, 0xfffaecc6, 0xfffaedc7, 
                                0xfffaedc8, 0xfffaeec9, 0xfffaeeca, 0xfffaeecb, 0xfffaefcb, 0xfffaefcc, 0xfffbf0cd, 0xfffbf0ce, 
                                0xfffbf1ce, 0xfffbf1cf, 0xfffbf1d0, 0xfffbf2d1, 0xfffbf2d1, 0xfffbf3d2, 0xfffbf3d3, 0xfffcf3d4, 
                                0xfffcf4d4, 0xfffcf4d5, 0xfffcf5d6, 0xfffcf5d6, 0xfffcf5d7, 0xfffcf6d8, 0xfffcf6d8, 0xfffcf6d9, 
                                0xfffcf7da, 0xfffdf7da, 0xfffdf7db, 0xfffdf8dc, 0xfffdf8dc, 0xfffdf8dd, 0xfffdf8dd, 0xfffdf9de, 
                                0xfffdf9df, 0xfffdf9df, 0xfffdfae0, 0xfffdfae0, 0xfffdfae1, 0xfffdfae1, 0xfffdfbe2, 0xfffefbe2, 
                                0xfffefbe3, 0xfffefbe3, 0xfffefbe4, 0xfffefce4, 0xfffefce5, 0xfffefce5, 0xfffefce6, 0xfffefce6, 
                                0xfffefde6, 0xfffefde7, 0xfffefde7, 0xfffefde8, 0xfffefde8, 0xfffefde8, 0xfffefde9, 0xfffefee9, 
                                0xfffefee9, 0xfffefeea, 0xfffefeea, 0xfffefeea, 0xfffefeea, 0xfffefeeb, 0xfffefeeb, 0xfffefeeb, 
                                0xfffefeeb, 0xfffefeec, 0xfffefeec, 0xfffefeec, 0xfffefeec, 0xfffefeec, 0xfffefeec, 0xfffefeed, 
                                0xfffefeed, 0xfffefeed, 0xfffefeed, 0xfffefeed, 0xfffefeed, 0xfffefeed, 0xfffefeee, 0xfffdfeee, 
                                0xfffdfeee, 0xfffdfeee, 0xfffcfeee, 0xfffcfeee, 0xfffcfeee, 0xfffbfeef, 0xfffbfeef, 0xfffafeef, 
                                0xfffafeef, 0xfffafeef, 0xfff9fdef, 0xfff8fdef, 0xfff8fdef, 0xfff7fdf0, 0xfff7fdf0, 0xfff6fdf0, 
                                0xfff5fdf0, 0xfff5fdf0, 0xfff4fcf0, 0xfff3fcf0, 0xfff2fcf1, 0xfff2fcf1, 0xfff1fcf1, 0xfff0fcf1, 
                                0xffeffcf1, 0xffeefbf1, 0xffedfbf1, 0xffedfbf1, 0xffecfbf2, 0xffebfbf2, 0xffeafaf2, 0xffe9faf2, 
                                0xffe8faf2, 0xffe7faf2, 0xffe6faf2, 0xffe5f9f2, 0xffe4f9f3, 0xffe3f9f3, 0xffe1f9f3, 0xffe0f9f3, 
                                0xffdff8f3, 0xffdef8f3, 0xffddf8f3, 0xffdcf8f3, 0xffdaf7f3, 0xffd9f7f4, 0xffd8f7f4, 0xffd7f7f4, 
                                0xffd5f6f4, 0xffd4f6f4, 0xffd3f6f4, 0xffd2f6f4, 0xffd0f5f4, 0xffcff5f4, 0xffcef5f5, 0xffccf4f5, 
                                0xffcbf4f5, 0xffc9f4f5, 0xffc8f3f5, 0xffc7f3f5, 0xffc5f3f5, 0xffc4f3f5, 0xffc2f2f5, 0xffc1f2f6, 
                                0xffbff2f6, 0xffbef1f6, 0xffbcf1f6, 0xffbbf1f6, 0xffb9f0f6, 0xffb8f0f6, 0xffb6f0f6, 0xffb5eff6, 
                                0xffb3eff7, 0xffb2eff7, 0xffb0eef7, 0xffafeef7, 0xffadedf7, 0xffabedf7, 0xffaaedf7, 0xffa8ecf7, 
                                0xffa7ecf7, 0xffa5ecf7, 0xffa3ebf8, 0xffa2ebf8, 0xffa0ebf8, 0xff9feaf8, 0xff9deaf8, 0xff9be9f8, 
                                0xff9ae9f8, 0xff98e9f8, 0xff96e8f8, 0xff95e8f8, 0xff93e7f8, 0xff91e7f9, 0xff90e7f9, 0xff8ee6f9, 
                                0xff8ce6f9, 0xff8be5f9, 0xff89e5f9, 0xff87e4f9, 0xff86e4f9, 0xff84e4f9, 0xff82e3f9, 0xff81e3f9, 
                                0xff7fe2f9, 0xff7de2fa, 0xff7ce1fa, 0xff7ae1fa, 0xff78e1fa, 0xff77e0fa, 0xff75e0fa, 0xff73dffa, 
                                0xff72dffa, 0xff70defa, 0xff6edefa, 0xff6dddfa, 0xff6bddfa, 0xff69dcfb, 0xff68dcfb, 0xff66dcfb, 
                                0xff64dbfb, 0xff63dbfb, 0xff61dafb, 0xff5fdafb, 0xff5ed9fb, 0xff5cd9fb, 0xff5bd8fb, 0xff59d8fb, 
                                0xff57d7fb, 0xff56d7fb, 0xff54d6fb, 0xff53d6fc, 0xff51d5fc, 0xff4fd5fc, 0xff4ed4fc, 0xff4cd4fc, 
                                0xff4bd3fc, 0xff49d3fc, 0xff48d2fc, 0xff46d2fc, 0xff45d1fc, 0xff43d1fc, 0xff42d0fc, 0xff40d0fc, 
                                0xff3fcffc, 0xff3dcffc, 0xff3ccefc, 0xff3acefc, 0xff39cdfd, 0xff37cdfd, 0xff36ccfd, 0xff35ccfd, 
                                0xff33cbfd, 0xff32cbfd, 0xff30cafd, 0xff2fcafd, 0xff2ec9fd, 0xff2cc9fd, 0xff2bc8fd, 0xff2ac8fd, 
                                0xff29c7fd, 0xff27c7fd, 0xff26c6fd, 0xff25c6fd, 0xff24c5fd, 0xff22c5fd, 0xff21c4fd, 0xff20c4fd, 
                                0xff1fc3fd, 0xff1ec3fe, 0xff1dc2fe, 0xff1bc2fe, 0xff1ac1fe, 0xff19c1fe, 0xff18c0fe, 0xff17c0fe, 
                                0xff16bffe, 0xff15bffe, 0xff14befe, 0xff13befe, 0xff12bdfe, 0xff11bcfe, 0xff11bcfe, 0xff10bbfe, 
                                0xff0fbbfe, 0xff0ebafe, 0xff0dbafe, 0xff0cb9fe, 0xff0cb9fe, 0xff0bb8fe, 0xff0ab8fe, 0xff09b7fe, 
                                0xff09b7fe, 0xff08b6fe, 0xff07b6fe, 0xff07b5fe, 0xff06b5fe, 0xff06b4fe, 0xff05b4fe, 0xff04b3fe, 
                                0xff04b3fe, 0xff04b2fe, 0xff03b2fe, 0xff03b1fe, 0xff02b1fe, 0xff02b0fe, 0xff02b0fe, 0xff01affe, 
                                0xff01affe, 0xff01aefe, 0xff00aefe, 0xff00adfe, 0xff00acfe, 0xff00acfe, 0xff00abfe, 0xff00abfe, 
                                0xff00aafe, 0xff00aafe, 0xff00a9fe, 0xff00a9fe, 0xff00a8fe, 0xff00a8fe, 0xff00a7fe, 0xff00a7fe, 
                                0xff00a6fe, 0xff00a6fe, 0xff00a5fd, 0xff00a4fd, 0xff00a4fd, 0xff00a3fd, 0xff00a3fc, 0xff00a2fc, 
                                0xff00a1fc, 0xff00a1fb, 0xff00a0fb, 0xff009ffa, 0xff009ffa, 0xff009ef9, 0xff009df9, 0xff009df8, 
                                0xff009cf7, 0xff009bf7, 0xff009af6, 0xff009af5, 0xff0099f5, 0xff0098f4, 0xff0097f3, 0xff0096f2, 
                                0xff0096f2, 0xff0095f1, 0xff0094f0, 0xff0093ef, 0xff0092ee, 0xff0092ed, 0xff0091ec, 0xff0090eb, 
                                0xff008fea, 0xff008ee9, 0xff008de8, 0xff008ce7, 0xff008ce6, 0xff008be5, 0xff008ae4, 0xff0089e3, 
                                0xff0088e2, 0xff0087e1, 0xff0086df, 0xff0085de, 0xff0084dd, 0xff0083dc, 0xff0083da, 0xff0082d9, 
                                0xff0081d8, 0xff0080d7, 0xff007fd5, 0xff007ed4, 0xff007dd3, 0xff007cd1, 0xff007bd0, 0xff007acf, 
                                0xff0079cd, 0xff0078cc, 0xff0077ca, 0xff0076c9, 0xff0075c7, 0xff0074c6, 0xff0073c4, 0xff0072c3, 
                                0xff0071c1, 0xff0070c0, 0xff006fbe, 0xff006ebd, 0xff006dbb, 0xff006cba, 0xff006bb8, 0xff006ab7, 
                                0xff0069b5, 0xff0068b3, 0xff0067b2, 0xff0066b0, 0xff0065af, 0xff0064ad, 0xff0063ab, 0xff0062aa, 
                                0xff0061a8, 0xff005fa6, 0xff005ea5, 0xff005da3, 0xff005ca1, 0xff005ba0, 0xff005a9e, 0xff00599c, 
                                0xff00589b, 0xff005799, 0xff005697, 0xff005595, 0xff005494, 0xff005392, 0xff005290, 0xff00518f, 
                                0xff00508d, 0xff004f8b, 0xff004e89, 0xff004d88, 0xff004c86, 0xff004b84, 0xff004a82, 0xff004981, 
                                0xff00487f, 0xff00477d, 0xff00467c, 0xff00457a, 0xff004478, 0xff004376, 0xff004275, 0xff004173, 
                                0xff004071, 0xff003f6f, 0xff003e6e, 0xff003d6c, 0xff003c6a, 0xff003b69, 0xff003a67, 0xff003965, 
                                0xff003863, 0xff003762, 0xff003660, 0xff00355e, 0xff00345d, 0xff00335b, 0xff003259, 0xff003158, 
                                0xff003056, 0xff002f54, 0xff002e53, 0xff002d51, 0xff002d4f, 0xff002c4e, 0xff002b4c, 0xff002a4b, 
                                0xff002949, 0xff002847, 0xff002746, 0xff002644, 0xff002543, 0xff002541, 0xff002440, 0xff00233e, 
                                0xff00223d, 0xff00213b, 0xff00203a, 0xff002038, 0xff001f37, 0xff001e35, 0xff001d34, 0xff001c32, 
                                0xff001c31, 0xff001b2f, 0xff001a2e, 0xff00192d, 0xff00192b, 0xff00182a, 0xff001729, 0xff001727, 
                                0xff001626, 0xff001525, 0xff001524, 0xff001422, 0xff001321, 0xff001320, 0xff00121f, 0xff00111d, 
                                0xff00111c, 0xff00101b, 0xff000f1a, 0xff000f19, 0xff000e18, 0xff000e17, 0xff000d16, 0xff000d15, 
                                0xff000c14, 0xff000c13, 0xff000b12, 0xff000b11, 0xff000a10, 0xff000a0f, 0xff00090e, 0xff00090d, 
                                0xff00080c, 0xff00080c, 0xff00070b, 0xff00070a, 0xff000709, 0xff000609, 0xff000608, 0xff000607, 
                                0xff000507, 0xff000506, 0xff000505, 0xff000405, 0xff000404, 0xff000404, 0xff000403, 0xff000303, 
                                0xff000302, 0xff000302, 0xff000302, 0xff000201, 0xff000201, 0xff000201, 0xff000201, 0xff000200, 
                                0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 
                                0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 0xff000200, 
                                0xff000200, 0xff000200, 0xff010200, 0xff010200, 0xff010200, 0xff010200, 0xff010200, 0xff020200, 
                                0xff020200, 0xff020200, 0xff020200, 0xff030200, 0xff030200, 0xff030200, 0xff040200, 0xff040200, 
                                0xff040200, 0xff050200, 0xff050200, 0xff050200, 0xff060200, 0xff060200, 0xff070200, 0xff070200, 
                                0xff070200, 0xff080200, 0xff080200, 0xff090200, 0xff090200, 0xff0a0200, 0xff0a0200, 0xff0b0200, 
                                0xff0b0200, 0xff0c0200, 0xff0c0200, 0xff0d0200, 0xff0d0200, 0xff0e0200, 0xff0f0200, 0xff0f0200, 
                                0xff100200, 0xff100200, 0xff110200, 0xff120200, 0xff120200, 0xff130200, 0xff140200, 0xff140200, 
                                0xff150200, 0xff160200, 0xff160200, 0xff170200, 0xff180200, 0xff180200, 0xff190200, 0xff1a0200, 
                                0xff1b0200, 0xff1b0200, 0xff1c0200, 0xff1d0200, 0xff1e0200, 0xff1e0200, 0xff1f0200, 0xff200200, 
                                0xff210200, 0xff220200, 0xff220200, 0xff230200, 0xff240200, 0xff250200, 0xff260200, 0xff260200, 
                                0xff270200, 0xff280200, 0xff290200, 0xff2a0200, 0xff2b0300, 0xff2c0300, 0xff2c0300, 0xff2d0300, 
                                0xff2e0300, 0xff2f0300, 0xff300300, 0xff310300, 0xff320300, 0xff330300, 0xff340300, 0xff350300, 
                                0xff350300, 0xff360300, 0xff370300, 0xff380300, 0xff390300, 0xff3a0300, 0xff3b0300, 0xff3c0300, 
                                0xff3d0300, 0xff3e0300, 0xff3f0400, 0xff400400, 0xff410400, 0xff420400, 0xff430400, 0xff440400, 
                                0xff440400, 0xff450400, 0xff460400, 0xff470400, 0xff480400, 0xff490400, 0xff4a0400, 0xff4b0400, 
                                0xff4c0400, 0xff4d0400, 0xff4e0500, 0xff4f0500, 0xff500500, 0xff510500, 0xff520500, 0xff530500, 
                                0xff540500, 0xff550500, 0xff560500, 0xff570500, 0xff580500, 0xff590500, 0xff5a0600, 0xff5b0600, 
                                0xff5c0600, 0xff5d0600, 0xff5e0600, 0xff5f0600, 0xff600600, 0xff610600, 0xff620600, 0xff630600, 
                                ];
