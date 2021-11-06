mod terminal;

fn main() {
    color_backtrace::install();

    const DEVICE_NAME: &str = "device_name";
    const DEFAULT_DEVICE_NAME: &str = "/dev/ttyUSB0";

    const BAUD: &str = "baud";
    const DEFAULT_BAUD: &str = "921_600";

    const LOAD: &str = "load";
    const IMAGE: &str = "image";

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
                .value_name("DEVICE")
                .required(false),
        )
        .arg(
            clap::Arg::with_name(BAUD)
                .help(&format!("The baud to use; defaults to {}", DEFAULT_BAUD))
                .short("b")
                .long("baud")
                .value_name("BAUD")
                .required(false),
        )
        .subcommand(
            clap::SubCommand::with_name(LOAD)
                .about("Load a kernel image to the device.")
                .arg(
                    clap::Arg::with_name(IMAGE)
                        .help("The path to the kernel image.")
                        .value_name("IMAGE")
                        .required(true),
                ),
        )
        .get_matches();

    let baud = matches
        .value_of(BAUD)
        .unwrap_or(DEFAULT_BAUD)
        .replace('_', "");

    let baud = match baud.parse::<u32>() {
        Ok(baud) => baud,
        Err(_) => {
            eprintln!("{} {} could not be parsed to a `u32`.", BAUD, baud);
            std::process::exit(1);
        }
    };

    let device = matches.value_of(DEVICE_NAME).unwrap_or(DEFAULT_DEVICE_NAME);

    if let Some(matches) = matches.subcommand_matches(LOAD) {
        // Calling .unwrap() is safe here because "IMAGE" is required.
        let image = matches.value_of(IMAGE).unwrap();

        terminal::run_loader(device, baud, image);
    } else {
        terminal::run(device, baud);
    }
}
