use {
    serialport::SerialPort,
    std::{
        io::{stdout, Read, Write},
        time::Duration,
    },
    termion::{color, raw::IntoRawMode},
};

pub fn run(device: &str, baud_rate: u32) {
    print_intro();
    set_ctrl_c_handler();

    let mut target_serial_input = open_serial(device, baud_rate);
    let mut target_serial_output = target_serial_input
        .try_clone()
        .expect("Error creating second serial connection");

    std::thread::spawn(move || loop {
        let mut buffer = [0; 1];
        // wait_for_data(&target_serial_input);

        match target_serial_input.read(&mut buffer) {
            // loop again if its a timeout
            Err(ref error) if error.kind() == std::io::ErrorKind::TimedOut => (),
            Err(error) => {
                eprintln!("Error getting target serial input: {}", error);
            }
            Ok(n) => {
                let mut host_stdout = stdout().into_raw_mode().expect("Error getting raw stdout.");

                // let mut write_buffer = |buffer: &[u8]| {
                //     host_stdout
                //         .write(&buffer[0..n])
                //         .expect("Error writing to stdout.");
                // };

                // Translate incoming newline to carriage return + newline.
                if buffer[0] == b'\n' {
                    write!(host_stdout, "\r\n").expect("Error writing to stdout.");
                    // write_buffer(&[b'\r', b'\n']);
                } else {
                    host_stdout
                        .write(&buffer[0..n])
                        .expect("Error writing to stdout.");
                }
                host_stdout.flush().expect("Error flushing stdout.");
            }
        }
    });

    let mut host_stdint = termion::async_stdin().bytes();
    loop {
        // if let Some(Ok(key)) = key_inputs.next() {
        //     if key == termion::event::Key::Ctrl('c') {
        //         // target_to_host.join();
        //         break;
        //     }
        // }
        // std::thread::sleep(Duration::from_millis(50));
        if let Some(input) = host_stdint.next() {
            let input = input.expect("Error with stdin stream");
            let mut buffer = [0; 1];
            buffer[0] = input;
            target_serial_output
                .write(&buffer)
                .expect("Error writing to target serial");
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

fn open_serial(device: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    wait_for_serial_device(device);

    println!("Opening serial port to {} with baud {}.", device, baud_rate);

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
