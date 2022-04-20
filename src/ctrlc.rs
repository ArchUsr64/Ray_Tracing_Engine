pub fn ctrl_c_init() {
    ctrlc::set_handler(move || {
        println!("Keyboard interrupt received");
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");
}
