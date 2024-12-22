use app::*;
use core::num::NonZeroU32;
use rand::Rng;
use std::rc::Rc;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

mod app;

fn bru(x: u32) -> u32 {
    let mut lp2 = 1;
    while lp2 <= x {
        lp2 <<= 1;
    }
    lp2 as u32 - 1
}

fn color(x: u32, y: u32, h: u32, w: u32, s: (u32, u32)) -> u32 {
    let x2 = x ^ s.0;
    let y2 = y ^ s.1;
    let xmax = bru(w) | s.0;
    let ymax = bru(h) | s.1;
    adjust(x2 + y2, xmax + ymax)
}

fn adjust(v: u32, vm: u32) -> u32 {
    const CMAX: u64 = 255 * (65536 + 256 + 1);
    ((v as u64 * CMAX) / vm as u64) as u32
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = WinitAppBuilder::with_init(
        |ael| {
            let window = ael.create_window(Window::default_attributes());
            Rc::new(window.unwrap())
        },
        |_ael, win| {
            let PhysicalSize { height, width } = win.inner_size();
            let context = softbuffer::Context::new(win.clone()).unwrap();
            let surface = softbuffer::Surface::new(&context, win.clone()).unwrap();

            (surface, (0u32, 0u32), rand::thread_rng())
        },
    )
    .with_event_handler(|win, surf, evt, ael| {
        let PhysicalSize { height, width } = win.inner_size();
        let (ref mut surf, ref mut state, ref mut rng) = surf.unwrap();
        surf.resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();

        let mut buf = surf.buffer_mut().unwrap();
        for x in 0..width {
            for y in 0..height {
                buf[(y * width + x) as usize] = color(x, y, height, width, *state);
            }
        }
        buf.present().unwrap();

        if rng.gen::<f32>() < 0.1 {
            state.0 += 1;
        }
        if rng.gen::<f32>() < 0.1 {
            state.1 += 1;
        }
    });

    event_loop.run_app(&mut app).unwrap();
}
