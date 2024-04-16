use tokio::fs;
use tokio::io::AsyncWriteExt;
use reqwest;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use chrono::Utc;
use dirs::desktop_dir;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::io::{self, AsyncBufReadExt, BufReader};

// Struct to hold resolution details
struct Resolution {
    width: u32,
    height: u32,
    description: String,
}

impl Resolution {
    fn new(width: u32, height: u32, description: &str) -> Self {
        Self {
            width,
            height,
            description: description.to_owned(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resolution_options: HashMap<String, Resolution> = setup_resolutions();
    let resolution_choice: String = get_user_input("Please select your desired resolution (e.g., FHD, QHD, 4K):")?.to_uppercase();
    let resolution_details: &Resolution = validate_resolution_choice(&resolution_options, &resolution_choice)?;

    let query: String = get_user_input("Please enter the search query for the images:")?;
    let times: usize = get_user_input("How many images would you like to download?")?.parse::<usize>()?;

    recap_choices(&resolution_details, &query, times);

    if !confirm("Do you want to proceed with the download? (Y/N):").await? {
        println!("{}", "Download canceled by the user.".bright_red());
        return Ok(());
    }

    let dir_name: String = query.replace("?", "-").replace(" ", "_");
    let desktop_path = desktop_dir().unwrap_or_else(|| env::current_dir().unwrap());
    let target_directory = desktop_path.join(&dir_name);

    perform_downloads(&resolution_details, &query, times, &target_directory, &dir_name).await?;

    if confirm("Do you want to open the folder where the images are saved? (Y/N):").await? {
        Command::new("explorer").arg(&target_directory).spawn()?;
    }

    println!("All downloads completed successfully.");
    Ok(())
}

fn setup_resolutions() -> HashMap<String, Resolution> {
    let mut options: HashMap<String, Resolution> = HashMap::new();
    options.insert("FHD".to_string(), Resolution::new(1920, 1080, "Full HD (16:9 aspect ratio)"));
    options.insert("QHD".to_string(), Resolution::new(2560, 1440, "Quad HD (16:9 aspect ratio)"));
    options.insert("4K".to_string(), Resolution::new(3840, 2160, "4K Ultra HD (16:9 aspect ratio)"));
    options
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    println!("{}", prompt.bright_white().underline());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn validate_resolution_choice<'a>(
    resolutions: &'a HashMap<String, Resolution>,
    choice: &str,
) -> Result<&'a Resolution, String> {
    resolutions.get(choice).ok_or_else(|| "Invalid resolution choice".to_string())
}

fn recap_choices(resolution: &Resolution, query: &str, num_images: usize) {
    println!("\n{}\n", "Here are your choices:".bright_white().underline());
    println!("Resolution: {} - {}", resolution.width.to_string().bright_cyan(), resolution.description.bright_cyan());
    println!("Search Query: {}", query.bright_yellow());
    println!("Number of Images to Download: {}", num_images.to_string().bright_green());
    println!("---");
}

async fn confirm(prompt: &str) -> Result<bool, io::Error> {
    println!("{}", prompt.bright_white().underline());

    let mut stdin = BufReader::new(io::stdin());
    let mut input = String::new();

    stdin.read_line(&mut input).await?;
    Ok(input.trim().to_lowercase() == "y")
}

async fn perform_downloads(
    resolution: &Resolution,
    query: &str,
    num_images: usize,
    target_directory: &PathBuf,
    dir_name: &str
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_url = "https://source.unsplash.com/random/";
    fs::create_dir_all(target_directory).await?;

    let progress_bar = ProgressBar::new(num_images as u64);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Error setting progress bar template")
        .progress_chars("#>-");

    progress_bar.set_style(style);
    progress_bar.set_position(0);  // Initialize progress bar position

    let mut tasks: Vec<tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>> = Vec::new();
    for _i in 0..num_images {
        let filename = format!("{}-{}.png", dir_name, Utc::now().format("%Y%m%d%H%M%S%f"));
        let full_path = target_directory.join(&filename);
        let image_url = format!("{}{}/?{}", base_url, format!("{}x{}", resolution.width, resolution.height), query);

        let client = reqwest::Client::new();
        let progress_clone = progress_bar.clone();
        tasks.push(tokio::spawn(async move {
            let response = client.get(&image_url).send().await?;
            let bytes = response.bytes().await?;
            let mut file = fs::File::create(&full_path).await?;
            file.write_all(&bytes).await?;
            progress_clone.inc(1);  // Increment progress after each download
            Ok(())
        }));
    }

    for task in tasks {
        task.await??;
    }

    progress_bar.finish_with_message("Download complete");
    Ok(())
}
