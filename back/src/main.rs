mod controller;

use controller::Controller;
use settings::Settings;

fn main() {
    let settings = Settings::load("../settings/settings.json");
    let crl = Controller::new(&settings);
    crl.run();
}
