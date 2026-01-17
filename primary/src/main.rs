use axum::{Extension, Router, extract::Query, routing::get};
use dotenv::dotenv;
use img::{compute, extract};
use serde::Deserialize;
use sheets::Client;
use sheets_core::batch_update;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

mod img;
mod sheets_core;

#[derive(Deserialize, Debug, Clone)]
struct OauthVar {
    code: String,
    state: String,
}

async fn oauth_callback(
    Query(params): Query<OauthVar>,
    Extension(store): Extension<Arc<Mutex<Option<OauthVar>>>>,
) -> String {
    let mut lock = store.lock().unwrap();
    *lock = Some(params);

    "Authorization complete — you can close this window.".into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let spreadsheets = loop {
        print!("Is the access code in .env valid? (y/n): ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "y" => {
                let google_sheets = Client::new(
                    env::var("CLIENT_ID")?,
                    env::var("CLIENT_SECRET")?,
                    env::var("REDIRECT_URI")?,
                    env::var("TOKEN")?,
                    env::var("REFRESH_TOKEN")?,
                );

                println!("{}", "=".repeat(40));

                break google_sheets.spreadsheets();
            }

            "n" => {
                println!("{}", "=".repeat(40));
                println!("Will run OAuth flow.");
                println!("{}", "=".repeat(40));

                let oauth_store: Arc<Mutex<Option<OauthVar>>> = Arc::new(Mutex::new(None));

                let app = Router::new()
                    .route("/", get(oauth_callback))
                    .layer(Extension(oauth_store.clone()));

                let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
                let server = tokio::spawn(async move {
                    axum::serve(listener, app).await.unwrap();
                });

                let mut google_sheets = Client::new(
                    env::var("CLIENT_ID")?,
                    env::var("CLIENT_SECRET")?,
                    env::var("REDIRECT_URI")?,
                    env::var("TOKEN")?,
                    env::var("REFRESH_TOKEN")?,
                );

              
                let user_consent_url = google_sheets.user_consent_url(&[
                    "https://www.googleapis.com/auth/spreadsheets".to_string(),
                ]);

                println!(
                    "Open this URL in your browser:\n{}\n{user_consent_url}",
                    "-".repeat(40)
                );
                println!("{}", "=".repeat(40));

                let oauth_var: OauthVar = loop {
                    if let Some(oauth) = oauth_store.lock().unwrap().clone() {
                        println!("Google redirect received:");
                        println!("CODE = {}", oauth.code);
                        println!("STATE = {}", oauth.state);
                        println!("{}", "=".repeat(40));

                        break oauth;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                };

                server.abort();

                // In your redirect URL capture the code sent and our state.
                // Send it along to the request for the token.
                let code = &oauth_var.code;
                let state = &oauth_var.state;

                let access_token = google_sheets.get_access_token(code, state).await?;
                println!("{:?}", access_token);
                unsafe {
                    std::env::set_var("TOKEN", &access_token.access_token);
                }

                let google_sheets = Client::new(
                    env::var("CLIENT_ID")?,
                    env::var("CLIENT_SECRET")?,
                    env::var("REDIRECT_URI")?,
                    env::var("TOKEN")?,
                    env::var("REFRESH_TOKEN")?,
                );

                break google_sheets.spreadsheets();
            }

            _ => println!("Please type y or n."),
        }
    };

    let sheets_id = "1q3Py3vSx1HVKqxgm8eplNdkCltO5zQi0Wp8Hybj_U2s";

    // let range = ["F1:F2".to_string()];
    // let metadata = &spreadsheets.get(sheets_id, true, &range).await;
    // println!("{:?}", metadata);

    // Ask user for image path
    print!("Enter the path to the image file: ");
    io::stdout().flush()?;
    let mut image_path = String::new();
    io::stdin().read_line(&mut image_path)?;
    let image_path = image_path.trim();

    println!("{}", "=".repeat(40));
    println!("Processing image: {}", image_path);
    println!("{}", "=".repeat(40));

    let image_pixel_data = extract::ImageFeature::extract_pixels_feature(image_path);

    let compute_pixels = compute::ComputeConvolution::extract_pixels_feature(&image_pixel_data);

    batch_update::draw_in_sheets(&compute_pixels, &spreadsheets, sheets_id).await?;
    
    println!("Rendered image to Google Sheets!");
    println!("View at: https://docs.google.com/spreadsheets/d/{}", sheets_id);

    

    Ok(())
}
