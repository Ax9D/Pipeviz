use simple_logger::SimpleLogger;
use std::{rc::Rc, thread};

mod pipewire_impl;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let (sender, receiver) = std::sync::mpsc::channel();

    let pw_thread_handle = thread::spawn(move || {
        let sender = Rc::new(sender);

        pipewire_impl::thread_main(&sender).expect("Failed to init pipewire client");
    });

    ui::run_graph_ui(receiver);

    pw_thread_handle.join().expect("👽👽👽");

    Ok(())
}
