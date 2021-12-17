#![feature(test)]

fn main() {
    let x_range = [244, 303];
    let y_range = [-91, -54];

    let mut cnt = 0;
    for x_velocity in 0..1000 {
        for y_velocity in -1000..1000 {
            let (mut x, mut y) = (0, 0);
            let (mut xv, mut yv) = (x_velocity, y_velocity);
            while x <= x_range[1] && y >= y_range[0] {
                x += xv;
                y += yv;
                if x >= x_range[0] && x <= x_range[1] && y >= y_range[0] && y <= y_range[1] {
                    cnt += 1;
                    break;
                }
                if xv > 0 {
                    xv -= 1;
                }
                yv -= 1;
            }
        }
    }
    println!("{}", cnt);
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| main());
    }
}
