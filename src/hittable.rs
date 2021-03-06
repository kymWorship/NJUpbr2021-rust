use crate::{
    material::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
    sphere::Sphere,
    sphere_blur::SphereBlur,
    triangle::Triangle,
    mesh::Mesh,
    cylinder::Cylinder,
    aabb::Aabb,
    bvh::BvhNode
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(material: Material) -> Self {
        Self {
            p: Point3::zero(),
            normal: Point3::zero(),
            material,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}

#[derive(Clone)]
pub enum Shape {
    Sphere(Box<Sphere>),
    SphereBlur(Box<SphereBlur>),
    Triangle(Box<Triangle>),
    Cylinder(Box<Cylinder>),
    Mesh(Box<Mesh>),
    BvhNode(Box<BvhNode>),
}

impl Shape {
    pub fn new_sphere(center: Point3, radius: f64, material: Material) -> Self {
        Shape::Sphere(Box::new(Sphere::new(center, radius, material)))
    }
    pub fn new_sphere_blur(center1: Point3, center2: Point3, radius: f64, material: Material, time1: f64, time2: f64) -> Self {
        Shape::SphereBlur(Box::new(SphereBlur::new(center1, center2, radius, material, time1, time2)))
    }
    pub fn new_triangle(a0: Point3, a1: Point3, a2: Point3, material: Material) -> Self {
        Shape::Triangle(Box::new(Triangle::new(a0, a1, a2, material)))
    }
    pub fn new_mesh(a0: Point3, a1: Point3, a2: Point3, n0:Point3, n1:Point3, n2: Point3, material: Material) -> Self {
        Shape::Mesh(Box::new(Mesh::new(a0, a1, a2, n0, n1, n2, material)))
    }
    pub fn new_cylinder(r: f64, d: f64, material: Material) -> Self {
        Shape::Cylinder(Box::new(Cylinder::new(r, d, material)))
    }
}

impl Hittable for Shape {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord
    ) -> bool {
        match self {
            Shape::Sphere(m) => m.hit(r, t_min, t_max, rec),
            Shape::SphereBlur(m) => m.hit(r, t_min, t_max, rec),
            Shape::Triangle(m) => m.hit(r, t_min, t_max, rec),
            Shape::Cylinder(m) => m.hit(r, t_min, t_max, rec),
            Shape::Mesh(m) => m.hit(r, t_min, t_max, rec),
            Shape::BvhNode(m) => m.hit(r, t_min, t_max, rec)
        }
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        bounding_box: &mut Aabb
    ) -> bool {
        match self {
            Shape::Sphere(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::SphereBlur(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::Triangle(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::Cylinder(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::Mesh(m) => m.bounding_box(time0, time1, bounding_box),
            Shape::BvhNode(m) => m.bounding_box(time0, time1, bounding_box)
        }
    }

}
