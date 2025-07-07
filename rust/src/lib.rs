#![allow(warnings)]

use godot::prelude::*;
use godot::classes::*;
use godot::global::*;

struct FlappyCat;

#[gdextension]
unsafe impl ExtensionLibrary for FlappyCat{}

#[derive(GodotClass)]
#[class(base = StaticBody2D, init)]
struct Cat{
    base:Base<StaticBody2D>,

    #[init(val = 0.0)]
    speed:f64
}
#[godot_api]
impl IStaticBody2D for Cat{
    fn process(&mut self, delta:f64){
        let input:Gd<Input> = Input::singleton();
        
        if(
            input.is_physical_key_pressed(Key::UP) 
            || input.is_physical_key_pressed(Key::W)
        ){
            self.speed = -300.0;
        }
        self.speed += 1500.0 * delta;

        let speed:f64 = self.speed;
        
        if(
            self.base_mut()
                .move_and_collide(Vector2::new(0.0, speed as f32) * delta as f32)
                .is_some()
        ){
            self.speed = 0.0;
        }
    }
}
#[derive(GodotClass)]
#[class(base = StaticBody2D, init)]
struct Pipes{
    base:Base<StaticBody2D>,

    #[export]
    speed:f64
}
#[godot_api]
impl IStaticBody2D for Pipes{
    fn process(&mut self, delta:f64){
        let position:Vector2 = self.base().get_position();

        let speed:f64 = self.speed;

        self.base_mut()
            .set_position(
                position + Vector2::new(speed as f32, 0.0) * delta as f32
            );

        if(self.base().get_position().x < -100.0){
            self.base_mut().queue_free();
        }
    }
}
#[derive(GodotClass)]
#[class(base = Node, init)]
struct Main{
    base:Base<Node>,

    pipes:Gd<PackedScene>
}
impl Main{
    fn on_timer_timeout(&mut self){
        let mut pipes:Gd<Pipes> = self.pipes.instantiate()
            .unwrap()
            .cast();

        self.base_mut()
            .add_child(&pipes);

        let mut pipes_mut:GdMut<Pipes> = pipes.bind_mut();
        
        pipes_mut.speed = -200.0;

        pipes_mut.base_mut()
            .set_position(
                Vector2::new(
                    962.0, 
                    randf_range(0.0, 430.0) as f32
                )
            );
    }
}
#[godot_api]
impl INode for Main{
    fn ready(&mut self){
        self.pipes = load::<PackedScene>("res://pipes.scn");

        let timer:Gd<Timer> = self.base()
            .get_node_as("Timer");

        timer.signals()
            .timeout()
            .connect_other(self, Self::on_timer_timeout);
    }
}