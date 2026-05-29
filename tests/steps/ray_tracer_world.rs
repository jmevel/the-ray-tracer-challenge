use std::collections::HashMap;

use cucumber::World;
use the_ray_tracer_challenge::{Canvas, Intersection, Matrix, Ray, Sphere, Tuple};

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
pub struct RayTracerWorld {
    canvases: HashMap<String, Canvas>,
    tuples: HashMap<String, Tuple>,
    ppms: HashMap<String, String>,

    matrices2x2: HashMap<String, Matrix<2, 2>>,
    matrices3x3: HashMap<String, Matrix<3, 3>>,
    matrices4x4: HashMap<String, Matrix<4, 4>>,

    rays: HashMap<String, Ray>,

    spheres: HashMap<String, Sphere>,

    intersections: HashMap<String, Intersection>,
    intersections_collections: HashMap<String, Option<Vec<Intersection>>>,

    index: HashMap<String, Type>,
}

#[derive(Debug)]
pub enum Type {
    Canvas,
    Tuple,
    PPM,
    Matrix((usize, usize)),
    Ray,
    Sphere,
    Intersections,
    IntersectionsCollection,
}

impl RayTracerWorld {
    fn new() -> Self {
        let mut world = RayTracerWorld::default();
        world.add_matrix2x2("identity_matrix".to_string(), Matrix::identity_matrix());
        world.add_matrix3x3("identity_matrix".to_string(), Matrix::identity_matrix());
        world.add_matrix4x4("identity_matrix".to_string(), Matrix::identity_matrix());
        world
    }

    pub fn get_element_type(&self, name: &str) -> &Type {
        let element_type = self.index.get(name).expect("Element doesn't exist");
        element_type
    }

    pub fn add_tuple(&mut self, tuple_name: String, tuple: Tuple) {
        self.tuples.insert(tuple_name.clone(), tuple);
        self.index.insert(tuple_name, Type::Tuple);
    }

    pub fn get_tuple(&self, tuple: &str) -> &Tuple {
        self.tuples
            .get(tuple)
            .expect(format!("{tuple} does not exist").as_str())
    }

    pub fn add_canvas(&mut self, canvas_name: String, canvas: Canvas) {
        self.canvases.insert(canvas_name.clone(), canvas);
        self.index.insert(canvas_name, Type::Canvas);
    }

    pub fn get_canvas(&self, canvas: &str) -> &Canvas {
        self.canvases
            .get(canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }

    pub fn get_mut_canvas(&mut self, canvas: &str) -> &mut Canvas {
        self.canvases
            .get_mut(canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }

    pub fn add_ppm(&mut self, ppm_name: String, ppm: String) {
        self.ppms.insert(ppm_name.clone(), ppm);
        self.index.insert(ppm_name, Type::PPM);
    }

    pub fn get_ppm(&self, ppm: &str) -> &str {
        self.ppms
            .get(ppm)
            .expect(format!("{ppm} does not exist").as_str())
    }

    pub fn add_matrix2x2(&mut self, matrix_name: String, matrix: Matrix<2, 2>) {
        self.matrices2x2.insert(matrix_name.clone(), matrix);
        self.index.insert(matrix_name, Type::Matrix((2, 2)));
    }

    pub fn get_matrix2x2(&self, matrix: &str) -> &Matrix<2, 2> {
        self.matrices2x2
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn add_matrix3x3(&mut self, matrix_name: String, matrix: Matrix<3, 3>) {
        self.matrices3x3.insert(matrix_name.clone(), matrix);
        self.index.insert(matrix_name, Type::Matrix((3, 3)));
    }

    pub fn get_matrix3x3(&self, matrix: &str) -> &Matrix<3, 3> {
        self.matrices3x3
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn add_matrix4x4(&mut self, matrix_name: String, matrix: Matrix<4, 4>) {
        self.matrices4x4.insert(matrix_name.clone(), matrix);
        self.index.insert(matrix_name, Type::Matrix((4, 4)));
    }

    pub fn get_matrix4x4(&self, matrix: &str) -> &Matrix<4, 4> {
        self.matrices4x4
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn add_ray(&mut self, ray_name: String, ray: Ray) {
        self.rays.insert(ray_name.clone(), ray);
        self.index.insert(ray_name, Type::Ray);
    }

    pub fn get_ray(&self, ray: &str) -> &Ray {
        self.rays
            .get(ray)
            .expect(format!("{ray} does not exist").as_str())
    }

    pub fn add_sphere(&mut self, sphere_name: String, sphere: Sphere) {
        self.spheres.insert(sphere_name.clone(), sphere);
        self.index.insert(sphere_name, Type::Sphere);
    }

    pub fn get_sphere(&self, sphere: &str) -> &Sphere {
        self.spheres
            .get(sphere)
            .expect(format!("{sphere} does not exist").as_str())
    }

    pub fn add_intersection(&mut self, intersection_name: String, intersection: Intersection) {
        self.intersections
            .insert(intersection_name.clone(), intersection);
        self.index.insert(intersection_name, Type::Intersections);
    }

    pub fn get_intersection(&self, intersection: &str) -> &Intersection {
        self.intersections
            .get(intersection)
            .expect(format!("{intersection} does not exist").as_str())
    }

    pub fn add_intersections_collection(
        &mut self,
        intersection_collection_name: String,
        intersections: Option<Vec<Intersection>>,
    ) {
        self.intersections_collections
            .insert(intersection_collection_name.clone(), intersections);
        self.index
            .insert(intersection_collection_name, Type::IntersectionsCollection);
    }

    pub fn get_intersections_collection(
        &self,
        intersections_collection: &str,
    ) -> &Option<Vec<Intersection>> {
        self.intersections_collections
            .get(intersections_collection)
            .expect(format!("{intersections_collection} does not exist").as_str())
    }
}
