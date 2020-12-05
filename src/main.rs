extern crate clap;
use clap::{Arg, App};
use ureq::json;
use serde::Deserialize;


#[derive(Deserialize)]
struct IamResponse {
    access_token: String,
    _expiration: i32,
    expires_in: i32,
    _ims_user_id: i32,
    _refresh_token: String,
    _refresh_token_expiration: i32,
    _scope: String,
    _token_type: String
}


fn main() {
    let matches = App::new("Rust Template")
        .version("1.0")
        .author("Vanderlei Munhoz <vnderlev@protonmail.ch>")
        .about("CLI for Challenge 9 - MBTC 2020")
        .arg(
            Arg::with_name("email")
                .short("e")
                .long("email")
                .value_name("EMAIL ADDRESS")
                .help("Sets your e-mail address")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("wml_url")
                .short("w")
                .long("wml_url")
                .value_name("WML URL")
                .help("Sets your deployed ML pipeline scoring endpoint URL")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("api_key")
                .short("a")
                .long("api_key")
                .value_name("IBM CLOUD API KEY")
                .help("Sets your IBM Cloud API key")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    // We can safely use unwrap() here because the args are required by clap
    let email = matches.value_of("email").unwrap_or("no_email");
    let wml_url = matches.value_of("wml_url").unwrap_or("no_wml_url");
    
    
    // requesting IAM token from IBM Cloud
    let resp = ureq::post("https://iam.cloud.ibm.com/identity/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Accept", "application/json")
        .send_form(&[
            ("apikey", &api_key),
            ("grant_type", "urn:ibm:params:oauth:grant-type:apikey")
        ]);
    
    // resp.ok() -> Code 200-299
    match resp.ok() {  
        true => {
            println!("\n[OK] IBM Cloud IAM identity/token API request successful");
            let iam_data = resp.into_json_deserialize::<IamResponse>().unwrap();
            println!("\nIAM TOKEN: {}", iam_data.access_token);
            println!("\nEXPIRES IN: {}s", iam_data.expires_in);
            // Submitting model for scoring
            let resp2 = ureq::post("http://172.21.86.186:5000/submit")
                .set("Content-Type", "application/json")
                .set("Accept", "application/json")
                .send_json(
                    json!({
                        "email_addr": &email,
                        "wml_url": ,
                        "iam_token": &iam_data.access_token,
                        "submit_confirmation": false
                    })
                );
            println!("\nResult: {}", resp2.into_string().unwrap())
        },
        false => {
            println!(
                "\n[ERROR] Failed getting IAM token: response_code={}; body={}",
                resp.status(), resp.into_string().unwrap()
            );
        }
    }

}
