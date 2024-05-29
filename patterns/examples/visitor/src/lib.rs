pub trait Animal {
    fn run(&self, visitor: &impl Visitor) {

    }
}

pub struct Dog;
pub struct Cat;

impl Animal for Cat {
    fn run(&self, visitor: &impl Visitor) {
        visitor.run_cat(&self);
    }
}

impl Animal for Dog {
    fn run(&self, visitor: &impl Visitor) {
        visitor.run_dog(&self);
    }
}

pub trait Visitor {
    fn run_dog(&self, d: &Dog);
    fn run_cat(&self, c: &Cat);
}

pub struct DefaultVisitor;

impl Visitor for DefaultVisitor {
    fn run_dog(&self, d: &Dog) {
        println!("DefaultVisitor for Dog.")
    }

    fn run_cat(&self, c: &Cat) {
        println!("DefaultVisitor for Cat.")
    }
}