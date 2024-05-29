pub trait Form {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

pub trait Button {
    fn click(&self);
}

pub trait Checkbox {
    fn select(&self);
}