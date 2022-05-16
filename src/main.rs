use clap::{Command, Arg};
use reqwest::blocking::Response;
use std::{fs::File, io::Write};
use regex::Regex;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const URL: &str = "https://unsplash.com";

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg_required_else_help(true)
        .arg(Arg::new("filename")
             .short('f')
             .long("filename")
             .takes_value(true))
        .get_matches();
    if let Some(filename) = matches.value_of("filename"){
        let result = get_response(URL);
        match result{
            Ok(response) => {
                if response.status().is_success(){
                    let response_text = response.text().unwrap();
                    // println!("{}", &response_text);
                    let first_url = get_first_url(&response_text).unwrap();
                    println!("{}", &first_url);
                    let content = get_response(&first_url).unwrap().text().unwrap();
                    let second_url = get_second_url(&content).unwrap();
                    println!("{}", &second_url);
                    download_filename(&second_url, filename);
                }else if response.status().is_server_error() {
                    println!("Server error");
                }else if response.status().is_client_error(){
                    println!("Client error")
                }else {
                    println!("Something happened")
                }
            },
            Err(e) => {
                println!("{:?}", e);
            },
        }
    }
}

fn get_response(url: &str) -> Result<Response, String>{
    let result = reqwest::blocking::get(url);
    match result{
        Ok(response) => {
            if response.status().is_success(){
                Ok(response)
            }else if response.status().is_server_error() {
               Err("Server error".to_string())
            }else if response.status().is_client_error(){
                Err("Client error".to_string())
            }else {
                Err("Something happened".to_string())
            }
        },
        Err(e) => Err(format!("{:?}", e)),
    }

}

fn download_filename(url: &str, filename: &str) -> Result<String, String>{
    let result = get_response(url);
    match result{
        Ok(response) => {
            let content = response.bytes().unwrap();
            let mut file = File::create(filename).unwrap();
            match file.write_all(&content){
                Ok(_) => Ok("Image saved".to_string()),
                Err(e) => Err(format!("{:?}", e)),
            }
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn get_first_url(content: &str) -> Result<String, String>{
    let re = Regex::new(r#"<a[^>]*>Photo of the Day</a>"#).unwrap();
    match re.find(content){
        Some(found) => {
            let re2 = Regex::new(r#"href="([^"]*)""#).unwrap();
            match re2.captures(found.as_str()){
                Some(url) => Ok(format!("{}{}", URL, url[1].to_string())),
                None => Err("Nothing found".to_string()),
            }

        },
        None => Err("Nothing found".to_string()),
    }
}

fn get_second_url(content: &str) -> Result<String, String>{
    let re = Regex::new(r#""contentUrl":"([^"]*)""#).unwrap();
    match re.captures(content){
        Some(url) => Ok(url[1].to_string()),
        None => Err("Nothing found".to_string()),
    }
}
