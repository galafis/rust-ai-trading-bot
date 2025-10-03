use log::info;

pub fn setup_logger() {
    env_logger::init();
    info!("Logger initialized.");
}

