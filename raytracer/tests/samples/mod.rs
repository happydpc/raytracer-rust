/*
MIT License

Copyright (c) 2019, 2020 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use raytracer::cameras::PerspectiveCamera;
use raytracer::colors::Color;
use raytracer::lights::LightPoint;
use raytracer::primitives::Sphere;
use raytracer::scene::{Scene, SceneObject};
use raytracer::textures::CheckedPattern;
use raytracer::vector::Vec3;
use std::f64::consts::PI;
use std::fs;
use std::iter;
use std::path::PathBuf;



pub enum SampleScene {
    OkBasic
}

impl SampleScene {
    const SAMPLES_ROOT_DIR: [&'static str; 2] = ["tests", "samples"];

    pub fn to_string(&self) -> String {
        let sample = match self {
            SampleScene::OkBasic => "ok_basic.toml",
        };
        Self::load_sample_file(sample)
    }

    fn load_sample_file<T: AsRef<str>>(sample_file: T) -> String {
        let sample_file = sample_file.as_ref();
        let scene_path: PathBuf = SampleScene::SAMPLES_ROOT_DIR
            .iter()
            .chain(iter::once(&sample_file))
            .collect();
        fs::read_to_string(scene_path).unwrap()
    }

}





pub fn generate_test_scene() -> Scene {
    let camera = PerspectiveCamera::new(
        Vec3::new(0.0, 10.0, -10.0),
        Vec3::new(0.0, 0.0, 30.0),
        16.0 * 2.0,
        9.0 * 2.0,
        (PI / 8.0) as f64,
    );
    let light = LightPoint::with_color(Vec3::new(50.0, 100.0, -50.0), Color::new(0.8, 0.8, 0.8));
    let primitive: Sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5.0,
    };
    let texture = <CheckedPattern as Default>::default();
    let object = SceneObject {
        shape: Box::new(primitive),
        texture: Box::new(texture),
        effects: Default::default(),
    };
    Scene {
        camera: Box::new(camera),
        lights: vec![Box::new(light)],
        objects: vec![object],
        config: Default::default(),
    }
}
