use std::marker::PhantomData;
use std::collections::{HashMap};
use wasm_bindgen::prelude::*;

use crate::gl::GL;

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = "loadImage")]
    fn load_image(gl: &JsValue, texture: &JsValue, path: &JsValue);
}

pub struct Texture {
    pub texture: web_sys::WebGlTexture,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Asset<T>(u32, PhantomData<T>);

impl<T> Asset<T> {
    pub fn new(handle: u32) -> Asset<T> {
        Asset(handle, PhantomData)
    }
}

pub struct AssetCache {
    paths_cache: Vec<String>,
    next_texture_handle: u32,
    textures: HashMap<u32, Texture>,
}

impl AssetCache {
    pub fn new() -> AssetCache {
        AssetCache {
            paths_cache: Vec::new(),
            next_texture_handle: 0,
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(&self, handle: &Asset<Texture>) -> &Texture {
        self.textures.get(&handle.0).unwrap()
    }

    pub fn load_texture(&mut self, gl: &GL, name: String) {

        if let Some(_) = self.paths_cache.iter().find(|cached| *cached == &name) {
            return;
        }

        self.paths_cache.push(name.clone());

        let texture = gl.create_texture().unwrap();

        load_image(
            &JsValue::from(gl.clone()),
            &JsValue::from(texture.clone()),
            &wasm_bindgen::JsValue::from("assets/me.JPG"),
        );
        let asset = Texture {
            texture,
        };

        self.textures.insert(self.next_texture_handle, asset);
    }
}