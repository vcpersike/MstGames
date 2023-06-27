use gdnative::methods::MethodDecorators;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
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
}

godot_init!(init);
