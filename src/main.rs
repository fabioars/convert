use std::env;
use std::collections::HashMap;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (value, from, to) = extract(args);

    let factor = get_factor(from.clone(), to.clone()).unwrap();
    let result = factor * value;

    println!("{} in {} is {} {}", value, from, result, to);
}

#[tokio::main]
async fn get_factor(from: String, to: String) -> Result<f32, Box<dyn std::error::Error>> {
    let currencies = format!("{}_{}", from, to).to_uppercase();
    let uri = format!("https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey=6bbe8c6dd4dedea89ea7", currencies);
    let resp = reqwest::get(&uri)
        .await?
        .json::<HashMap<String, f32>>()
        .await?;

    let factor = resp.get(&currencies);
    Ok(factor.unwrap().clone())
}

fn extract(args: Vec<String>) -> (f32, String, String) {
    let value: f32 = args[1].parse().unwrap();
    let from: String = args[2].clone().to_uppercase();
    let to: String = args[4].clone().to_uppercase();

    (value, from, to)
}