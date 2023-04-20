mod server;

use std::ptr;
use obs_wrapper::wrapper::PtrWrapper;
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
    width: u32,
    height: u32,
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
        log!(Level::Info, "BLAH BLAH BLAHA BLAH BLAHLA BLAHALB LAHLABN LAHLAB LAHALAB BLAGH BALH BLALH BLAH BLAHL BLAH LBALH blah");

        let width = settings.get("width").unwrap_or(900);
        let height = settings.get("height").unwrap_or(100);

        let browser;

        let mut settings = DataObj::from_json(format!(r##"{{
            "width": {},
            "height": {},
            "x": 0,
            "y": 0,
            "is_local_file": true,
            "local_file": "C:/Users/beaux/Documents/doot/overlay/overlay/index.html"
        }}"##, width, height)).unwrap();

        unsafe {
            browser = obs_sys::obs_source_create_private(
                obs_string!("browser_source").as_ptr(),
                obs_string!("Browser Source").as_ptr(),
                settings.as_ptr_mut());

            let scene = obs_sys::obs_scene_from_source(obs_sys::obs_frontend_get_current_scene());
            obs_sys::obs_scene_add(scene, browser);
        }

        Self { _source, width, height, browser }
    }
}

impl GetWidthSource for KeyOverlaySrc {
    fn get_width(&mut self) -> u32 {
        self.width
    }
}

impl GetHeightSource for KeyOverlaySrc {
    fn get_height(&mut self) -> u32 {
        self.height
    }
}

impl GetNameSource for KeyOverlaySrc {
    fn get_name() -> ObsString {
        obs_string!("Keystroke Overlay")
    }
}

impl GetDefaultsSource for KeyOverlaySrc {
    fn get_defaults(settings: &mut DataObj) {
        settings.set_default::<u32>("width", 900u32);
        settings.set_default::<u32>("height", 100u32);
    }
}

impl GetPropertiesSource for KeyOverlaySrc {
    fn get_properties(&mut self) -> obs_wrapper::properties::Properties {
        let mut properties = Properties::new();
        properties
            .add(obs_string!("width"),
                 obs_string!("Width"),
                 NumberProp::new_int().with_range(1u32..=3840))
            .add(obs_string!("height"),
                 obs_string!("Height"),
                 NumberProp::new_int().with_range(1u32..=2160));

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
        let source = load_context.create_source_builder::<KeyOverlaySrc>()
                                 .enable_get_name()
                                 .enable_get_defaults()
                                 .enable_get_properties()
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

