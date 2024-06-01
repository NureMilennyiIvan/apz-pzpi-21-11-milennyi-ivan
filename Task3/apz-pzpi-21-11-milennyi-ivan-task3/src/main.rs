mod scanner_error;
mod temperature;
mod auth;
mod utils;

use std::env;
use std::time::Duration;
use dotenv::dotenv;
use tokio::time::sleep as sleepAsync;
use crate::auth::{authenticate, read_and_format_cli_arguments};
use crate::temperature::{read_temperature, send_temperature};
use crate::utils::{continue_action, display, panic};


#[tokio::main]
async fn main() -> ! {
    dotenv().ok();

    let server_url: String = match env::var("SERVER_URL"){
        Ok(url) => url,
        Err(error) => panic(&error.to_string())
    };

    let auth = match read_and_format_cli_arguments() {
        Ok(auth) => auth,
        Err(error) => {
            panic(&error.to_string())
        }
    };

    if let Err(error) = authenticate(&auth, &server_url).await {
        panic(&error.to_string())
    }
    let mut temperature: u16 = 370;
    loop {
        temperature = read_temperature(temperature);
        match send_temperature(auth.id, temperature, &server_url).await {
            Ok(result) => {
                display(&format!("{}", result));
                sleepAsync(Duration::from_millis(5000)).await;
            },
            Err(error) => {
                display(&format!("Fix and try again. {}", error.to_string()));
                continue_action();
            }
        }

    }
}
