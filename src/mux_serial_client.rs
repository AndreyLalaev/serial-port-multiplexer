use clap::Parser;
use futures::{SinkExt, StreamExt};
use tokio_serial::{self, SerialPortBuilderExt};
use tokio_util::codec::Decoder;

use serial_mux::serial_con::SerialConnection;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = String::from("/dev/ttyS0"))]
    serial_port: String,
    #[clap(short, long, default_value_t = 115200)]
    baudrate: u32,
}

fn parse_command_args(cmd: &str) -> (&str, Vec<&str>) {
    let mut iter = cmd.split_whitespace();
    let bin = iter.next().expect("getting binary name");

    (bin, iter.collect())
}

fn run_command(cmd: &str) -> String {
    let (bin, args) = parse_command_args(cmd);

    let p = std::process::Command::new(bin)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawning command");

    let output = p.wait_with_output().expect("waiting for output");
    String::from_utf8(output.stdout).expect("converting output to string")
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut serial_port = tokio_serial::new(args.serial_port, args.baudrate)
        .open_native_async()
        .expect("opening serial port");

    serial_port
        .set_exclusive(true)
        .expect("setting exclusive access");

    let connection = SerialConnection::new().framed(serial_port);
    let (mut tx, mut rx) = connection.split();

    loop {
        let cmd = rx
            .next()
            .await
            .expect("awaiting cmd from rx")
            .expect("reading cmd");

        let out = tokio::task::spawn_blocking(move || run_command(&cmd))
            .await
            .expect("awaiting for cmd output");

        tx.send(out).await.expect("sending cmd output");
    }
}
