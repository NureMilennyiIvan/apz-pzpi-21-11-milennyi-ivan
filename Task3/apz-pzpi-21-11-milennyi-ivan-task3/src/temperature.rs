use json::object;
use rand::Rng;
use reqwest::Client;
use crate::scanner_error::ScannerError;
use crate::utils::display;

pub(crate) fn read_temperature(previous_temp: u16) -> u16 {
    let mut rng = rand::thread_rng();
    let min_temp = 360;
    let max_temp = 400;
    let step = 1;

    let new_temp = if previous_temp < min_temp {
        min_temp
    } else if previous_temp > max_temp {
        max_temp
    } else {
        let delta = rng.gen_range(-step..=step);
        let temp = previous_temp as i16 + delta;
        temp.max(min_temp as i16).min(max_temp as i16) as u16
    };
    display(&format!("Temperature is {}", new_temp));
    new_temp
}

pub(crate) async fn send_temperature(id: u64, temp: u16, server_url: &str) -> Result<String, ScannerError>  {
    let client = Client::new();
    let url = format!("{}update-temperature/{}", server_url, id);

    let body = object!{
            temperature: temp,
    }.dump();

    let response = match  client.patch(&url).header("Content-Type", "application/json").body(body).send().await{
        Ok(res) => res,
        Err(error) => return Err(ScannerError::UpdatingError(error.to_string()))
    };

    match response.json().await{
        Ok(text) => Ok(text),
        Err(error) => return Err(ScannerError::UpdatingError(error.to_string()))
    }
}
