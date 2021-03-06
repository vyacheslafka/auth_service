use auth_service::Config;

fn main() {
    let config = Config::read();
    println!("Config: {:?}", config);
}
