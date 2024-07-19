use clap::Parser;
use sms_bomber::api::{data, ApiData};
use sms_bomber::cli::App;
use sms_bomber::sms::sender::Attacker;

fn main() {
    let cli = App::parse();
    if cli.uninstall {
        let r = cli.remove_self();
        if r.is_ok() {
            println!("Uninstall success....");
            std::process::exit(0);
        } else {
            eprintln!("Faild to uninstall....");
            std::process::exit(1);
        }
    };
    if cli.mobile.is_none() {
        return;
    };
    let noice_data = data::get();
    let filter_data = ApiData::filter_all(noice_data, cli.mobile.unwrap());
    let attacker = Attacker::init(filter_data, cli);
    let _ = attacker.attack();
}
