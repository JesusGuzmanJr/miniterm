use {
    serialport::SerialPort,
    std::{
        io::{stdout, Read, Write},
        time::Duration,
    },
    termion::{color, raw::IntoRawMode},
};

type SerialDevice = Box<dyn SerialPort>;

/// Run the kernel image loader on the given device name
pub fn run_loader(device: &str, baud: u32, image: &str) {
    print_intro();
    set_ctrl_c_handler();

    let mut target_serial_input = open_serial(device, baud);
    let mut target_serial_output = target_serial_input
        .try_clone()
        .expect("Error creating second serial connection");

    println!("🔌 Please power the target now.");
    wait_for_payload_request(&mut target_serial_input);

    let image = load_kernel_image(image);

    send_kernel_size(
        image.len(),
        &mut target_serial_input,
        &mut target_serial_output,
    );

    send_kernel(&image, &mut target_serial_output);
    run_terminal(target_serial_input, target_serial_output);
}

/// Sends kernel image to a serial device)
fn send_kernel(image: &[u8], target_serial_output: &mut SerialDevice) {
    let progress_bar = indicatif::ProgressBar::new(image.len() as _);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("⏩ Loading {bytes} [{wide_bar:.white}] {percent}% {binary_bytes_per_sec}")
            .progress_chars("=🦀 "),
    );

    let mut sent = 0;
    // Send in 512 byte chunks
    for chunk in image.chunks(512) {
        target_serial_output
            .write_all(chunk)
            .expect("Error loading image to target serial.");
        sent += chunk.len();
        progress_bar.set_position(sent as _);
    }
    progress_bar.finish();
}

fn send_kernel_size(
    size: usize,
    target_serial_input: &mut SerialDevice,
    target_serial_output: &mut SerialDevice,
) {
    target_serial_output
        // loader expects size to be little-endian 32 bits.
        .write_all(&(size as u32).to_le_bytes())
        .expect("Error writing the size to target serial.");

    // wait for ok
    loop {
        let mut buffer = [0; 2];
        match target_serial_input.read(&mut buffer) {
            // loop again if its a timeout
            Err(ref error) if error.kind() == std::io::ErrorKind::TimedOut => (),
            Err(error) => {
                eprintln!("Error getting target serial input: {}", error);
                std::process::exit(1);
            }
            Ok(_) => {
                if buffer == "OK".as_bytes() {
                    return;
                } else {
                    eprintln!(
                        "Error reading OK confirmation after sending image size to target serial."
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}

fn load_kernel_image(image: &str) -> Vec<u8> {
    match std::fs::read(image) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("Error reading kernel image: {}", error);
            std::process::exit(1);
        }
    }
}

fn wait_for_payload_request(target_serial_input: &mut SerialDevice) {
    let mut count = 0;
    loop {
        let mut buffer = [0; 4096];

        match target_serial_input.read(&mut buffer) {
            // loop again if its a timeout
            Err(ref error) if error.kind() == std::io::ErrorKind::TimedOut => (),
            Err(error) => {
                eprintln!("Error getting target serial input: {}", error);
                std::process::exit(1);
            }
            Ok(n) => {
                print!("{}", &String::from_utf8_lossy(&buffer[0..n]));
                for byte in &buffer[0..n] {
                    if byte == &3 {
                        count += 1;
                        // if we read three 3s, that's the signal to proceed.
                        if count == 3 {
                            return;
                        }
                    } else {
                        // Any other byte resets token counting.
                        count = 0;
                    }
                }
            }
        }
    }
}

pub fn run(device: &str, baud: u32) {
    print_intro();
    set_ctrl_c_handler();

    let target_serial_input = open_serial(device, baud);
    let target_serial_output = target_serial_input
        .try_clone()
        .expect("Error creating second serial connection.");

    run_terminal(target_serial_input, target_serial_output);
}

fn run_terminal(mut target_serial_input: SerialDevice, mut target_serial_output: SerialDevice) {
    std::thread::spawn(move || loop {
        // buffer of 1 byte.
        let mut buffer = [0; 1];

        match target_serial_input.read(&mut buffer) {
            // loop again if its a timeout
            Err(ref error) if error.kind() == std::io::ErrorKind::TimedOut => (),
            Err(error) => {
                eprintln!("Error getting target serial input: {}", error);
                std::process::exit(1);
            }
            Ok(n) => {
                let mut host_stdout = stdout().into_raw_mode().expect("Error getting raw stdout.");

                // Translate incoming newline to carriage return + newline.
                if buffer[0] == b'\n' {
                    write!(host_stdout, "\r\n").expect("Error writing to stdout.");
                } else {
                    host_stdout
                        .write_all(&buffer[0..n])
                        .expect("Error writing to stdout.");
                }
                host_stdout.flush().expect("Error flushing stdout.");
            }
        }
    });

    let mut host_stdin = termion::async_stdin().bytes();
    loop {
        if let Some(input) = host_stdin.next() {
            let input = input.expect("Error with stdin stream.");
            let mut buffer = [0; 1];
            buffer[0] = input;
            target_serial_output
                .write_all(&buffer)
                .expect("Error writing to target serial.");
        }
    }
}

fn print_intro() {
    println!(
        "{}\n{} - {}\n{}",
        color::Fg(color::Cyan),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        termion::style::Reset
    );
}

fn set_ctrl_c_handler() {
    ctrlc::set_handler(move || {
        println!("\nBye 👋");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler.");
}

fn open_serial(device: &str, baud_rate: u32) -> SerialDevice {
    wait_for_serial_device(device);

    let connection = match serialport::new(device, baud_rate)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("🚫 {}. Maybe try with 'sudo'.", error);
            std::process::exit(1);
        }
    };

    println!("✅ Serial connected");
    connection
}

fn wait_for_serial_device(device: &str) {
    let is_connected = || std::path::Path::new(device).exists();

    if is_connected() {
        return;
    }

    println!("⏳ Waiting for {}", device);
    loop {
        std::thread::sleep(Duration::from_secs(1));
        if is_connected() {
            break;
        };
    }
}
