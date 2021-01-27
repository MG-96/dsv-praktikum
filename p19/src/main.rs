extern crate bmp;
// Specify the extern crate in your lib.rs or main.rs
extern crate simple_matrix;

// You can now use it
use simple_matrix::Matrix;


fn main() {
    let img = bmp::open("./data/LoewPaint.bmp").unwrap(); //bmp::Image::new(100, 100);
    
    let mut mat: Matrix<i32> = Matrix::new(3, 3);
    let mut outimg = bmp::Image::new(img.get_width(), img.get_height());

    let mut i: usize = 0;
    let mut j: usize = 0;
    let k = 1;

    let factor: i32 = 1;

    mat.set(0, 0, 1);
    mat.set(0, 1, 1);
    mat.set(0, 2, 1);
    mat.set(1, 0, 1);
    mat.set(1, 1, -2);
    mat.set(1, 2, 1);
    mat.set(2, 0, -1);
    mat.set(2, 1, -1);
    mat.set(2, 2, -1);

    // Iterate through the matrix (row by row)
/*     while i <= 2 {
        while j <= 2 {

            mat.set(i as usize, j as usize, k);
            j += 1;
        }
        j = 0;
        i +=1 ;
    } */

    /*
    for val in mat {
        print!("{} ", val);
    } */

/*     for (x, y) in img.coordinates() {
        outimg.set_pixel(x, y, img.get_pixel(x,y));
    } */


    let bilbreite = outimg.get_width();
    let bildhoehe = outimg.get_height();

    /** (Destruktive) Bildoperation   */
    for (x, y) in img.coordinates() {

        if (x > 1) && (x < bilbreite - 1) && (y > 1) && (y < bildhoehe - 1) {
            let mut neuer_pixel_grauwert: i32 = 0;

            i = 0;
            j = 0;

            while i <= 2 {

                while j <= 2 {

                    let op1: u32 = x as u32 - i as u32;
                    let op2: u32 = y as u32 - j as u32;

                    let pixelwert = img.get_pixel(op1, op2);
                    let mittelwert: u8 = (pixelwert.r as f32 * 0.3 + pixelwert.g as f32 * 0.59 + pixelwert.b as f32 * 0.11) as u8;

                    let zwischenwert = mat.get(i, j).unwrap();

                    neuer_pixel_grauwert += (mittelwert as i32 * zwischenwert) as i32;

                    j += 1;
                }
                j = 0;
                i +=1;
            }

            let neuer_pixel_rgb_wert = bmp::Pixel::new((neuer_pixel_grauwert / factor) as u8, (neuer_pixel_grauwert / factor) as u8, (neuer_pixel_grauwert / factor) as u8);

            outimg.set_pixel(x, y, neuer_pixel_rgb_wert);

        }
    }

    let _ = outimg.save("./data/out.bmp");

}
