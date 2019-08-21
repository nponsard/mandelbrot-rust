extern crate image;

fn main() {
    let start = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let mut max_i = 100 as u16;
    let mut xmax = 5000 as u32;
    let mut ymax = 5000 as u32;

    if args.len() >= 2{
        max_i = args[1].parse().unwrap_or(100);
    }
    if args.len() >= 4 {
        xmax = args[2].parse().unwrap_or(500);
        ymax= args[3].parse().unwrap_or(500);
    }
    






    /*preset
    0.025 scale
    x -0.24
    y -0.721
    */

    let scalex = 2.0 / xmax as f64;

    let scaley = 2.0 / ymax as f64;
    /*
    let scalex = 0.025 / xmax as f64;

    let scaley = 0.025 / ymax as f64;
    */
    let nxm = xmax as f64;
    let nym = ymax as f64;
    let mut imgbuf = image::RgbImage::new(xmax, ymax);

    for x in 0..xmax {
        let nx = x as f64;
        for y in 0..ymax {
            let ny = y as f64;
            let mut a = (nx - nxm / 2.0) * scalex - 0.5;
            let mut b = (ny - nym / 2.0) * scaley - 0.0;
            /*
            let mut a = (nx - nxm / 2.0) * scalex - 0.24;
            let mut b = (ny - nym / 2.0) * scaley - 0.721;
            */

            let oa = a;
            let ob = b;
            let mut n = 0;
            while n < max_i && a * a + b * b < 64.0 {
                let aa = a * a - b * b;
                let bb = 2.0 * a * b;
                a = aa + oa;
                b = bb + ob;

                n += 1;
            }
            let level:u8 =(( n*255/max_i)) as u8 ;
            imgbuf.get_pixel_mut(x, y).data = [level,level,level];
            /*
            if n == max_i {
                imgbuf.get_pixel_mut(x, y).data = [0, 0, 0];
            } else if n as f32 > max_i as f32 * 0.9 {
                imgbuf.get_pixel_mut(x, y).data = [10, 100, 250];
            } else if n as f32 > max_i as f32 * 0.8 {
                imgbuf.get_pixel_mut(x, y).data = [255, 0, 0];
            } else if n as f32 > max_i as f32 * 0.7 {
                imgbuf.get_pixel_mut(x, y).data = [200, 50, 200];
            } else if n as f32 > max_i as f32 * 0.6 {
                imgbuf.get_pixel_mut(x, y).data = [0, 250, 0];
            } else if n as f32 > max_i as f32 * 0.5 {
                imgbuf.get_pixel_mut(x, y).data = [250, 100, 50];
            } else if n as f32 > max_i as f32 * 0.4 {
                imgbuf.get_pixel_mut(x, y).data = [0, 250, 250];
            } else if n as f32 > max_i as f32 * 0.3 {
                imgbuf.get_pixel_mut(x, y).data = [60, 50, 0];
            } else if n as f32 > max_i as f32 * 0.2 {
                imgbuf.get_pixel_mut(x, y).data = [0, 100, 80];
            } else {
                imgbuf.get_pixel_mut(x, y).data = [n as u8 * 2, n as u8 * 2, n as u8 * 2];
            }*/
        }
    }
    let path = format!("mandelbrot {} {} {}.png",max_i,xmax,ymax);
    let _result = imgbuf.save(path);
    println!("finished in {} ms", start.elapsed().as_millis())
}
