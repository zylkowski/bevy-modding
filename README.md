# bevy-modding
Simple plugin enabling you to make your game moddable in Bevy engine

## Usage
1. Add plugin to your bevy app and specify mod folder path
2. Compile your mod to `.so` file(`crate-type = ["cdylib"]` flag in `Cargo.toml`) and place it in mod folder. For more details look into example
3. Run your game.

## Safety
For now this crate is just proof of concept and probably root of something bigger. For now loaded mods don't have any restrictions to what they can do, eg. they can read files from your computer, download files from malicious websites etc. That's something I will be targeting in future updates. I am relatively new to Rust and I don't really know any crates for sandboxing, will have to research that in order to make this plugin safe.

## Todo
- [X] Have Fun
- [x] Create simple working example
- [ ] Make documentation
