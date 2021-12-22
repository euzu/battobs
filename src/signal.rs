pub(crate) fn signal_handling() {
    match ctrlc::set_handler(|| std::process::exit(0)) {
        _ => (),
    }
}