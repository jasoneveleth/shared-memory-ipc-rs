use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use std::env;
use std::process;

fn consumer() {
    let (server, server_name) = IpcOneShotServer::<String>::new().unwrap();
    println!("Server name: {}", server_name);
    let (client_channel, received_message): (IpcReceiver<String>, String) = server.accept().unwrap();
    loop {
        println!("Received message from client: {}", received_message);
        client_channel.recv().unwrap();
    }
}

fn producer(msg: String, server_name: String) {
    let server_receiver: IpcSender<String> = IpcSender::connect(server_name).unwrap();

    server_receiver.send(msg).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [produce|consume]", args[0]);
        process::exit(1);
    }

    let mode = &args[1];

    if mode == "produce" && args.len() < 3 {
        eprintln!("Usage: {} produce <server_name>", args[0]);
        process::exit(1);
    }

    if mode == "produce" {
        producer("Hello from the client!".to_string(), args[2].to_string());
    } else if mode == "consume" {
        consumer();
    } else {
        eprintln!("Unknown mode: {}", mode);
        process::exit(1);
    }
}
