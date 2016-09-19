/*!
    A simple button
*/
use std::hash::Hash;
use controls::ControlTemplate;
use controls::base::{WindowBase, create_base};
use winapi::HWND;

/**
    Configuration properties to create simple button

    * text: The button text
    * size: The button size (width, height) in pixels
    * position: The button position (x, y) in the parent control
    * parent: The control parent
*/
pub struct Button<ID: Eq+Clone+Hash> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Button<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let base = WindowBase::<ID> {
            text: self.text.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            class: Some("BUTTON".to_string()),
            parent: Some(self.parent.clone())
        };

        unsafe { create_base::<ID>(ui, base) }
    }

}