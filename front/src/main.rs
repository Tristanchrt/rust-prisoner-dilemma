slint::include_modules!();

mod controller;

use controller::Controller;
use settings::Settings;

fn main() {    
    let settings = Settings::load("../settings/settings.json");
    let crl = Controller {
        settings,
    };
    crl.run();
}
