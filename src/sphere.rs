use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{dot, Point3},
    aabb::Aabb
};
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let root = f64::sqrt(discriminant);
        let mut temp = (-half_b - root) / a;
        if temp > t_max || temp < t_min {
            temp = (-half_b + root) / a;
            if temp > t_max || temp < t_min {
                return false;
            }
        }

        rec.t = temp;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        return true;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let r = Point3::from([self.radius, self.radius, self.radius]);
        output_box.modify(self.center-r, self.center+r);
        return true
    }

    fn pdf_value(&self, origin: Point3, direction: Point3, time: f64) -> f64 {
        let mut rec = HitRecord::new(Material::new_lambertian(Point3::zero()));
        if !self.hit(&Ray::new(origin, direction, time), 0.001, INFINITY, &mut rec) {
            0.0
        } else {
            let cos_theta_max = (1 - self.radius * self.radius / (origin - self.center).length_squared()).sqrt();
            return 1 / (2.0 * PI * (1 - cos_theta_max))
        }
    }

}
