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
