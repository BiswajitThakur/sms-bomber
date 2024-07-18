use clap::Parser;

#[derive(Debug, Clone)]
pub enum Limit {
    Infinity,
    Value(u32),
}

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    /// Mobile number
    #[arg(short, long)]
    pub mobile: u64,

    /// number of sms. Default infinity. (For infinity don't use this flag.)
    #[arg(short, long)]
    pub limit: Option<u32>,

    /// Disable output
    #[arg(short, long)]
    pub no_output: bool,

    /// Delay in second
    #[arg(short, long, default_value_t = 2)]
    pub delay: u64,
}

impl App {
    pub fn get_limit(&self) -> Limit {
        if self.limit.is_some() {
            Limit::Value(self.limit.unwrap())
        } else {
            Limit::Infinity
        }
    }
}
