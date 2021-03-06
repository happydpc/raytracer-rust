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

#![cfg(target_arch = "wasm32")]

use crate::ray_algorithm::strategy::{RandomAntiAliasingRenderStrategy, StandardRenderStrategy};
use crate::ray_algorithm::AnyPixelRenderStrategy;
use crate::renderer::{render_scene, Pixel, RenderConfiguration};
use crate::result::Result;
use crate::scene::Scene;
use log::*;
use serde::de::Unexpected::Str;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    #[cfg(feature = "console_log")]
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[wasm_bindgen]
#[serde(default)]
#[derive(Serialize, Deserialize)]
pub struct JsConfig {
    pub canvas_width: u32,
    pub ray_number: u32,
    pub strategy: Strategy,
}

impl JsConfig {
    pub fn generate_strategy(&self) -> Box<dyn AnyPixelRenderStrategy> {
        match self.strategy {
            Strategy::Normal => Box::new(StandardRenderStrategy),
            Strategy::Random => Box::new(RandomAntiAliasingRenderStrategy {
                rays_per_pixel: self.ray_number,
            }),
        }
    }
}

impl Default for JsConfig {
    fn default() -> Self {
        JsConfig {
            canvas_width: 1024,
            ray_number: 50,
            strategy: Strategy::Normal,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "&str")]
#[serde(rename_all="snake_case")]
pub enum Strategy {
    Normal,
    Random,
}

impl TryFrom<&str> for Strategy {
    type Error = String;

    fn try_from(val: &str) -> std::result::Result<Self, Self::Error> {
        let result = match val {
            "random" => Strategy::Random,
            "normal" => Strategy::Normal,
            _ => return Err(String::from("Coud not convert rendering strategy value")),
        };
        Ok(result)
    }
}

#[wasm_bindgen]
pub struct Renderer {
    render_iterator: Box<dyn Iterator<Item = Result<Pixel>>>,
    img_buffer: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(
        scene_description: &str,
        js_config: JsValue,
    ) -> std::result::Result<Renderer, JsValue> {
        let scene = Scene::from_str(scene_description).map_err(|e| e.to_string())?;
        let js_config: JsConfig = js_config.into_serde().map_err(|e| e.to_string())?;
        let config = RenderConfiguration {
            canvas_width: js_config.canvas_width,
            canvas_height: (js_config.canvas_width as f64 / scene.camera.size_ratio()) as u32,
            render_strategy: js_config.generate_strategy(),
        };
        let width = config.canvas_width;
        let height = config.canvas_height;
        let img_buffer = vec![0; (config.canvas_width * config.canvas_height * 4) as usize];
        let render_iterator = Box::new(render_scene(scene, config, false).unwrap());
        Ok(Renderer {
            render_iterator,
            img_buffer,
            width,
            height,
        })
    }

    pub fn buffer_ptr(&self) -> *const u8 {
        self.img_buffer.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn next(&mut self) -> bool {
        match self.render_iterator.next() {
            None => false,
            Some(Ok(pixel)) => {
                let index = 4 * (pixel.x + pixel.y * self.width) as usize;
                self.img_buffer[index] = (pixel.color.red() * 255.0) as u8;
                self.img_buffer[index + 1] = (pixel.color.green() * 255.0) as u8;
                self.img_buffer[index + 2] = (pixel.color.blue() * 255.0) as u8;
                self.img_buffer[index + 3] = 0xFF;
                true
            }
            Some(Err(err)) => {
                warn!("{}", err);
                false
            }
        }
    }
}