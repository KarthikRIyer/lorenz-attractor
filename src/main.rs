use kiss3d::window::Window;
use kiss3d::light::Light;
use palette::{Hsv, Rgb, RgbHue};
use kiss3d::nalgebra::Point3;

fn main() {
    let mut points : Vec<Point3<f32>> = Vec::new();

    let mut window = Window::new("Lorenz Attractor");
    window.set_light(Light::StickToCamera);

    let a = 10.0;
    let b = 28.0;
    let c = 8.0/3.0;

    let scale : f32 = 0.5;

    let mut x = 0.01;
    let mut y = 0.0;
    let mut z = 0.0;

    points.push(Point3::new(scale * x, scale * y, scale * z));

    while window.render() {
        let dt: f32 = 0.01;
        let dx = (a * (y - x)) * dt;
        let dy = (x * (b - z) - y) * dt;
        let dz = (x * y - c * z) * dt;
        x = x + dx;
        y = y + dy;
        z = z + dz;

        points.push(Point3::new(scale * x, scale * y, scale * z));

        let mut hu = -180.0;
        for p in points.windows(2).collect::<Vec<_>>() {
            let hex_color = Hsv{
                hue: RgbHue::from(hu),
                saturation: 1.0,
                value: 1.0
            };
            let srgb = Rgb::from(hex_color);
            let color = Point3::new(srgb.red,
                                    srgb.green,
                                    srgb.blue);

            window.draw_line(&p[0], &p[1], &color);
            hu += 0.1;
            if hu > 180.0 {
                hu = -180.0;
            }
        }
    }
}
