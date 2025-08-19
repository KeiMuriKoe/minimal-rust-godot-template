use godot::prelude::*;
use godot::classes::Object;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Object)]
struct MyStaticClass {
    base: Base<Object>,
}

#[godot_api]
impl IObject for MyStaticClass {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MyStaticClass {
    #[func]
    fn my_static_method(&self) {
        godot_print!("This is a static method in MyStaticClass!");
    }
}