use image::{ImageBuffer, Rgba};
use num::{complex::Complex, traits::Inv};
// use rand::{Rng, distributions::Uniform};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
mod rgbaf;
use rgbaf::RgbaF;

pub struct Args {
    origin: Complex<f32>,
    zoom: f32,
    samples: usize,
    limit: f32,
    bail: f32,
    // c_exp: f32,
    sample_d: f32,
}

fn normalize_coords(x: i64, y: i64, w: i64, h: i64, z: f32) -> Complex<f32> {
    let nx = 2.0 * (x as f32 / w as f32) - 1.0;
    let ny = 2.0 * (y as f32 / h as f32) - 1.0;
    Complex::new(nx / z, ny * (h as f32 / w as f32) / z)
    //Complex::new( ((x - (w / 2)) as f32) / (w as f32 / 2.0 * z), ((y - (h / 2)) as f32) / (h as f32 / 2.0 * z) )
}

fn compute_brot(i: i64, w: i64, h: i64, args: &Args) -> Rgba<u16> {
    let mut out = RgbaF::new(0.0, 0.0, 0.0, 0.0, false);
    let d = normalize_coords(1, 1, w, h, args.zoom) - normalize_coords(0, 0, w, h, args.zoom);
    // let mut rng = rand::thread_rng();
    // let distr = Uniform::new(-1.0, 1.0);
    // let mut maxv = 0.0;
    // let mut minv = 0.0;
    // let currPoint = normalize_coords(i / h, i % h, w, h, args.zoom) + args.origin;
    let mut past_iter_point = Complex::new(0.0,0.0);
    for s in 0..args.samples {
        let mut c = normalize_coords(i / h, i % h, w, h, args.zoom) + args.origin;
        // c.re += d.re * (rng.sample(distr) / args.sample_d);
        // c.im += d.im * (rng.sample(distr) / args.sample_d);
        c.re += d.re * (2.0 * (s % ((args.samples as f32).sqrt() as usize)) as f32 / (args.samples as f32).sqrt() - 1.0) / args.sample_d; // equally spaced deltas
        c.im += d.im * (2.0 * (s / ((args.samples as f32).sqrt() as usize)) as f32 / (args.samples as f32).sqrt() - 1.0) / args.sample_d;
        let mut z = c.clone();
        // let dc = 0.0;
        // let mut dz = c.clone();
        // let mut angle_sum = 0.0;
        // let mut act_to_der = Complex::new(0.0,0.0);
        let mut i = 0.0;
        // let mut deltaZ = Complex::new(0.0,0.0);
        // let mut s = 0.0;
        // abs(z) < args.bail &&
        // let mut max_norm = 0.0;
        // let mut min_norm = 0.0;
        let mut z_sum_norm = 0.0;
        while (z.norm_sqr() < args.bail) && i < args.limit {
            // if i % 20.0 == 0.0 {
            //     past_iter_point = z.clone();
            // } else if past_iter_point == z {
            //     i = args.limit;
            //     // dbg!("bailing!");
            //     break;
            // }
            // act_to_der += dz;

            // deltaZ = z*z+c - z;
            // z = z + deltaZ;
            z_sum_norm += z.norm();
            z = z*z + c;

            // if (z).norm_sqr() > max_norm {
            //     max_norm = (z).norm_sqr();
            // }
            // if (z).norm_sqr() < min_norm {
            //     min_norm = (z).norm_sqr();
            // }
            // deltaZ = z-c;
            // angle_sum = angle_sum + (deltaZ.im/deltaZ.re).atan();
            // let angle = (deltaZ.im).atan2(deltaZ.re);
            // if angle < 0.0 {
            //     angle = 2.0*std::f32::consts::PI + angle;
            // }
            // angle_sum = angle_sum + angle;
            // dz = (dz * 2.0 * z) + dc;
            i += 1.0;
            // s = s + (-(abs(z).sqrt())).exp();
        }
        // .powf(c_exp)).powf(1.5)
        // dbg!(s);
        // if i > args.limit/2.0 {
        //     dbg!(i);
        //     panic!("helo")
        //     // dbg!(v);
        // }


        // z = (z-c).sqrt(); // go back to the one just before exit
        // let mut v = 1000.0* act_to_der.norm_sqr() / args.bail;
        // let mut v: f32 = act_to_der.norm_sqr();
        // let mut v: f32 = angle_sum;
        // if s == 0 {
        //     maxv = v;
        //     minv = v;
        // }
        // if v > maxv {maxv = v;}
        // if v < minv {minv = v;}

        let mut v = ((z_sum_norm / i).log2()+2.0).inv().powf(0.75);

        // v = v.sqrt();

        // let mut v: f32 = angle_sum; // (2.0*std::f32::consts::PI);
        // let mut v: f32 = max_norm-min_norm;//.sqrt();
        // dbg!(v);
        // if v < 0.0 {
        //     dbg!(v);
        // }
        // v = (1.0 - (1.0/(1.0+v.abs())));
        // v = v.sin();
        // v = (2.0/(1.0+(-v/1.0).exp()));
        // let mut v = (z.norm_sqr()) / args.bail;
        // v = v.powf(1.0/16.0);

        // if v > 1.0 {
        //     dbg!(v, z.norm_sqr(), args.bail);
        // }
        // let mut v = (i) / args.limit;
        // dbg!(v);
        // v = v.exp();
        // v = v.powf(1.0/16.0); // adjust power if needed, smaller exponent is more color
        // v = v* (1.0-((z-args.origin).norm_sqr()).sin()/1.0);
        // v = -v + 1.0;
        // v = v + 1.0;
        // let mut v = 1.0;
        // dbg!("edited");
        // dbg!(v);
        // v = (v+2.0).ln();
        // dbg!(v);
        // let v = ((1.0 - (s / args.limit)) * 360.0).powf(1.5);
        // println!("{}", v);
        // let mut v = (1.0 - (s / args.limit).exp()).abs();
        // let v = ((dz.re.atan2(dz.im) + (PI / 2.0)) / (PI)).abs();

        //let mut color = palette.get_color(v);
        // let mut color = RgbaF::from_hsv(v, 0.5, 1.0, 1.0);
        let mut color = RgbaF::from_hsv((v*360.0 + 180.0) % 360.0, v, 1.0 - v, 1.0);
        // dbg!(color);
        color.a = 1.0;
        color = color.to_sRGB();
        color.r = 1.0 - color.r;
        color.g = 1.0 - color.g;
        color.b = 1.0 - color.b;

        // dbg!(color);
        let cond: u8 = (i < args.limit).into();
        // dbg!(cond);
        out = out + (color * color * (cond as f32));
        // out = out + (color * color * 1.0);
        out.a = out.a + 1.0;
        // out.r = 1.0 - out.r;
        // out.g = 1.0 - out.g;
        // out.b = 1.0 - out.b;
    }
    out = out / args.samples as f32;
    // dbg!("st", maxv, minv, "en");
    // println!();
    Rgba::from(
        out.to_RGB()
            .to_arr()
            .map(|v| (v.sqrt() * u16::MAX as f32) as u16),
    )
}

fn main() {
    // let (w, h) = (1920*1, 1080*1);
    // let (w, h) = (10_000, 10_000);
    let (w, h) = (2560, 1600);
    let mut output = ImageBuffer::<Rgba<u16>, Vec<u16>>::new(w as u32, h as u32);
    println!("{}x{}", w, h);

    // let args = Args {
    //     // origin: Complex::new(-0.75, 0.0),
    //     // origin: Complex::new(0.25, 0.0),
    //     origin: Complex::new(-0.743_643_900_055, 0.131_825_890_901),
    //     // zoom: 0.5,
    //     zoom: 1500.0,
    //     samples: 50,
    //     limit: 128.0*3.0*4.0,
    //     bail: 8.0,
    //     // c_exp: 2.0,
    //     sample_d: 18.0,
    // };

    let args = Args {
        // origin: Complex::new(-0.75, 0.0),
        // origin: Complex::new(0.25, 0.0),
        origin: Complex::new(-0.743_643_900_055, 0.131_825_890_901),
        // zoom: 0.5,
        zoom: 15000.0,
        samples: 49,
        limit: 128.0*2.0*4.0,//*2.0*4.0,
        bail: 8.0,
        // bail: 2.0,
        // c_exp: 2.0,
        sample_d: 8.0,
    };

    // let args = Args {
    //     // origin: Complex::new(-0.75, 0.0),
    //     // origin: Complex::new(0.25, 0.0),
    //     origin: Complex::new(0.25, 0.0),
    //     // zoom: 0.5,
    //     zoom: 0.5,
    //     samples: 50,
    //     limit: 128.0*3.0*4.0,
    //     bail: 8.0,
    //     // c_exp: 2.0,
    //     sample_d: 18.0,
    // };
   println!("width and height: {}", normalize_coords(w, h, w, h, args.zoom) - normalize_coords(0, 0, w, h, args.zoom));
    let out: Vec<Rgba<u16>> = (0..(w * h))
        .into_par_iter()
        .map(|i| compute_brot(i as i64, w as i64, h as i64, &args))
        .collect();
    //out.into_par_iter().enumerate().for_each(|(i, x)| output[(i as u32 % w as u32, i as u32 / h as u32)] = x);
    for (i, e) in out.iter().enumerate() {
        // dbg!(e);
        let (x, y) = (
            (i as u64 / ((h) as u64)) as u32,
            (i as u64 % (h) as u64) as u32,
        );
        if (y as u64) < h.try_into().unwrap() {
            output.put_pixel(x, y, *e);
        }
    }
    output.save("ok.png").unwrap();
    println!("Hello, world!");
}
