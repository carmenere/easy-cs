use crate::gui::{Form, Button, Checkbox};

pub struct MacOSForm {}

pub struct MacOSButton {}

pub struct MacOSCheckBox {}

impl Form for MacOSForm {
    fn create_button(&self) -> Box<dyn Button> {
        println!("Сreate MacOS button.");
        Box::new(MacOSButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        println!("Сreate MacOS checkbox.");
        Box::new(MacOSCheckBox {})
    }
}

impl Button for MacOSButton {
    fn click(&self) {
        println!("Click MacOS button.")
    }
}

impl Checkbox for MacOSCheckBox {
    fn select(&self) {
        println!("Select MacOS button.")
    }
}
