use lab3::{
    math::{Point3, Vector3},
    scene::{Camera, Sphere},
};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const FOV_X: f32 = std::f32::consts::FRAC_PI_4;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    let camera = Camera::new(
        Point3::new(0., 0., 0.),
        Vector3::new(0., 0., -1.),
        Vector3::new(0., 1., 0.),
        FOV_X,
        WIDTH,
        HEIGHT,
    );

    let sphere = Sphere::new(Point3::new(0., 0., -3.), 1.0);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = camera.get_ray_through(x, y);

        *pixel = match sphere.intersection(&ray) {
            Some(intersection) => {
                let dir = (intersection.normal + Vector3::new(1., 1., 1.)) * 0.5;
                dir.to_rgb()
            }
            None => image::Rgb([0u8, 0u8, 0u8]),
        };
    }

    imgbuf.save("output/test.png").unwrap();
}
