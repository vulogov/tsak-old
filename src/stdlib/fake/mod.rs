extern crate log;
extern crate fakeit;

use rhai::{Engine};
use rhai::plugin::*;


#[export_module]
pub mod fake_module {
    pub fn password() -> String {
        fakeit::password::generate(true, true, true, 12)
    }
    pub mod person {
        pub fn ssn() -> String {
        	format!("{:0<3}-{:0<2}-{:0<4}", fakeit::currency::price(0 as f64, 999 as f64) as u64, fakeit::currency::price(0 as f64, 99 as f64) as u64, fakeit::currency::price(0 as f64, 9999 as f64) as u64)
        }
        pub fn first() -> String {
        	fakeit::name::first()
        }
        pub fn last() -> String {
        	fakeit::name::last()
        }
        pub fn full() -> String {
        	fakeit::name::full()
        }
        pub fn phone() -> String {
        	fakeit::contact::phone_formatted()
        }
    }
    pub mod internet {
        pub fn email() -> String {
        	fakeit::contact::email()
        }
        pub fn username() -> String {
        	fakeit::internet::username()
        }
        pub fn domain() -> String {
        	fakeit::internet::domain_name()
        }
        pub fn ipv4() -> String {
        	fakeit::internet::ipv4_address()
        }
        pub fn ipv6() -> String {
        	fakeit::internet::ipv6_address()
        }
    }
    pub mod address {
        pub fn street() -> String {
        	fakeit::address::street()
        }
        pub fn city() -> String {
        	fakeit::address::city()
        }
        pub fn state() -> String {
        	fakeit::address::state_abr()
        }
        pub fn zip() -> String {
        	fakeit::address::zip()
        }
        pub fn country() -> String {
        	fakeit::address::country()
        }
    }
    pub mod cc {
        pub fn number() -> String {
        	fakeit::payment::credit_card_number()
        }
        pub fn expiration() -> String {
        	fakeit::payment::credit_card_exp()
        }
        pub fn cvv() -> String {
        	fakeit::payment::credit_card_cvv()
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::fake init");
    let module = exported_module!(fake_module);

    engine.register_static_module("fake", module.into());
}
