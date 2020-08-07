use csv::Reader;
use std::error::Error;

use csv;
use csv::ReaderBuilder;

use serde::Deserialize;

use reqwest; // https://docs.rs/reqwest/0.10.7/reqwest/

#[derive(Debug, Deserialize)]
struct Row {
    shop_name: String,
    shop_address: String,
    lat: String,
    lon: String,
}

fn example2() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path("./ShopAddressesLongLat.csv")?;
    for record in rdr.deserialize() {
        let record: Row = record?;
        println!(
            "Shop Name:{},Shop Address:{},LAT:{},Long:{}",
            record.shop_name, record.shop_address, record.lat, record.lon
        );
    }

    Ok(())
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./ShopAddressesLongLat.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record.get(1));
    }
    Ok(())
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers()?;
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: Row = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn request_googlemaps_distancematrix(
    origins_lat: &str,
    origins_lon: &str,
    destinatio_lat: &str,
    destination_lon: &str,
    apikey: &str,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let url = format!("https://maps.googleapis.com/maps/api/distancematrix/json?units=imperial&origins={},{}&destinations={},{}&key={}",origins_lat,origins_lon,destinatio_lat,destination_lon,apikey);
    let res = reqwest::blocking::get(&url)?;
    Ok(res)
}

async fn request_googlemaps_distancematrix2(
    origins_lat: &str,
    origins_lon: &str,
    destinatio_lat: &str,
    destination_lon: &str,
    apikey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://maps.googleapis.com/maps/api/distancematrix/json?units=imperial&origins={},{}&destinations={},{}&key={}",origins_lat,origins_lon,destinatio_lat,destination_lon,apikey);
    println!("{}", url);
    let response = reqwest::get(&url).await?.text().await?;
    use serde_json::{Result, Value};
    let v: Value = serde_json::from_str(&response)?;
    println!("------------------Api Response------------------\n");
    println!("Api Response:{:?}\n", response);
    println!("------------------Parsed Response------------------\n");
    println!("origin_addresses:{}\n", v["origin_addresses"]);
    println!("destination_addresses:{}\n", v["destination_addresses"]);
    println!(
        "distance:{}\n",
        v["rows"][0]["elements"][0]["distance"]["text"]
    );
    println!(
        "duration:{}\n",
        v["rows"][0]["elements"][0]["duration"]["text"]
    );
    Ok(())
}

async fn request_googlemaps_distancematrix3(
    origins_lat: &str,
    origins_lon: &str,
    destination_latlon: &str,
    apikey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://maps.googleapis.com/maps/api/distancematrix/json?units=imperial&origins={},{}&destinations={}&key={}",origins_lat,origins_lon,destination_latlon,apikey);
    println!("{}", url);
    let response = reqwest::get(&url).await?.text().await?;
    use serde_json::{Result, Value};
    let v: Value = serde_json::from_str(&response)?;
    println!("------------------Api Response------------------\n");
    println!("Api Response:{:?}\n", response);
    println!("------------------Parsed Response------------------\n");
    println!("origin_addresses:{}\n", v["origin_addresses"]);
    println!("destination_addresses:{}\n", v["destination_addresses"]);
    println!(
        "distance:{}\n",
        v["rows"][0]["elements"][0]["distance"]["text"]
    );
    println!(
        "duration:{}\n",
        v["rows"][0]["elements"][0]["duration"]["text"]
    );
    Ok(())
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //paste api key here
    let apikey = "";

    let mut reader = csv::Reader::from_path("./ShopAddressesLongLat.csv")?;

    let headers = reader.headers()?;
    //customer lat & lon
    let origins_lat = "3.1421613";
    let origins_lon = "101.7106398";
    let mut alldestinationaddress = String::from("");
    let mut counter: u128 = 0;
    for result in reader.deserialize() {
        let record: Row = result?;
        if counter < 1 {
            alldestinationaddress = format!("{},{}", record.lat, record.lon);
            println!("{}", alldestinationaddress);
        } else {
            alldestinationaddress =
                format!("{}|{},{}", alldestinationaddress, record.lat, record.lon);
            println!("{}", alldestinationaddress);
        }
        counter = counter + 1;
        println!("{}", counter);
    }

    request_googlemaps_distancematrix3(origins_lat, origins_lon, &alldestinationaddress, apikey)
        .await;

    Ok(())
}
