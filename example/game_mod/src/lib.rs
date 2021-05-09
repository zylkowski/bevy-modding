#[macro_use]

use bevy::prelude::*;
use bevy_modding::*;

#[repr(C)]
struct State;

mod_loader!{
    host: Host;
    state: State;
    init: init;
}

fn hello_from_mod(){
    println!("Hello from mod!");
}

fn init(host: &mut Host, _state: &mut State) {
    host.app_builder
        .add_system(hello_from_mod.system());
}