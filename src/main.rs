use lab3::math::{Point3, Ray, Vector3};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const ASPECT: f32 = HEIGHT as f32 / WIDTH as f32;
const FOV_X: f32 = std::f32::consts::FRAC_PI_4;
const FOV_Y: f32 = FOV_X * ASPECT;

fn get_cam_ray(u: u32, v: u32) -> Ray {
    let x = ((2 * u as i32) - WIDTH as i32) as f32 / WIDTH as f32 * FOV_X.tan();
    let y = ((2 * v as i32) - HEIGHT as i32) as f32 / HEIGHT as f32 * FOV_Y.tan();
    let mut dir = Vector3::new(x, y, -1.);
    dir.normalize();
    Ray::new(Point3::zero(), dir)
}

fn main() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = get_cam_ray(x, y);
        let mut dir = *ray.dir();
        // scale back into 0..1
        dir += Vector3::new(1., 1., 1.);
        dir *= 0.5;
        *pixel = dir.to_rgb();
    }

    imgbuf.save("output/test.png").unwrap();
}
