use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use dotenv::dotenv;

use std::{io, time::Duration};

use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = weather_cli::App::new();
    let res = weather_cli::run_app(&mut terminal, app, tick_rate);

    // dotenv().ok();
    // let api_token =
    // std::env::var("API_TOKEN").expect("expected there to be an api token");

    // let mut arg_iterator = std::env::args();
    // arg_iterator.next();
    // let args: String = arg_iterator.collect();

    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", weather_cli::PARIS.lat.to_string()),
            ("longitude", weather_cli::PARIS.lon.to_string()),
            ("hourly", "temperature_2m,relativehumidity_2m,apparent_temperature,precipitation_probability,precipitation,pressure_msl,cloudcover,windspeed_10m,windspeed_80m,winddirection_10m,winddirection_80m,temperature_80m,soil_temperature_0cm,soil_moisture_0_1cm".to_string()),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}
