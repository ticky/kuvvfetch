#[macro_use]
extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate scraper;

fn main() {
    use clap::Arg;

    let arguments = app_from_crate!()
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Print more information about activity"),
        )
        .arg(
            Arg::with_name("share-url")
                .help("Your personal Kuvva share URL. Must begin with \"https://www.kuvva.com/s/\"")
                .index(1)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Directory to save wallpaper images to. This will default to the current working directory.")
                .index(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("resolution")
                .long("resolution")
                .short("r")
                .help("Wallpaper size to download")
                .takes_value(true)
                .default_value("2880x1800"),
        )
        .get_matches();

    // Figure out our output directory, and then make sure it exists
    let output = match arguments.value_of("output") {
        Some(output_path_str) => std::path::PathBuf::from(output_path_str),
        None => std::env::current_dir().unwrap()
    };

    if !output.exists() {
        println!("Specified output directory \"{}\" doesn't seem to exist!", output.display());
        std::process::exit(1);
    }

    if !output.is_dir() {
        println!("Specified output directory \"{}\" doesn't seem to be a directory!", output.display());
        std::process::exit(1);
    }

    // Get the share URL from the arguments
    let share_url = arguments.value_of("share-url").unwrap();

    // Check the specified resolution seems valid
    let resolution = arguments.value_of("resolution").unwrap();
    let resolution_regex = regex::Regex::new(r"^\d+x\d+$").unwrap();

    if !resolution_regex.is_match(resolution) {
        println!("Specified resolution \"{}\" doesn't look valid!", resolution);
        std::process::exit(1);
    }

    // Set up the selectors we'll use to find the necessary things
    let thumbnail_selector = scraper::Selector::parse(".thumb-grid li > a[href] > img[src]").unwrap();
    let next_page_selector = scraper::Selector::parse(".pagination > a.next").unwrap();

    println!(
        "Fetching wallpapers from {}, and saving them to {} at {}...",
        share_url,
        output.display(),
        resolution
    );

    let http_client = reqwest::Client::builder().build().unwrap();

    let favourites_page = http_client.get(share_url).send().unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&favourites_page);

    let thumbnails = document.select(&thumbnail_selector);
    let next_page = document.select(&next_page_selector);

    if next_page.count() > 0 {
        print!("NOTE: There are more pages of results to fetch. This utility won't pick them up yet!");
    }

    let url_resolution_regex = regex::Regex::new(r"/\d+x\d+_").unwrap();
    let url_resolution = &*format!("/{}_", resolution);

    for thumbnail in thumbnails {
        let thumbnail_url = thumbnail.value().attr("src").unwrap();
        println!("Thumbnail found at {}", thumbnail_url);

        let fullsize_url = url_resolution_regex.replace(thumbnail_url, url_resolution);
        println!("Fetching full size from {}", fullsize_url);

        let mut fullsize_response = http_client.get(&*fullsize_url.into_owned()).send().unwrap();

        if !fullsize_response.status().is_success() {
            println!("Error: {}. This wallpaper might not exist in this resolution.", fullsize_response.status());
        } else {
            let fullsize_filename = match fullsize_response.url().path_segments().unwrap().next_back() {
                Some(filename) => filename.to_owned(),
                None => panic!("what, that makes no sense???")
            };

            let fullsize_path = output.join(&fullsize_filename);

            println!("Output is {}", fullsize_path.display());

            let mut fullsize_file = match std::fs::File::create(&fullsize_path) {
                Err(why) => {
                    use std::error::Error;
                    panic!("couldn't create {}: {}",
                        fullsize_path.display(),
                        why.description())
                },
                Ok(file) => file,
            };

            match fullsize_response.copy_to(&mut fullsize_file) {
                Err(error) => panic!("error writing image: {}", error),
                Ok(_bytes_written) => ()
            }
        }
    }
}
