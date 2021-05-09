use bevy::prelude::*;
use bevy_modding::*;

fn main() {
    App::build()
        .add_plugin(ModdingPlugin::new( "target/debug/mods"))
        .run();
}
