use godot::prelude::*;
use godot::classes::Object;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Object)]
struct MyClass {
    base: Base<Object>,
}

#[godot_api]
impl IObject for MyClass {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MyClass {
    #[func]
    fn my_static_method() {
        godot_print!("This is a static method in MyClass!");
    }

    #[func]
    fn my_const_method(&self) {
        godot_print!("This is a const method in MyClass!");
    }
}