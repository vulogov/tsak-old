extern crate log;
use rhai::{Engine, Module};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use lazy_static::lazy_static;
use std::sync::Mutex;

mod detect;

lazy_static! {
    static ref LANG_DETECTOR: Mutex<NRLanguage> = {
        let e: Mutex<NRLanguage> = Mutex::new(NRLanguage::init());
        e
    };
}

pub struct NRLanguage {
    d: LanguageDetector,
}

impl NRLanguage {
    fn new() -> Self {
        Self {
            d: LanguageDetectorBuilder::from_all_languages().with_preloaded_language_models().build(),
        }
    }
    pub fn init() -> NRLanguage {
        let res = NRLanguage::new();
        res
    }
}

pub fn languages_preload() {
    log::trace!("Pre-loading languages");
    let e = LANG_DETECTOR.lock().unwrap();
    drop(e);
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::linguistic init");

    let mut module = Module::new();
    module.set_native_fn("detect", detect::detect_language);
    engine.register_static_module("linguistic", module.into());
}
