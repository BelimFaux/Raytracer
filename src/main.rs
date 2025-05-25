use lab3::{
    math::{Color, Point3, Vector3},
    scene::{Camera, Light, Material, Sphere},
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const FOV_X: f32 = std::f32::consts::FRAC_PI_4;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

    let background_color = Color::new(0., 0., 0.);
    let camera = Camera::new(
        Point3::new(0., 0., 1.),
        Vector3::new(0., 0., -2.5),
        Vector3::new(0., 1., 0.),
        FOV_X,
        WIDTH,
        HEIGHT,
    );

    let light = Light::Ambient {
        color: Color::new(1.0, 1.0, 1.0),
    };

    let spheres = [
        Sphere::new(
            Point3::new(-2.1, -2., -3.),
            1.0,
            Material::new(Color::new(0.25, 0.18, 0.5), 0.3, 0.9, 1.0, 200),
        ),
        Sphere::new(
            Point3::new(0., 0., -3.),
            1.0,
            Material::new(Color::new(0.95, 0.63, 0.01), 0.3, 0.9, 1.0, 200),
        ),
        Sphere::new(
            Point3::new(2.1, 2., -3.),
            1.0,
            Material::new(Color::new(0.13, 0.43, 0.1), 0.3, 0.9, 1.0, 200),
        ),
    ];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = camera.get_ray_through(x, HEIGHT - y);

        let mut min = f32::INFINITY;
        let mut col = background_color.to_rgb();
        for sphere in spheres.iter() {
            match sphere.intersection(&ray) {
                Some(intersection) => {
                    if intersection.t < min {
                        min = intersection.t;
                        col = sphere.intersection_color(&intersection, &light).to_rgb();
                    }
                }
                None => continue,
            }
        }

        *pixel = col
    }

    imgbuf.save("output/test.png").unwrap();
}
