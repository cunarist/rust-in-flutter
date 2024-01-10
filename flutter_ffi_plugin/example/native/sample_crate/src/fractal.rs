//! Code for drawing fractal for performance tests and examples.
//! Copied and modified from
//! https://github.com/abour/fractal

use image::{self, ImageEncoder};

const WIDTH: u32 = 384;
const HEIGHT: u32 = 384;
const BUF_SIZE: u32 = WIDTH * HEIGHT * 3;
const NB_SAMPLES: u32 = 1;
const SIZE: f64 = 0.000000001;
const MAX_ITER: u32 = 1000;

pub fn fractal(scale: f64) -> Option<Vec<u8>> {
    let point_x: f64 = -0.5557506;
    let point_y: f64 = -0.55560;
    let mut buffer: Vec<u8> = vec![0; BUF_SIZE as usize];

    render(&mut buffer, HEIGHT, point_x, point_y, scale);

    let mut image_data: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut image_data);
    let result = encoder.write_image(buffer.as_slice(), WIDTH, HEIGHT, image::ColorType::Rgb8);

    match result {
        Ok(_) => Some(image_data),
        Err(_) => None,
    }
}

fn render(buffer: &mut Vec<u8>, height: u32, point_x: f64, point_y: f64, scale: f64) {
    for y in 0..height {
        let (line, line_number) = render_line(y, point_x, point_y, scale);
        write_line(buffer, &line, line_number);
    }
}

fn write_line(buffer: &mut Vec<u8>, line: &Vec<u8>, line_number: u32) {
    for i in 0..WIDTH {
        buffer[(((line_number * WIDTH) + i) * 3) as usize] = line[(i * 3) as usize];
        buffer[((((line_number * WIDTH) + i) * 3) + 1) as usize] = line[((i * 3) + 1) as usize];
        buffer[((((line_number * WIDTH) + i) * 3) + 2) as usize] = line[((i * 3) + 2) as usize];
    }
}

fn render_line(line_number: u32, px: f64, py: f64, scale: f64) -> (Vec<u8>, u32) {
    let line_size = WIDTH * 3;
    let mut line: Vec<u8> = vec![0; line_size as usize];

    for x in 0..WIDTH {
        // Calculate the offset from the center for x and y
        let center_offset_x = (x as f64 - WIDTH as f64 / 2.0) / (WIDTH as f64 / 2.0);
        let center_offset_y = (line_number as f64 - HEIGHT as f64 / 2.0) / (HEIGHT as f64 / 2.0);

        let sampled_colours = (0..NB_SAMPLES)
            .map(|_| {
                let nx = SIZE * center_offset_x * scale + px;
                let ny = SIZE * center_offset_y * scale + py;
                let (m_res, m_iter) = mandelbrot_iter(nx, ny);
                paint(m_res, m_iter)
            })
            .map(|(r, g, b)| (r as i32, g as i32, b as i32));

        let (r, g, b): (i32, i32, i32) = sampled_colours
            .fold((0, 0, 0), |(cr, cg, cb), (r, g, b)| {
                (cr + r, cg + g, cb + b)
            });

        line[(x * 3) as usize] = ((r as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 1) as usize] = ((g as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 2) as usize] = ((b as f64) / (NB_SAMPLES as f64)) as u8;
    }

    (line, line_number)
}

fn paint(r: f64, n: u32) -> (u8, u8, u8) {
    if r > 4. {
        return hsl_to_rgb(n as f64 / 800. * r, 1., 0.5);
    } else {
        return (255, 255, 255);
    }
}

fn mandelbrot_iter(px: f64, py: f64) -> (f64, u32) {
    let (mut x, mut y, mut xx, mut yy) = (0., 0., 0., 0.);
    let mut xy;

    for i in 0..MAX_ITER {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4. {
            return (xx + yy, i);
        }
        x = xx - yy + px;
        y = 2. * xy + py;
    }

    return (xx + yy, MAX_ITER);
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let (r, g, b);

    if s == 0. {
        r = l;
        g = l;
        b = l;
    } else {
        let (q, p);
        if l < 0.5 {
            q = l * (1. + s);
        } else {
            q = l + s - l * s;
        }

        p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    return ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8);
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0. {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < (1.0 / 6.0) {
        return p + (q - p) * 6.0 * t;
    } else if t < (1.0 / 2.0) {
        return q;
    } else if t < (2.0 / 3.0) {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    return p;
}
