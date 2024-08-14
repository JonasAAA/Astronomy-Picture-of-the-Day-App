use eframe::egui;
use reqwest::{blocking::Client, Error};
use serde::Deserialize;

fn main() -> eframe::Result {
    eframe::run_native(
        "NASA API app",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<NasaApiApp>::new(NasaApiApp::new()))
        }),
    )
}

const API_KEY: &str = "6Nn2ROeWkxZqu54zpdvneUiYxczLjMeSexx7GZ4Q";

#[derive(Deserialize, Clone)]
struct PictureOfTheDayResponse {
    // resource: HashMap<String, String>,
    title: String,
    hdurl: String,
}

#[derive(Clone)]
struct StringError(String);

struct NasaApiApp {
    client: Client,
    response: Result<PictureOfTheDayResponse, StringError>,
}

impl NasaApiApp {
    fn new() -> Self {
        Self {
            client: Client::new(),
            response: Err(StringError("".to_string())),
        }
    }
}

impl eframe::App for NasaApiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NASA API App");
            if ui.button("Show next image").clicked() {
                self.response = get_picture_of_the_day(&self.client)
                    .map_err(|error| StringError(format!("Failed because: {error}")));
            }
            match &self.response {
                Ok(resp) => {
                    ui.label(resp.title.as_str());
                    ui.add(egui::Image::from_uri(resp.hdurl.as_str()));
                }
                Err(StringError(error)) => {
                    ui.label(error);
                }
            }
        });
    }
}

fn get_picture_of_the_day(client: &Client) -> Result<PictureOfTheDayResponse, Error> {
    // let mut resp = client
    //     .get("https://api.nasa.gov/planetary/apod")
    //     .query(&[("api_key", API_KEY), ("count", "1")])
    //     .send()?;
    // let mut content = String::new();
    // resp.read_to_string(&mut content).unwrap();
    // Ok(PictureOfTheDayResponse { title: content })

    Ok(client
        .get("https://api.nasa.gov/planetary/apod")
        .query(&[("api_key", API_KEY), ("count", "1")])
        .send()?
        .json::<Vec<PictureOfTheDayResponse>>()?
        .first()
        .unwrap()
        .clone())
}
