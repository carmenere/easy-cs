# Builder
**Builder** is a **creational** design pattern that lets you **construct complex objects step by step** and provide a method that will actually return the **final object**.<br>

<br>

# Example
```rust
#[derive(Debug, PartialEq)]
pub struct Foo {
    name: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::new()
    }
}

pub struct FooBuilder {
    foo: Foo
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        FooBuilder {
            foo: Foo {
                name: String::from("ABC")
            }
        }
    }

    pub fn name(mut self, name: String) -> FooBuilder {
        self.foo.name = name;
        self
    }

    pub fn build(self) -> Foo {
        Foo { name: self.foo.name }
    }
}

fn main() {

}

#[test]
fn builder_test() {
    let foo = Foo {
        name: String::from("qwerty"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("qwerty")).build();
    assert_eq!(foo, foo_from_builder);
}
```