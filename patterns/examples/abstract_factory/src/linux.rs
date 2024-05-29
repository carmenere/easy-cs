use crate::gui::{Form, Button, Checkbox};

pub struct LinuxForm {}

pub struct LinuxButton {}

pub struct LinuxCheckBox {}

impl LinuxForm {
    pub fn new() -> Self {
        Self {}
    }
}

/// `FormFactory` is an `AbstractFactory`.
impl Form for LinuxForm {
    fn create_button(&self) -> Box<dyn Button> {
        println!("Сreate Linux button.");
        Box::new(LinuxButton { })
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        println!("Сreate Linux checkbox.");
        Box::new(LinuxCheckBox {})
    }
}

impl Button for LinuxButton {
    fn click(&self) {
        println!("Click Linux button.")
    }
}

impl Checkbox for LinuxCheckBox {
    fn select(&self) {
        println!("Select Linux button.")
    }
}