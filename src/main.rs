mod terminal;

fn main() {
    color_backtrace::install();

    const DEVICE_NAME: &str = "DEVICE_NAME";
    const DEFAULT_DEVICE_NAME: &str = "/dev/ttyUSB0";
    const BAUD: &str = "BAUD";
    const DEFAULT_BAUD: &str = "921_600";

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name(DEVICE_NAME)
                .help(&format!(
                    "The serial device to use; defaults to {}",
                    DEFAULT_DEVICE_NAME
                ))
                .short("d")
                .long("device")
                .value_name(DEVICE_NAME)
                .required(false),
        )
        .arg(
            clap::Arg::with_name(BAUD)
                .help(&format!("The baud to use; defaults to {}", DEFAULT_BAUD))
                .short("b")
                .long("baud")
                .value_name(BAUD)
                .required(false),
        )
        .get_matches();

    let device = matches.value_of(DEVICE_NAME).unwrap_or(DEFAULT_DEVICE_NAME);
    let baud = matches
        .value_of(BAUD)
        .unwrap_or(DEFAULT_BAUD)
        .replace('_', "");

    match baud.parse::<u32>() {
        Err(_) => {
            eprintln!("{} {} could not be parsed to a `u32`.", BAUD, baud);
            std::process::exit(1);
        }
        Ok(baud_rate) => terminal::run(device, baud_rate),
    }
}
