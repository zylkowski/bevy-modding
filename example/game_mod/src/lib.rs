#[macro_use]            // Let's you use macro from other crate

use bevy::prelude::*;
use bevy_modding::*;

#[repr(C)]
struct State;           // State struct, it's needed by crate that loads your mod

mod_loader!{
    host: Host;
    state: State;
    init: init;         // Passing initialize function for your mod
}

fn hello_from_mod(){
    println!("Hello from mod!");
}

fn init(host: &mut Host, _state: &mut State) {
    host.app_builder
        .add_system(hello_from_mod.system());
}