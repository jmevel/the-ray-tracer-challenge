use std::collections::HashMap;

use cucumber::World;
use the_ray_tracer_challenge::{
    Canvas, Color, Computations, Intersection, Intersections, Material, Matrix, Object, Point,
    PointLight, Ray, Sphere, Vector,
};

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
pub struct RayTracerWorld {
    // canvases: HashMap<String, Canvas>,
    // points: HashMap<String, Point>,
    // colors: HashMap<String, Color>,
    // vectors: HashMap<String, Vector>,
    // ppms: HashMap<String, String>,

    // matrices2x2: HashMap<String, Matrix<2, 2>>,
    // matrices3x3: HashMap<String, Matrix<3, 3>>,
    // matrices4x4: HashMap<String, Matrix<4, 4>>,

    // rays: HashMap<String, Ray>,

    // spheres: HashMap<String, Sphere>,

    // intersections: HashMap<String, Option<Intersection>>,
    // intersections_collections: HashMap<String, Option<Intersections>>,

    // point_lights: HashMap<String, PointLight>,

    // materials: HashMap<String, Material>,

    // world: the_ray_tracer_challenge::World,

    // computations: HashMap<String, OwnedComputations>,

    // index: HashMap<String, ElementType>,
    elements: HashMap<String, ElementType>,
}

#[derive(Debug)]
pub enum ElementType {
    Canvas(Canvas),
    Point(Point),
    Color(Color),
    Vector(Vector),
    PPM(String),
    Matrix2x2(Matrix<2, 2>),
    Matrix3x3(Matrix<3, 3>),
    Matrix4x4(Matrix<4, 4>),
    Ray(Ray),
    Sphere(Sphere),
    Intersection(Option<Intersection>),
    IntersectionsCollection(Option<Intersections>),
    PointLight(PointLight),
    Material(Material),
    World(the_ray_tracer_challenge::World),
    Computations(OwnedComputations),
}

impl RayTracerWorld {
    fn new() -> Self {
        let mut world = RayTracerWorld::default();
        world.add_matrix2x2("identity_matrix".to_string(), Matrix::identity_matrix());
        world.add_matrix3x3("identity_matrix".to_string(), Matrix::identity_matrix());
        world.add_matrix4x4("identity_matrix".to_string(), Matrix::identity_matrix());
        world
    }

    pub fn get_element(&self, element: &str) -> &ElementType {
        self.elements
            .get(element)
            .expect(format!("{element} does not exist").as_str())
    }

    pub fn add_point(&mut self, point_name: String, point: Point) {
        self.elements.insert(point_name, ElementType::Point(point));
    }

    pub fn get_point(&self, point: &str) -> &Point {
        let ElementType::Point(point) = self
            .elements
            .get(point)
            .expect(format!("{point} does not exist").as_str())
        else {
            panic!("{point} is not a point");
        };
        point
    }

    pub fn add_color(&mut self, color_name: String, color: Color) {
        self.elements.insert(color_name, ElementType::Color(color));
    }

    pub fn get_color(&self, color: &str) -> &Color {
        let ElementType::Color(color) = self
            .elements
            .get(color)
            .expect(format!("{color} does not exist").as_str())
        else {
            panic!("{color} is not a color");
        };
        color
    }

    pub fn add_vector(&mut self, vector_name: String, vector: Vector) {
        self.elements
            .insert(vector_name, ElementType::Vector(vector));
    }

    pub fn get_vector(&self, vector: &str) -> &Vector {
        let ElementType::Vector(vector) = self
            .elements
            .get(vector)
            .expect(format!("{vector} does not exist").as_str())
        else {
            panic!("{vector} is not a vector");
        };
        vector
    }

    pub fn add_canvas(&mut self, canvas_name: String, canvas: Canvas) {
        self.elements
            .insert(canvas_name, ElementType::Canvas(canvas));
    }

    pub fn get_canvas(&self, canvas: &str) -> &Canvas {
        let ElementType::Canvas(canvas) = self
            .elements
            .get(canvas)
            .expect(format!("{canvas} does not exist").as_str())
        else {
            panic!("{canvas} is not a canvas");
        };
        canvas
    }

    pub fn get_mut_canvas(&mut self, canvas: &str) -> &mut Canvas {
        let ElementType::Canvas(canvas) = self
            .elements
            .get_mut(canvas)
            .expect(format!("{canvas} does not exist").as_str())
        else {
            panic!("{canvas} is not a canvas");
        };
        canvas
    }

    pub fn add_ppm(&mut self, ppm_name: String, ppm: String) {
        self.elements.insert(ppm_name, ElementType::PPM(ppm));
    }

    pub fn get_ppm(&self, ppm: &str) -> &str {
        let ElementType::PPM(ppm) = self
            .elements
            .get(ppm)
            .expect(format!("{ppm} does not exist").as_str())
        else {
            panic!("{ppm} is not a ppm");
        };
        ppm
    }

    pub fn add_matrix2x2(&mut self, matrix_name: String, matrix: Matrix<2, 2>) {
        self.elements
            .insert(matrix_name, ElementType::Matrix2x2(matrix));
    }

    pub fn add_matrix3x3(&mut self, matrix_name: String, matrix: Matrix<3, 3>) {
        self.elements
            .insert(matrix_name, ElementType::Matrix3x3(matrix));
    }

    pub fn get_matrix3x3(&self, matrix: &str) -> &Matrix<3, 3> {
        let ElementType::Matrix3x3(matrix) = self
            .elements
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
        else {
            panic!("{matrix} is not a 3x3 matrix");
        };
        matrix
    }

    pub fn add_matrix4x4(&mut self, matrix_name: String, matrix: Matrix<4, 4>) {
        self.elements
            .insert(matrix_name, ElementType::Matrix4x4(matrix));
    }

    pub fn get_matrix4x4(&self, matrix: &str) -> &Matrix<4, 4> {
        let ElementType::Matrix4x4(matrix) = self
            .elements
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
        else {
            panic!("{matrix} is not a 4x4 matrix");
        };
        matrix
    }

    pub fn add_ray(&mut self, ray_name: String, ray: Ray) {
        self.elements.insert(ray_name, ElementType::Ray(ray));
    }

    pub fn get_ray(&self, ray: &str) -> &Ray {
        let ElementType::Ray(ray) = self
            .elements
            .get(ray)
            .expect(format!("{ray} does not exist").as_str())
        else {
            panic!("{ray} is not a ray");
        };
        ray
    }

    pub fn add_sphere(&mut self, sphere_name: String, sphere: Sphere) {
        self.elements
            .insert(sphere_name, ElementType::Sphere(sphere));
    }

    pub fn get_sphere(&self, sphere: &str) -> &Sphere {
        let ElementType::Sphere(sphere) = self
            .elements
            .get(sphere)
            .expect(format!("{sphere} does not exist").as_str())
        else {
            panic!("{sphere} is not a sphere");
        };
        sphere
    }

    pub fn get_mut_sphere(&mut self, sphere: &str) -> &mut Sphere {
        let ElementType::Sphere(sphere) = self
            .elements
            .get_mut(sphere)
            .expect(format!("{sphere} does not exist").as_str())
        else {
            panic!("{sphere} is not a sphere");
        };
        sphere
    }

    pub fn add_intersection(
        &mut self,
        intersection_name: String,
        intersection: Option<Intersection>,
    ) {
        self.elements
            .insert(intersection_name, ElementType::Intersection(intersection));
    }

    pub fn get_intersection(&self, intersection: &str) -> &Option<Intersection> {
        let ElementType::Intersection(intersection) = self
            .elements
            .get(intersection)
            .expect(format!("{intersection} does not exist").as_str())
        else {
            panic!("{intersection} is not a intersection");
        };
        intersection
    }

    pub fn add_intersections_collection(
        &mut self,
        intersection_collection_name: String,
        intersections: Option<Intersections>,
    ) {
        self.elements.insert(
            intersection_collection_name,
            ElementType::IntersectionsCollection(intersections),
        );
    }

    pub fn get_intersections_collection(
        &self,
        intersections_collection: &str,
    ) -> &Option<Intersections> {
        let ElementType::IntersectionsCollection(intersections_collection) = self
            .elements
            .get(intersections_collection)
            .expect(format!("{intersections_collection} does not exist").as_str())
        else {
            panic!("{intersections_collection} is not a intersections_collection");
        };
        intersections_collection
    }

    pub fn add_point_light(&mut self, point_light_name: String, point_light: PointLight) {
        self.elements
            .insert(point_light_name, ElementType::PointLight(point_light));
    }

    pub fn get_point_light(&self, point_light: &str) -> &PointLight {
        let ElementType::PointLight(point_light) = self
            .elements
            .get(point_light)
            .expect(format!("{point_light} does not exist").as_str())
        else {
            panic!("{point_light} is not a point light");
        };
        point_light
    }

    pub fn add_material(&mut self, material_name: String, material: Material) {
        self.elements
            .insert(material_name, ElementType::Material(material));
    }

    pub fn get_material(&self, material: &str) -> &Material {
        let ElementType::Material(material) = self
            .elements
            .get(material)
            .expect(format!("{material} does not exist").as_str())
        else {
            panic!("{material} is not a material");
        };
        material
    }

    pub fn get_mut_material(&mut self, material: &str) -> &mut Material {
        let ElementType::Material(material) = self
            .elements
            .get_mut(material)
            .expect(format!("{material} does not exist").as_str())
        else {
            panic!("{material} is not a material");
        };
        material
    }

    pub fn add_world(&mut self, world_name: String, world: the_ray_tracer_challenge::World) {
        self.elements.insert(world_name, ElementType::World(world));
    }

    pub fn get_world(&self, world: &str) -> &the_ray_tracer_challenge::World {
        let ElementType::World(world) = self
            .elements
            .get(world)
            .expect(format!("{world} does not exist").as_str())
        else {
            panic!("{world} is not a world");
        };
        world
    }

    pub fn add_computations(&mut self, computations_name: String, computations: Computations) {
        self.elements.insert(
            computations_name,
            ElementType::Computations(OwnedComputations::from(computations)),
        );
    }

    pub fn get_computations(&self, computation: &str) -> Computations<'_> {
        let ElementType::Computations(computation) = self
            .elements
            .get(computation)
            .expect(format!("{computation} does not exist").as_str())
        else {
            panic!("{computation} is not a computation");
        };
        computation.as_computations()
    }
}

#[derive(Debug)]
pub struct OwnedComputations {
    t: f32,
    object: Object,
    point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
}

impl From<Computations<'_>> for OwnedComputations {
    fn from(value: Computations<'_>) -> Self {
        Self {
            t: *value.t(),
            object: *value.object(),
            point: value.point(),
            eye_vector: value.eye_vector(),
            normal_vector: value.normal_vector(),
        }
    }
}

impl OwnedComputations {
    pub fn as_computations(&self) -> Computations<'_> {
        Computations {
            t: &self.t,
            object: &self.object,
            point: self.point,
            eye_vector: self.eye_vector,
            normal_vector: self.normal_vector,
        }
    }
}
