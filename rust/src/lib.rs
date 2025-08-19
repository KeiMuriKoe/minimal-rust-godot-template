use godot::prelude::*;
use godot::classes::Object;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

/// This is a simple Godot extension that defines a class with static and const methods.
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
    /// This is a static method that can be called without an instance of MyClass.
    fn my_static_method() {
        godot_print!("This is a static method in MyClass!");
    }

    
    #[func]
    /// This is a const method that can be called on an instance of MyClass.
    fn my_const_method(&self) {
        godot_print!("This is a const method in MyClass!");
    }
}