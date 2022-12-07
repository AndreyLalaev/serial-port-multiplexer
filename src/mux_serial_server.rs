use clap::Parser;
use futures::stream::SplitStream;
use futures::{stream::StreamExt, SinkExt};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use serial_mux::serial_con::SerialConnection;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    serial_port: String,
    #[clap(short, long, default_value_t = 115200)]
    baudrate: u32,
}

async fn get_command_output(rx: &mut SplitStream<Framed<SerialStream, SerialConnection>>) {
    let output = rx
        .next()
        .await
        .expect("awaiting item in rx stream")
        .expect("reading command output");

    tokio::task::spawn_blocking(move || {
        println!("{}", output);
    });
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

    let mut input = String::new();
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    loop {
        stdout
            .write_all("uart -> ".as_bytes())
            .await
            .expect("writing prompt");
        stdout.flush().await.expect("flushing stdout");

        reader
            .read_line(&mut input)
            .await
            .expect("reading user command");

        let cmd: String = input.trim().into();
        input.clear();

        tx.send(cmd).await.expect("sending cmd");
        get_command_output(&mut rx).await;
    }
}
