extern crate hyper;
extern crate serialize;

use std::os;

use serialize::json;

use hyper::Client;


#[allow(non_snake_case)]
#[deriving(Show, Decodable, Encodable)]
struct Station {
    code: String,
    name: String,
    ch1903X: u32,
    ch1903Y: u32,
    lat: f32,
    lng: f32,
}

#[allow(non_snake_case)]
#[deriving(Show, Decodable, Encodable)]
struct Measurement {
    station: Station,
    code: String,
    dateTime: String,
    temperature: f32,
    sunshine: u8,
    precipitation: f32,
    windDirection: u16,
    windSpeed: f32,
    qnhPressure: f32,
    gustPeak: f32,
    humidity: u8,
    qfePressure: f32,
    qffPressure: f32,
}

fn main() {
    // Argument parsing
    let args = os::args();
    match args.len() {
        2 => (),
        _ => {
            println!("Usage: showtemp <station>");
            println!("Stations: Three letter code like \"SMA\", see http://data.netcetera.com/smn/smn/");
            return;    
        }
    };
    let station = &*args[1];

    // Instantiate a HTTP client
    let mut client = Client::new();

    // Connect to server
    let url = format!("http://data.netcetera.com/smn/smn/{}/", station);
    let mut res = match client.get(url.as_slice()).send() {
        Ok(res) => res,
        Err(err) => panic!("Failed to connect: {}", err)
    };

    // Parse JSON
    let body = match res.read_to_string() {
        Ok(body) => body,
        Err(e) => panic!("Error while reading body: {}", e)
    };
    let measurement: Measurement = json::decode(body.as_slice()).unwrap();

    // Output data
    println!("{}C {}h {}mm", measurement.temperature, measurement.sunshine, measurement.precipitation);
}
