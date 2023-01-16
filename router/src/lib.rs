#![allow(non_snake_case)]
use std::collections::hash_map::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Path {
    Static,
    CatchOne,
    CatchAll,
    End,
}

pub struct Route {
    pub path: Path,
    pub id: usize,
}

#[wasm_bindgen]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    fn new() -> Self {
        Router { routes: Vec::new() }
    }
}

#[wasm_bindgen]
impl Router {
    pub fn add(&mut self, path: Path) -> usize {
        let num = self.routes.len();
        let newRoute = Route {
            id: num.into(),
            path: path,
        };

        self.routes.push(newRoute);
        num
    }
}

#[wasm_bindgen]
pub fn createRouter() -> Router {
    Router::default()
}
