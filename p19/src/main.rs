extern crate bmp;


fn main() {
    let img = bmp::open("data/simple.bmp").unwrap();

    let mut outimg = bmp::Image::new(img.get_width(), img.get_height());

    const FSIZE: usize = 9;
    let factor = 1.0/9.0;
    let mut filter = [[factor; FSIZE];FSIZE];

    for (x, y) in img.coordinates() {
        outimg.set_pixel(x, y, img.get_pixel(x,y));
    }

    let _ = img.save("data/out.bmp");

}
