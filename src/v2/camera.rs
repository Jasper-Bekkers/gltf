
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{Extensions, Extras};

// TODO: This implementation is rubbish. Replace with enum instead
// and derive (De)Serialize manually. It would be trivial to do so
// if it were not for the `name`, `extension`, and `extra` fields.
/// A camera's projection
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Camera<E: Extensions, X: Extras> {
    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// Orthographic camera values
    pub orthographic: Option<Orthographic<E, X>>,
    
    /// Perspective camera values
    pub perspective: Option<Perspective<E, X>>,
    
    /// `"perspective"` or `"orthographic"`
    #[serde(rename = "type")]
    pub ty: String,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::Camera,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::Camera,
}

/// Values for an orthographic camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Orthographic<E: Extensions, X: Extras> {
    /// The horizontal magnification of the view
    #[serde(default, rename = "xmag")]
    pub x_mag: f32,
    
    /// The vertical magnification of the view
    #[serde(default, rename = "ymag")]
    pub y_mag: f32,
    
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::CameraOrthographic,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::CameraOrthographic,
}

/// Values for a perspective camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Perspective<E: Extensions, X: Extras> {
    /// Aspect ratio of the field of view
    #[serde(default, rename = "aspectRatio")]
    pub aspect_ratio: f32,
    
    /// The vertical field of view in radians
    #[serde(default, rename = "yfov")]
    pub y_fov: f32,
    
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::CameraPerspective,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::CameraPerspective,
}
