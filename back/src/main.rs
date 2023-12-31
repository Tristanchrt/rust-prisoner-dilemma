mod controller;

use controller::Controller;
use settings::Settings;

fn main() {
    let settings = Settings::load("../settings/settings.json");
    let mut crl = Controller::new(&settings);
    crl.run();
}
