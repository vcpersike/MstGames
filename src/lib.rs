<<<<<<< HEAD
use gdnative::methods::MethodDecorators;
=======
>>>>>>> 337618aebdff1ce282f9fb566e5e0929a69b181a
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
<<<<<<< HEAD
pub struct MstGames;

#[methods]
impl MstGames {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello, world!");
    }
}

fn mstgames_constructor(_owner: &Node) -> MstGames {
    MstGames
}

fn init(handle: InitHandle) {
    handle.add_class::<MstGames>();
=======
struct HelloWorld;

#[methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
>>>>>>> 337618aebdff1ce282f9fb566e5e0929a69b181a
}

godot_init!(init);
