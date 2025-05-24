use lab3::{
    math::{Point3, Vector3},
    scene::{Camera, Sphere},
};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    let camera = Camera::new(
        Point3::new(0., 0., 0.),
        Vector3::new(0., 0., -1.),
        Vector3::new(0., 1., 0.),
        std::f32::consts::FRAC_PI_4,
        WIDTH,
        HEIGHT,
    );

    let sphere = Sphere::new(Point3::new(0., 0., -2.), 0.5);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = camera.get_ray_through(x, y);

        if sphere.intersection(&ray) {
            *pixel = image::Rgb([255u8, 255u8, 255u8]);
            continue;
        }

        let mut dir = *ray.dir();
        // scale back into 0..1
        dir += Vector3::new(1., 1., 1.);
        dir *= 0.5;

        *pixel = dir.to_rgb();
    }

    imgbuf.save("output/test.png").unwrap();
}
