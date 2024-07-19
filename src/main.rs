use clap::Parser;
use sms_bomber::api::{data, ApiData};
use sms_bomber::cli::App;
use sms_bomber::sms::sender::Attacker;

fn main() {
    let cli = App::parse();
    let noice_data = data::get();
    let filter_data = ApiData::filter_all(noice_data, cli.mobile);
    let attacker = Attacker::init(filter_data, cli);
    let _ = attacker.attack();
}
