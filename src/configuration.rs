use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub width: i64,
    pub height: i64,
    pub background_color: Vec<f64>,
    pub output_path: String,
    pub shapes: Vec<Shape>,
    pub light: Light,
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
    pub color: Vec<f64>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pub translation: Vec<f64>,
    pub scale: Vec<f64>,
    pub rotation: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    #[serde(rename = "type")]
    pub type_field: String,
    pub intensity: Vec<f64>,
    pub position: Vec<f64>,
}
