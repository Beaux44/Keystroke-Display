mod server;

use server::start_server;
use log::*;
use obs_wrapper::{
    prelude::*,
    obs_register_module,
    obs_string, log::Logger,
};

struct KeyOverlayModule {
    context: ModuleContext,
}

impl Module for KeyOverlayModule {
    fn new(context: ModuleContext) -> Self {
        Logger::new().init().unwrap();
        log!(Level::Info, "Initializing Keystroke Overlay Module...");
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, _load_context: &mut LoadContext) -> bool {
        start_server();
        true
    }

    fn description() -> ObsString {
        obs_string!("Plugin that adds a keystroke overlay for programming.")
    }

    fn name() -> ObsString {
        obs_string!("Keystroke Overlay")
    }

    fn author() -> ObsString {
        obs_string!("Beaux Moreau")
    }
}

obs_register_module!(KeyOverlayModule);

