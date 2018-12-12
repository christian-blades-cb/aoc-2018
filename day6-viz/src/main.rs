use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    use image::{ImageBuffer, Rgb};
    use palette::{Gradient, LinSrgb};
    use std::fs::File;

    let mut file = File::open("input-day6")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let points: Vec<Coord> = buf.lines().map(parse_line).collect();

    let gradient = Gradient::new(vec![
        LinSrgb::from(to_floats((0x36, 0x37, 0x55))),
        LinSrgb::from(to_floats((0xE3, 0x5D, 0x66))),
        LinSrgb::from(to_floats((0xF1, 0xE9, 0xC8))),
        LinSrgb::from(to_floats((0x6F, 0xEF, 0xE0))),
        LinSrgb::from(to_floats((0x1F, 0xA1, 0xA3))),
    ]);
    let palette: Vec<Rgb<u8>> = gradient
        .take(points.len())
        .map(|px| {
            let (r, g, b) = to_rgbu8(px.into_components());
            Rgb([r, g, b])
        })
        .collect();

    println!("rendering");
    let img = ImageBuffer::from_fn(400, 400, |x, y| {
        let distances = points
            .iter()
            .enumerate()
            .map(|(i, pt)| (i, manhattan_distance(pt, &(x as isize, y as isize))));
        let min_dist = distances.min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        palette[min_dist.0 % palette.len()]
    });

    println!("saving day6-viz.png");
    img.save("day6-viz.png").unwrap();

    Ok(())
}

fn to_floats(color: (u8, u8, u8)) -> (f64, f64, f64) {
    let (r, g, b) = color;
    let r: f64 = r as f64 / 255.0;
    let g: f64 = g as f64 / 255.0;
    let b: f64 = b as f64 / 255.0;
    (r, g, b)
}

fn to_rgbu8(color: (f64, f64, f64)) -> (u8, u8, u8) {
    let (r, g, b) = color;
    let r = (r * 255.0) as u8;
    let g = (g * 255.0) as u8;
    let b = (b * 255.0) as u8;
    (r, g, b)
}

type Coord = (isize, isize);

#[inline]
fn manhattan_distance(lhs: &Coord, rhs: &Coord) -> usize {
    use std::isize;
    let (l_x, l_y) = lhs;
    let (r_x, r_y) = rhs;

    let x = (*l_x as isize - *r_x as isize).abs();
    let y = (*l_y as isize - *r_y as isize).abs();
    (x + y) as usize
}

fn parse_line(ln: &str) -> Coord {
    let mut sp = ln.split(", ").take(2).map(|x| x.parse::<isize>().unwrap());
    let a = sp.next().unwrap();
    let b = sp.next().unwrap();
    (a, b)
}
