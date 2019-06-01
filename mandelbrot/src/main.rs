extern crate num;
use num::Complex;

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not member, return `Some(i)`, where `i` is the number of 
/// iterationsit took for `c` to leave the circle of radius two centered on 
/// the origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit, without being able to prove that `c` is not a member), 
/// return `None`.
fn escape_time(c: Complex<u64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
