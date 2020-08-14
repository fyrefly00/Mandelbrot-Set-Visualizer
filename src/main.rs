extern crate image;
extern crate num_complex;
extern crate nalgebra;
use hsl::HSL;

fn main() {
    // Define image dimensions and create buffer
    const WIDTH: u32 = 10000;
    const HEIGHT: u32 = 10000;
    let mut img = image::ImageBuffer:: new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // Adjust the x and y to be centered in the middle of the image
            let mut adjusted_x : f64= x as f64;
            adjusted_x = (adjusted_x - WIDTH as f64/ 2.0)  as f64 * (5.0 / WIDTH as f64);
            let mut adjusted_y :f64= y as f64;
            adjusted_y = (adjusted_y - HEIGHT as f64 / 2.0) as f64 * (5.0 / HEIGHT as f64);

            // Complex number creation
            let comp_val = num::complex::Complex::new(adjusted_x as f64 , adjusted_y as f64);
            let mut z = num::complex::Complex::new(0.0 as f64,0.0 as f64);
            
            for i in 0..101 {
                let absz : f64 =  (z.re * z.re) as f64 + (z.im * z.im) as f64;
                let pixel = img.get_pixel_mut(x, y);
                let data = (*pixel as image::Rgb<u8>).0;

                // Coloring function
                if absz.sqrt() > 2.0 {
                    let shade = HSL { h:((255.0 * absz.sqrt()) / 50.0), s: 1_f64, l: 0.5_f64 };
                    *pixel = image::Rgb([shade.to_rgb().0 , shade.to_rgb().1, shade.to_rgb().2]);
                    break;
                }
                else if i == 100 {
                    *pixel = image::Rgb([0 as u8,0 as u8,0 as u8]);
                    break;
                }
                else {
                    z =z * z + comp_val;
                }
            }
        }
    }
    img.save("fractal.png").unwrap();
}
