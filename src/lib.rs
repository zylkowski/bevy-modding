use bevy::prelude::*;
use live_reload;
use walkdir::WalkDir;

/// Live reload api for communication between mod and main game
pub struct Host<'a> {
    pub app_builder: &'a mut AppBuilder,
}

type ModLoader<'a> = live_reload::Reloadable<Host<'a>>;
pub struct ModdingPlugin<'a> {
    mod_folder_path: &'a str,
}

impl<'a> ModdingPlugin<'a> {
    pub fn new(mod_folder_path: &'a str) -> Self {
        ModdingPlugin { mod_folder_path }
    }
}

impl Plugin for ModdingPlugin<'static> {
    fn build(&self, app: &mut AppBuilder) {
        for entry in WalkDir::new(self.mod_folder_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            println!("File: {}", f_name);
            if f_name.ends_with(".so"){
                let host_api = Host { app_builder: app };
                ModLoader::new(entry.path(),host_api);
            }
        }
    }
}

#[macro_export]
macro_rules! mod_loader {
    (host: $Host:ty;
     state: $State:ty;
     init: $init:ident;) => {
        fn cast<'a>(raw_state: *mut ()) -> &'a mut $State {
            unsafe { &mut *(raw_state as *mut $State) }
        }

        fn init_wrapper(host: &mut $Host, raw_state: *mut ()) {
            $init(host, cast(raw_state))
        }

        fn reload_wrapper(host: &mut $Host, raw_state: *mut ()) {}

        fn update_wrapper(host: &mut $Host, raw_state: *mut ()) -> ::live_reload::ShouldQuit {
            live_reload::ShouldQuit::No
        }

        fn unload_wrapper(host: &mut $Host, raw_state: *mut ()) {}

        fn deinit_wrapper(host: &mut $Host, raw_state: *mut ()) {}

        #[no_mangle]
        pub static RELOAD_API: ::live_reload::internals::ReloadApi<$Host> =
            ::live_reload::internals::ReloadApi {
                size: ::std::mem::size_of::<$State>,
                init: init_wrapper,
                reload: reload_wrapper,
                update: update_wrapper,
                unload: unload_wrapper,
                deinit: deinit_wrapper,
            };
    };
}
