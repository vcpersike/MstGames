use gdnative::api::KinematicBody2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property(default = 100.0)]
    speed: f32,
    velocity: Vector2,
}

impl Player {
    fn new(_base: &KinematicBody2D) -> Self {
        Player {
            speed: 100.0,
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}

#[methods]
impl Player {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, _delta: f64) {
        let input = Input::godot_singleton();
        let mut direction = Vector2::new(0.0, 0.0);

        direction.x = (Input::get_action_strength(&input, "ui_right", false)
            - Input::get_action_strength(&input, "ui_left", false)) as f32;
        direction.y = (Input::get_action_strength(&input, "ui_down", false)
            - Input::get_action_strength(&input, "ui_up", false)) as f32;

        if direction.length() > 0.0 {
            direction = direction.normalized();
            self.velocity = direction * self.speed;
        } else {
            self.velocity = Vector2::new(0.0, 0.0);
        }

        owner.move_and_slide(
            self.velocity,
            Vector2::new(0.0, -1.0),
            false,
            4,
            0.785398,
            true,
        );
    }
}

struct Library;

#[gdnative::init::callbacks]
impl GDNativeCallbacks for Library {
    fn nativescript_init(handle: InitHandle) {
        handle.add_class::<Player>();
    }
}
