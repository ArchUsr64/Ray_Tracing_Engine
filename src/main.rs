#![allow(dead_code)]

mod camera;
use camera::Camera;

mod ctrlc;
mod fs_image;
mod matrix;
mod term;

mod vector;
use vector::Point3;
use vector::Vec3;

mod traits;
use traits::ToF64;

const NULL: i32 = 0;

#[derive(Copy, Clone)]
struct Colour {
    r: f64,
    g: f64,
    b: f64,
}
fn colour_init<T: ToF64>(red: T, green: T, blue: T) -> Colour {
    Colour {
        r: red.to_f64(),
        g: green.to_f64(),
        b: blue.to_f64(),
    }
}

struct Ray3 {
    origin: Point3,
    direction: Vec3,
}

impl Ray3 {
    fn new(origin_position: &Point3, direction_vec: &Vec3) -> Ray3 {
        Ray3 {
            origin: *origin_position,
            direction: *direction_vec,
        }
    }
}

#[derive(Copy, Clone)]
struct Sphere {
    centre: Point3,
    radius: f64,
    colour: Colour,
}

fn main() {
    let image_width = 1000;
    let image_height = 1000;
    ctrlc::ctrl_c_init();
    let camera = Camera::new(
        &Point3::new(4.5, 5.0, 4.5),
        0f64.to_radians(),
        -90f64.to_radians(),
        90f64.to_radians(),
    );
    let mut s: [Sphere; 100] = [Sphere {
        centre: Point3::new(NULL, NULL, NULL),
        radius: 0.25,
        colour: colour_init(1, 0, 0),
    }; 100];
    for i in 0..s.len() {
        s[i].centre = Point3::new((i % 10) as f64, 0.0, (i / 10) as f64);
    }
    let mut pix_arr = vec![vec![vec![0f64; 3]; image_height]; image_width];
    for i in 0..image_width {
        for j in 0..image_height {
            let ray = spawn_ray(i + (j * image_height), image_width, image_height, &camera);
            for k in 0..s.len() {
                let (ray_intersects, intersection_point) =
                    sphere_ray_intersection_test(&s[k], &ray);
                let strength = Vec3::dot(
                    &s[k].centre.vec_to(&intersection_point).normalise(),
                    &s[k].centre.vec_to(&camera.position).normalise(),
                );
                // let strength = 1.0;
                if ray_intersects {
                    pix_arr[i][j] = vec![
                        strength * s[k].colour.r,
                        strength * s[k].colour.g,
                        strength * s[k].colour.b,
                    ]
                }
            }
        }
        term::reset_terminal();
        term::print_progress("Spawning rays", i as f64 / image_height as f64);
    }
    fs_image::create_image(&pix_arr);
}

fn sphere_ray_intersection_test_tmp(sphere: &Sphere, ray: &Ray3) {}

fn sphere_ray_intersection_test(sphere: &Sphere, ray: &Ray3) -> (bool, Point3) {
    let (ray_intersects, intersection_point): (bool, Point3);
    if &sphere.centre.distance_from(&ray.origin) <= &sphere.radius {
        let ray_intersects = false;
        let intersection_point = Point3::new(NULL, NULL, NULL);
    }
    let ray_direction_normalised_vec = ray.direction.normalise();
    let ray_origin_to_sphere_vec = ray.origin.vec_to(&sphere.centre);
    let dist_to_sphere_nearest_point_along_ray =
        Vec3::dot(&ray_origin_to_sphere_vec, &ray_direction_normalised_vec);
    if dist_to_sphere_nearest_point_along_ray <= 0.0 {
        let ray_intersects = false;
        let intersection_point = Point3::new(NULL, NULL, NULL);
    }
    let sphere_centre_to_ray_perpendicular_vec = ray_direction_normalised_vec
        .scale(dist_to_sphere_nearest_point_along_ray)
        - ray_origin_to_sphere_vec;

    let ray_sphere_perpendicular_dist = sphere_centre_to_ray_perpendicular_vec.length();

    if ray_sphere_perpendicular_dist <= sphere.radius {
        ray_intersects = true;
        let ray_length_inside_sphere =
            2f64 * pythagorean_solver(sphere.radius, ray_sphere_perpendicular_dist);
        let ray_intersection_dist =
            dist_to_sphere_nearest_point_along_ray - ray_length_inside_sphere / 2f64;
        intersection_point = (ray.origin.to_vec()
            + ray_direction_normalised_vec.scale(ray_intersection_dist))
        .to_point();
    } else {
        ray_intersects = false;
        intersection_point = Point3::new(NULL, NULL, NULL);
    }
    (ray_intersects, intersection_point)
}

fn pythagorean_solver(hypotenuse: f64, side: f64) -> f64 {
    let other_side = hypotenuse.powi(2) - side.powi(2);
    other_side.sqrt()
}

fn spawn_ray(ray_index: usize, image_width: usize, image_height: usize, camera: &Camera) -> Ray3 {
    let mut pixel_canvas_coordinate = Point3::new(
        ray_index % image_width,
        ray_index / image_width,
        NULL as usize,
    );
    let camera_view_vector = camera.view_vec();
    pixel_canvas_coordinate.x /= image_width as f64;
    pixel_canvas_coordinate.y /= image_height as f64;
    pixel_canvas_coordinate.x -= 0.5;
    pixel_canvas_coordinate.y -= 0.5;
    pixel_canvas_coordinate.x *= 2f64;
    pixel_canvas_coordinate.y *= 2f64;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64;
    let canvas_center: Point3 = (camera.position.to_vec() + camera_view_vector).to_point();
    let mut canvas_x_vec: Vec3 = Vec3::new(NULL, NULL, NULL);
    canvas_x_vec.x = camera_view_vector.z;
    canvas_x_vec.x /= (1f64 + camera_view_vector.x.powi(2)).sqrt();
    canvas_x_vec.z = pythagorean_solver(1f64, canvas_x_vec.x);
    let mut canvas_y_vec = canvas_x_vec * camera_view_vector;
    canvas_x_vec = canvas_x_vec.scale((camera.fov / 2f64).tan());
    canvas_y_vec = canvas_y_vec.scale(canvas_x_vec.length() / aspect_ratio);
    let pixel_world_coordinate = (canvas_center.to_vec()
        + (canvas_x_vec.scale(pixel_canvas_coordinate.x)
            + canvas_y_vec.scale(pixel_canvas_coordinate.y)))
    .to_point();
    Ray3::new(
        &camera.position,
        &pixel_world_coordinate.vec_from(&camera.position),
    )
}
