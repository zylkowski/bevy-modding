# bevy-modding
Simple plugin enabling you to make your game moddable in Bevy engine. Basically it loads native library and calls `init()` function. Modders have complete access to bevy's `AppBuilder` and can add systems, plugins, assets etc. Currently works only on linux.

## Usage
In the `main.rs` file add `ModdingPlugin` with the bevy `AppBulder`, remember to specify mods folder path.
Create separate crate for your mod and add
```
[lib]
crate-type = ["cdylib"]

[dependencies]
live-reload = "0.2.0"
bevy = "0.5.0"
bevy-modding = { version = "0.1.0", path = "path"}
```
To the `Cargo.toml` file of the mod. Then in order for your mod to be properly mod loaded you need to use macro that let's for communication between mod and main game. You do that like this:
```
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
```
Your init function can look like this:
```
fn hello_from_mod(){
    println!("Hello from mod!");
}

fn init(host: &mut Host, _state: &mut State) {
    host.app_builder
        .add_system(hello_from_mod.system());
}
```
Now if you compile both your mod and core of the game you have to move compiled `.so` file from mod and place it in your designated mods folder and run the core game. 

## Safety
For now this crate is just proof of concept and probably root of something bigger. For now loaded mods don't have any restrictions to what they can do, eg. they can read files from your computer, download files from malicious websites etc. That's something I will be targeting in future updates. I am relatively new to Rust and I don't really know any crates for sandboxing, will have to research that in order to make this plugin safe.

## Todo
- [X] Have Fun
- [x] Create simple working example
- [ ] Add Windows compatibility
- [ ] Make documentation
