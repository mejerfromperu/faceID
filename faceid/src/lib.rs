use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use std::rc::Rc;
use std::cell::RefCell;
use std::result;
use web_sys::{FileReader, HtmlElement, Event, DragEvent};


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn setup_drag_and_drop(drop_zone_id: &str, output_id: &str) {
    let window = web_sys::window().expect("No window found");
    let document = window.document().expect("No document found");

    let drop_zone = document.get_element_by_id(drop_zone_id)
        .expect("Drop zone not found")
        .dyn_into::<HtmlElement>()
        .expect("Could not cast to HtmlElement");

    let output = document.get_element_by_id(output_id)
        .expect("Output not found");
    let output = Rc::new(RefCell::new(output));

    let closure = {
        let output = Rc::clone(&output);

        Closure::wrap(Box::new(move |event: DragEvent| {
            event.prevent_default();
            if let Some(data_transfer) = event.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if let Some(file) = files.item(0) {
                        let file_name = file.name();
                        let file_size = file.size();
                        let reader = FileReader::new().expect("Failed to create FileReader");
                        let reader_clone = reader.clone();

                        let output_clone = Rc::clone(&output);

                        let onload_callback = Closure::wrap(Box::new(move |_event: Event| {
                            if let Ok(result) = reader_clone.result(){
                                if let Some(text) = result.as_string() {

                                    let first_line = text.lines().next().unwrap_or("").to_string();

                                    output_clone.borrow().set_inner_html(&format!("<p>File: {} ({} bytes)</p><p>First Line: {}</p>",
                                        file_name, file_size, first_line));

                                        if let Some(window) = web_sys::window() {
                                            let js_function = js_sys::Function::new_no_args(
                                                &format!("window.handleFirstLine('{}')", first_line.replace("'", "\\'"))
                                            );
                                            window.set_timeout_with_callback(&js_function).expect("Failed to call JS");
                                        }

                                }

                            }
                                
                            let result = reader_clone.result().unwrap();
                            let url = result.as_string().unwrap();
                            
                            output_clone.borrow().set_inner_html(&format!(
                                "<p>File: {} ({} bytes)</p><img src='{}' style='max-width: 200px;'/>",
                                file_name, file_size, url
                            ));
                        }) as Box<dyn FnMut(_)>);

                        reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).expect("Failed to read file");

                        onload_callback.forget();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>)
    };

    drop_zone.set_ondrop(Some(closure.as_ref().unchecked_ref()));
    drop_zone.set_ondragover(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}

// counter function
#[wasm_bindgen]
pub fn setup_click_counter(button_id: &str, counter_id: &str) {
    let window = web_sys::window().expect("No window found");
    let document = window.document().expect("No document found");

    let button = document
        .get_element_by_id(button_id).expect("Button not found").dyn_into::<HtmlElement>().expect("Could not cast to HtmlElement");

    let counter = document.get_element_by_id(counter_id).expect("Counter not found").dyn_into::<HtmlElement>().expect("Could not cast to HtmlElement");

    let count = Rc::new(RefCell::new(0));

    let click_handler = {
        let count = Rc::clone(&count);
        let counter = counter.clone();
        Closure::wrap(Box::new(move |_: MouseEvent| {
            let mut count_ref = count.borrow_mut();
            *count_ref += 1;
            counter.set_inner_text(&format!("Clicks: {}", *count_ref));
        }) as Box<dyn FnMut(_)>)
    };

    button.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();
}
