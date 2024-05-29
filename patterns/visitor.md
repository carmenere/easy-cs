# Visitor
A **visitor** pattern allows to **change behavior** of the **existing code without altering** any existing code. **Visitor emulates double dispatch**.<br>

There are 2 main of interfaces in the pattern:
- **interface of element**;
- **interface of visitor**;

<br>

The **common idea** is:
1. Define **interface of element**:
```rust
trait CarElement {
  fn accept(&self, visitor: &CarVisitor);
}
```
1. Define **interface of visitor**. Each **visitor's method** must deal with each **concrete type of element**.
```rust
trait CarVisitor {
  fn visit_car(&self, element: &Car);
  fn visit_body(&self, element: &Body);
  fn visit_engine(&self, element: &Engine);
  fn visit_wheel(&self, element: &Wheel);
}
```
1. Create **concrete elements**. Every **concrete element** must implement **interface of element**. All `accept()` method does is calls the appropriate method on visitor it has got and passes itself as the parameter.
```rust
struct Wheel {id: u64};
struct Engine {rid: u64};

impl CarElement for Engine {
  fn accept(&self, visitor: &CarVisitor) {
    visitor.visit_engine(self);
  }
}

impl CarElement for Wheel {
  fn accept(&self, visitor: &CarElementVisitor) {
    visitor.visit_wheel(self);
  }
}
```
4. Create **concrete visitor**:
```rust
struct PrintVisitor {};

impl CarVisitor for PrintVisitor {
  fn visit_engine(&self, element: &Engine) {
    println!("Engine id is {}", element.id);
  }
  fn visit_wheel(&self, element: &Wheel) {
    println!("Wheel id is {}", element.rid);
  }
}
```
5. Then we can call **appropriate methods of elements** passing them **visitor**. If we change **visitor**, we change **behavior**.
