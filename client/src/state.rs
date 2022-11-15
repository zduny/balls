use std::collections::HashMap;

use common::Color;
use js_utils::{body, document};
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

#[derive(Debug)]
pub struct Ball {
    div: HtmlDivElement,
}

impl Ball {
    pub fn new(color: Color) -> Self {
        let document = document();
        let div = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        div.set_class_name("ball");
        div.style()
            .set_property("background-color", &color.into_style_value())
            .unwrap();

        let body = body();
        body.append_child(&div).unwrap();
        Ball { div }
    }

    pub fn set_position(&self, position: (i32, i32)) {
        let style = self.div.style();
        style
            .set_property("left", &format!("{}px", position.0))
            .unwrap();
        style
            .set_property("top", &format!("{}px", position.1))
            .unwrap();
    }
}

impl Drop for Ball {
    fn drop(&mut self) {
        self.div.remove();
    }
}

#[derive(Debug)]
pub struct State {
    pub my_ball: Ball,
    pub other_balls: HashMap<usize, Ball>,
}

impl State {
    pub fn new(my_color: Color) -> Self {
        State {
            my_ball: Ball::new(my_color),
            other_balls: HashMap::new(),
        }
    }
}
