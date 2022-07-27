








pub mod request_builder;
pub mod response;
pub use utility::get_best_stations;
pub mod utility;





type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub mod telegram_handler;
pub use telegram_handler::conversation_handler;


#[tokio::main]
async fn main() {

    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    //telegram handler
    loop {
        telegram_handler::conversation_handler().await;
    }
}
