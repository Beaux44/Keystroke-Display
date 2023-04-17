mod server;

use server::start_server;
use obs_wrapper::{
    prelude::*,
    source::*,
    obs_register_module,
    obs_string,
};

struct KeyOverlayModule {
    context: ModuleContext,
}

struct KeyOverlaySrc {
    source: SourceContext,

}

impl Sourceable for KeyOverlaySrc {
    fn get_id() -> ObsString {
        obs_string!("test_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(_create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        todo!()
    }
}

impl GetNameSource for KeyOverlaySrc {
    fn get_name() -> ObsString {
        obs_string!("Keystroke Overlay")
    }
}

impl Module for KeyOverlayModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    // Load the module - create all sources, returning true if all went well.
    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        // Create the source
        let source = load_context.create_source_builder::<KeyOverlaySrc>()
                                 .enable_get_name()
                                 .with_icon(Icon::Text)
                                 .build();

        load_context.register_source(source);

        true
    }

    fn post_load(&mut self) {
        start_server();
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

