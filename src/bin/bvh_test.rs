use rayt::{
    camera::Camera,
    hittable::{HitRecord, Hittable, Shape},
    hittable_list::HittableList,
    bvh::BvhNode,
    material::{Material, Scatter},
    ray::Ray,
    utils::{INFINITY, clamp},
    vec3::{unit_vector, Color, Point3, Vec3},
};

use rayon::prelude::*;
//use std::sync::Arc;

#[macro_use]
extern crate rayt;
#[macro_use]
extern crate itertools;

fn ray_color(r: &Ray, world: &Shape, depth: usize) -> Color {
    let mut rec = HitRecord::new(Material::new_lambertian(Color::zero()));
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point3::zero(), Vec3::zero(), 0.0);
        let mut attenuation = Color::zero();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0);
}

fn prism() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Material::new_metal(Color::from([0.7, 0.2, 0.1]), 0.7);
    world.add(Shape::new_triangle(Point3::from([1000.0,0.0,0.0]),Point3::from([0.0,0.0,-1000.0]),Point3::from([0.0,0.0,1000.0]),ground_material.clone()));
    world.add(Shape::new_triangle(Point3::from([0.0,0.0,-1000.0]),Point3::from([-1000.0,0.0,0.0]),Point3::from([0.0,0.0,1000.0]),ground_material.clone()));
    let prism_mat = Material::new_metal(Color::from([0.5, 0.5, 0.5]), 0.2);
    let p1 = Point3::from([2.0,0.0,0.0]);
    let p2 = Point3::from([2.0,2.0,0.0]);
    let p3 = Point3::from([0.0,2.0,0.0]);
    let p4 = Point3::from([0.0,0.0,0.0]);
    let p5 = Point3::from([0.0,0.0,2.0]);
    let p6 = Point3::from([0.0,2.0,2.0]);
    world.add(Shape::new_triangle(p2,p1,p4,prism_mat.clone()));
    world.add(Shape::new_triangle(p3,p2,p4,prism_mat.clone()));
    world.add(Shape::new_triangle(p3,p6,p2,prism_mat.clone()));
    world.add(Shape::new_triangle(p1,p5,p4,prism_mat.clone()));
    world.add(Shape::new_triangle(p3,p4,p6,prism_mat.clone()));
    world.add(Shape::new_triangle(p4,p5,p6,prism_mat.clone()));
    world.add(Shape::new_triangle(p1,p6,p5,prism_mat.clone()));
    world.add(Shape::new_triangle(p1,p2,p6,prism_mat.clone()));
    world
}

fn render(cam: &Camera, world: &Shape) -> Vec<Pixel> {
    let pix_coord: Vec<(u32,u32)> = iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH).collect();
    let img: Vec<Pixel> = pix_coord.par_iter().map(|(row, col)| simu(*row, *col, cam, world)).collect();
    img
}

fn simu(row: u32, col: u32, cam: &Camera, world: &Shape) -> Pixel {
    let pixel_color = (1..=SAMPLES_PER_PIXEL)
        .map(|_| {
            let u = (col as f64 + random_double!()) / (IMAGE_WIDTH - 1) as f64;
            let v = (row as f64 + random_double!()) / ( IMAGE_HEIGHT - 1) as f64;
            ray_color(&cam.get_ray(u, v), world, MAX_DEPTH)
        })
        .fold(Color::default(), |sum, c| sum + c);
    //write_color(pixel_color, 20);
    let scale = 1.0 / 20.0 as f64;
    let get_color = |c| (255.999 * clamp(f64::sqrt(scale * c), 0.0, 0.999)) as u32;
    let r = get_color(pixel_color.x);
    let g = get_color(pixel_color.y);
    let b = get_color(pixel_color.z);
    Pixel {r, g, b}
}

pub struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 20;
const MAX_DEPTH: usize = 50;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //let world = read_image();
    let world = BvhNode::new(&mut prism().objects,0, 10);
    let lookfrom = Point3::from([8.0, 2.5, -5.0]);
    let lookat = Point3::from([0.0,0.5,0.0]);
    let vup = Vec3::from([0.0, 1.0, 0.0]);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        0.0,
    );

    let img: Vec<Pixel> = render(&cam, &world);
    img.iter().for_each(|it| {
        println!("{} {} {}", it.r, it.g, it.b);
    });

    eprintln!("\nDone.");
}