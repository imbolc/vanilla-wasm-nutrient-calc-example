#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, HtmlInputElement, InputEvent, NodeList};

fn main() {
    set_panic_hook();
    calculate_totals();
    set_evens();
}

fn calculate_totals() {
    let mut nutrient_weight = HashMap::new();
    for food in query_selector_all::<HtmlInputElement>("[data-nutrients]") {
        let food_weight: f32 = food.value().parse().unwrap();
        let dataset = food.get_attribute("data-nutrients").unwrap();

        for pair in dataset.split(',') {
            let (nutrient, weight100) = pair.split_once(':').unwrap();
            let weight100: f32 = weight100.parse().unwrap();
            let weight = food_weight * weight100;
            nutrient_weight
                .entry(nutrient.to_owned())
                .and_modify(|w| *w += weight)
                .or_insert(weight);
        }
    }

    for (nutrient, weight) in nutrient_weight {
        query_selector::<HtmlElement>(&format!("[data-totals='{}']", nutrient))
            .set_inner_text(&weight.to_string())
    }
}

fn set_evens() {
    let oninput = Closure::wrap(Box::new(move |event: InputEvent| {
        let Some(target) = event.target() else { return };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        if !input.has_attribute("data-nutrients") {
            return;
        }
        calculate_totals()
    }) as Box<dyn FnMut(_)>);
    document()
        .body()
        .unwrap()
        .add_event_listener_with_callback("input", oninput.as_ref().unchecked_ref())
        .unwrap();
    oninput.forget();
}

fn document() -> Document {
    window().and_then(|w| w.document()).expect("no document")
}

fn query_selector<T: wasm_bindgen::JsCast>(query: &str) -> T {
    document()
        .query_selector(query)
        .unwrap()
        .expect(&format!("no element '{query}'"))
        .dyn_into::<T>()
        .unwrap()
}

fn query_selector_all<T: wasm_bindgen::JsCast>(query: &str) -> Vec<T> {
    let node_list: NodeList = document().query_selector_all(query).unwrap();
    let mut elements = Vec::new();
    for i in 0..node_list.length() {
        if let Some(node) = node_list.item(i) {
            let element = node.dyn_into::<T>().unwrap();
            elements.push(element);
        }
    }
    elements
}
