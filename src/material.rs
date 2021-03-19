use crate::{
    hittable::HitRecord,
    random_double,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color},
};

pub trait Scatter {
    fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn new_dielectric(ref_idx: f64) -> Self {
        Material::Dielectric(Dielectric::new(ref_idx))
    }
}

impl Scatter for Material {
    fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(m) => m.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(m) => m.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction, 0.0);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), 0.0);
        *attenuation = self.albedo;
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Scatter for Dielectric {
    fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));
        *scattered = if etai_over_etat * sin_theta > 1.0
            || random_double!() < schlick(cos_theta, etai_over_etat)
        {
            let reflected = reflect(unit_direction, rec.normal);
            Ray::new(rec.p, reflected, 0.0)
        } else {
            let refracted = refract(unit_direction, rec.normal, etai_over_etat);
            Ray::new(rec.p, refracted, 0.0)
        };
        true
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}