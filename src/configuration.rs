use crate::defs::Real;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub width: i64,
    pub aspect_ratio: Real,
    pub output_path: String,
    pub ray_step: i64,
    pub samples_per_pixel: i64,
    pub ray_max_depth: i64,
    pub camera: Camera,
    pub shapes: Vec<Shape>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub fov_deg: Real,
    pub position: Vec<Real>,
    pub look_at: Vec<Real>,
    pub up: Vec<Real>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shape {
    #[serde(rename = "type")]
    pub type_field: String,
    pub material: Material,
    pub transform: Transform,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    #[serde(rename = "type")]
    pub type_field: String,
    pub diffuse: Option<Vec<Real>>,
    pub fuzz: Option<Real>,
    pub refraction_index: Option<Real>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pub position: Vec<Real>,
    pub size: Vec<Real>,
}
