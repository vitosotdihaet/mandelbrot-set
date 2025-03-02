use std::time::UNIX_EPOCH;

use num::{complex::Complex64 as Complex, pow::Pow};
use sdl3::{event::Event, keyboard::Keycode, pixels::Color};

const BASE_COLOR: Color = Color {
    r: 255,
    g: 127,
    b: 0,
    a: 255,
};

const Z_BOUND: f64 = 2.;
const MAX_ITERATIONS: u64 = 50;

const ZOOM_FACTOR: f64 = 0.95;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 600;

/// in frame from `IN_FRAME_CENTER_X - IN_FRAME_X` to `IN_FRAME_CENTER_X + IN_FRAME_X`
static mut IN_FRAME_CENTER_X: f64 = 0.;
static mut IN_FRAME_PAD_X: f64 = 2.;

static mut IN_FRAME_CENTER_Y: f64 = 0.;

static mut TEMP1: Vec<Vec<Color>> = vec![];
static mut TEMP2: Vec<Vec<Color>> = vec![];
static mut TEMP3: Vec<Vec<Color>> = vec![];
static mut TEMP4: Vec<Vec<Color>> = vec![];

static mut M: Vec<Vec<Color>> = vec![];

fn f(c: Complex, z: Complex, p: f64) -> Complex {
    z.pow(p) + c
}

fn mandelbrot(x: f64, y: f64, p: f64, iters: u64) -> Color {
    let mut z = Complex { re: 0., im: 0. };
    let c = Complex { re: x, im: y };

    let mut t = 0.;

    for i in 0..iters {
        if z.norm() > Z_BOUND {
            t = i as f64 / MAX_ITERATIONS as f64;
            break;
        }

        z = f(c, z, p);
    }

    Color {
        r: (BASE_COLOR.r as f64 * t) as u8,
        g: (BASE_COLOR.g as f64 * t) as u8,
        b: (BASE_COLOR.b as f64 * t) as u8,
        a: 255,
    }
}

fn setup_vecs() {
    #[allow(static_mut_refs)]
    unsafe {
        M.resize(HEIGHT as usize, Vec::with_capacity(WIDTH as usize));
        for t in &mut M {
            t.resize(WIDTH as usize, Color::BLACK);
        }

        TEMP1.resize(
            (HEIGHT / 2) as usize,
            Vec::with_capacity((WIDTH / 2) as usize),
        );
        for t in &mut TEMP1 {
            t.resize((WIDTH / 2) as usize, Color::BLACK);
        }

        TEMP2.resize(
            (HEIGHT / 2) as usize,
            Vec::with_capacity((WIDTH - WIDTH / 2) as usize),
        );
        for t in &mut TEMP2 {
            t.resize((WIDTH - WIDTH / 2) as usize, Color::BLACK);
        }

        TEMP3.resize(
            (HEIGHT - HEIGHT / 2) as usize,
            Vec::with_capacity((WIDTH / 2) as usize),
        );
        for t in &mut TEMP3 {
            t.resize((WIDTH / 2) as usize, Color::BLACK);
        }

        TEMP4.resize(
            (HEIGHT - HEIGHT / 2) as usize,
            Vec::with_capacity((WIDTH - WIDTH / 2) as usize),
        );
        for t in &mut TEMP4 {
            t.resize((WIDTH - WIDTH / 2) as usize, Color::BLACK);
        }
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("mandelbrot-set", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    setup_vecs();

    let ar = WIDTH as f64 / HEIGHT as f64;

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => unsafe { IN_FRAME_CENTER_X -= IN_FRAME_PAD_X / 100. },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => unsafe { IN_FRAME_CENTER_X += IN_FRAME_PAD_X / 100. },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => unsafe { IN_FRAME_CENTER_Y += IN_FRAME_PAD_X / 100. },
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => unsafe { IN_FRAME_CENTER_Y -= IN_FRAME_PAD_X / 100. },
                Event::KeyDown {
                    keycode: Some(Keycode::Equals),
                    ..
                } => unsafe { IN_FRAME_PAD_X *= ZOOM_FACTOR },
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => unsafe { IN_FRAME_PAD_X /= ZOOM_FACTOR },
                _ => {}
            }
        }

        let p = 2.;

        // uncomment for FUN
        // let p = (std::time::SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs_f64()
        //     / 10.)
        //     .sin()
        //     / 2.
        //     + 2.5;

        let t1 = std::thread::spawn(move || {
            for y in 0..HEIGHT / 2 {
                for x in 0..WIDTH / 2 {
                    unsafe {
                        TEMP1[y as usize][x as usize] = mandelbrot(
                            (x as f64 / WIDTH as f64 - 0.5) * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_X,
                            (y as f64 / HEIGHT as f64 - 0.5) / ar * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_Y,
                            p,
                            MAX_ITERATIONS as u64,
                        );
                    }
                }
            }

            for y in 0..HEIGHT / 2 {
                for x in 0..WIDTH / 2 {
                    unsafe {
                        M[y as usize][x as usize] = TEMP1[y as usize][x as usize];
                    }
                }
            }
        });

        let t2 = std::thread::spawn(move || {
            for y in 0..HEIGHT / 2 {
                for x in WIDTH / 2..WIDTH {
                    unsafe {
                        TEMP2[y as usize][(x - WIDTH / 2) as usize] = mandelbrot(
                            (x as f64 / WIDTH as f64 - 0.5) * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_X,
                            (y as f64 / HEIGHT as f64 - 0.5) / ar * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_Y,
                            p,
                            MAX_ITERATIONS as u64,
                        );
                    }
                }
            }

            for y in 0..HEIGHT / 2 {
                for x in WIDTH / 2..WIDTH {
                    unsafe {
                        M[y as usize][x as usize] =
                            TEMP2[y as usize][x as usize - (WIDTH / 2) as usize];
                    }
                }
            }
        });

        let t3 = std::thread::spawn(move || {
            for y in HEIGHT / 2..HEIGHT {
                for x in 0..WIDTH / 2 {
                    unsafe {
                        TEMP3[y as usize - (HEIGHT / 2) as usize][x as usize] = mandelbrot(
                            (x as f64 / WIDTH as f64 - 0.5) * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_X,
                            (y as f64 / HEIGHT as f64 - 0.5) / ar * IN_FRAME_PAD_X * 2.
                                + IN_FRAME_CENTER_Y,
                            p,
                            MAX_ITERATIONS as u64,
                        );
                    }
                }
            }

            for y in HEIGHT / 2..HEIGHT {
                for x in 0..WIDTH / 2 {
                    unsafe {
                        M[y as usize][x as usize] =
                            TEMP3[y as usize - (HEIGHT / 2) as usize][x as usize];
                    }
                }
            }
        });

        let t4 = std::thread::spawn(move || {
            for y in HEIGHT / 2..HEIGHT {
                for x in WIDTH / 2..WIDTH {
                    unsafe {
                        TEMP4[y as usize - (HEIGHT / 2) as usize][(x - WIDTH / 2) as usize] =
                            mandelbrot(
                                (x as f64 / WIDTH as f64 - 0.5) * IN_FRAME_PAD_X * 2.
                                    + IN_FRAME_CENTER_X,
                                (y as f64 / HEIGHT as f64 - 0.5) / ar * IN_FRAME_PAD_X * 2.
                                    + IN_FRAME_CENTER_Y,
                                p,
                                MAX_ITERATIONS as u64,
                            );
                    }
                }
            }

            for y in HEIGHT / 2..HEIGHT {
                for x in WIDTH / 2..WIDTH {
                    unsafe {
                        M[y as usize][x as usize] = TEMP4[y as usize - (HEIGHT / 2) as usize]
                            [x as usize - (WIDTH / 2) as usize];
                    }
                }
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        t4.join().unwrap();

        {
            let mut surface = window.surface(&event_pump).unwrap();
            let pf = surface.pixel_format();
            let pitch = surface.pitch();

            surface.with_lock_mut(|s| {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        unsafe {
                            s[(y * pitch) as usize + x as usize * pf.byte_size_per_pixel() + 2] =
                                M[y as usize][x as usize].r;
                            s[(y * pitch) as usize + x as usize * pf.byte_size_per_pixel() + 1] =
                                M[y as usize][x as usize].g;
                            s[(y * pitch) as usize + x as usize * pf.byte_size_per_pixel() + 0] =
                                M[y as usize][x as usize].b;
                        }
                    }
                }
            });

            surface.update_window().unwrap();
        }
    }
}
