use clap::Parser;
use sms_bomber::cli::App;

fn main() {
    let cli = App::parse();
    dbg!(cli);
    //sms_bomber::sms::sender::pst();
}
