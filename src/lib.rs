mod server;

use server::start_server;
use log::*;
use obs_wrapper::obs_sys;
use obs_wrapper::{
    prelude::*,
    source::*,
    obs_register_module,
    obs_string, log::Logger, properties::{NumberProp, Properties},
};

struct KeyOverlayModule {
    context: ModuleContext,
}

struct KeyOverlaySrc {
    _source: SourceContext,
    browser: *mut obs_sys::obs_source_t,
}

impl Sourceable for KeyOverlaySrc {
    fn get_id() -> ObsString {
        obs_string!("key_overlay")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        let settings = &mut create.settings;

        let width = settings.get("browser_width").unwrap_or(900);
        let height = settings.get("browser_height").unwrap_or(100);

        let x = settings.get("browser_x").unwrap_or(0);
        let y = settings.get("browser_y").unwrap_or(0);

        let browser;

        unsafe {
            let settings = obs_sys::obs_data_create();
            obs_sys::obs_data_set_int(settings, obs_string!("width").as_ptr(), width as i64);
            obs_sys::obs_data_set_int(settings, obs_string!("height").as_ptr(), height as i64);
            obs_sys::obs_data_set_int(settings, obs_string!("x").as_ptr(), x as i64);
            obs_sys::obs_data_set_int(settings, obs_string!("y").as_ptr(), y as i64);
            obs_sys::obs_data_set_bool(settings, obs_string!("is_local_file").as_ptr(), true);
            obs_sys::obs_data_set_string(settings,
                                         obs_string!("local_file").as_ptr(),
                                         obs_string!("C:/Users/beaux/Documents/doot/overlay/overlay/index.html").as_ptr());

            browser = obs_sys::obs_source_create_private(
                obs_string!("browser_source").as_ptr(),
                obs_string!("Browser Source").as_ptr(),
                settings);
        }

        Self { _source, browser }
    }
}

impl GetNameSource for KeyOverlaySrc {
    fn get_name() -> ObsString {
        obs_string!("Keystroke Overlay")
    }
}

impl GetPropertiesSource for KeyOverlaySrc {
    fn get_properties(&mut self) -> obs_wrapper::properties::Properties {
        let mut properties = Properties::new();
        properties
            .add(obs_string!("browser_width"),
                 obs_string!("Width"),
                 NumberProp::new_int().with_range(1u32..=3840))
            .add(obs_string!("browser_height"),
                 obs_string!("Height"),
                 NumberProp::new_int().with_range(1u32..=2160))
            .add(obs_string!("browser_x"),
                 obs_string!("X"),
                 NumberProp::new_int().with_range(0u32..=3840))
            .add(obs_string!("browser_y"),
                 obs_string!("Y"),
                 NumberProp::new_int().with_range(0u32..=2160));

        properties
    }
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

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        // Create the source
        let source = load_context.create_source_builder::<KeyOverlaySrc>()
                                 .enable_get_name()
                                 .with_icon(Icon::Text)
                                 .build();

        // log!(Level::Info, "Registering Keystroke Source...");
        /* Source is not yet ready */
        // load_context.register_source(source);

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

