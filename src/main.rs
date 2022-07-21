// use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use chrono::prelude::*;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use clap::Parser;

const FOREST_HOST: &str = "https://forest-china.upwardsware.com";

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct ARG {
    #[clap(short, value_parser)]
    email: String,
    #[clap(short, value_parser)]
    password: String,
    #[clap(short, value_parser)]
    time: u64,
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: u32,
    user_name: String,
    remember_token: String,
}

#[derive(Serialize, Deserialize)]
struct Auth {
    email: String,
    password: String,
}


#[derive(Serialize, Deserialize)]
struct LoginData {
    session: Auth,
}

static mut USER: User = User{
    user_id: 0,
    user_name: String::new(),
    remember_token: String::new(),
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // display();
    let args: ARG = ARG::parse();

    login(args.email, args.password).await?;

    plant(args.time).await?;
    Ok(())
}

async fn login(email: String, password: String) -> Result<(), reqwest::Error> {
    unsafe {
        let auth = Auth {
            email,
            password,
        };

        let login_data = LoginData {
            session: auth,
        };
        let login_route = "/api/v1/sessions";
        let mut login_url = String::from(FOREST_HOST);
        login_url.push_str(login_route);


        let http_client = reqwest::Client::new();
        let resp = http_client.post(login_url)
            .json(&login_data)
            .send()
            .await?;

        // println!("{}", &resp.status());
        USER = resp.json::<User>().await?;
    }

    // async fn login(email: &str, password: &str) -> Result<(), reqwest::Error> {

    Ok(())
}

async fn plant(time: u64) -> Result<(), reqwest::Error> {
    let start_time = get_current_time();

    // println!("Plant Start: {}",start_time);
    for i in 0..time*60 {
        print!("\r{:0>2}:{:0>2}", (time*60-i)/60, (60 - i %60)%60);
        sleep(Duration::new(1, 0));
        #[warn(unused_must_use)]
        {std::io::stdout().flush().unwrap();}
    }

    let end_time = get_current_time();
    // println!("Plant End: {}", end_time);

    let plant_data = json!({
        "plant": {
            "tree_type_gid": 0,
            "start_time": start_time,
            "end_time": end_time,
            "tag": 0,
            "note": "",
            "is_success": true,
            "updated_at": end_time,
            "trees": [{
                "is_dead": false,
                "phase": 4,
                "tree_type": 0
            }, {
                "is_dead": false,
                "phase": 5,
                "tree_type": 0
            }]
        }
    });


    let plant_route = "/api/v1/plants";
    let mut plant_url = String::from(FOREST_HOST);
    plant_url.push_str(plant_route);


    let http_client = reqwest::Client::new();
    unsafe {
        let resp = http_client.post(plant_url)
            .json(&plant_data)
            .header("Cookie", format!("remember_token={}", USER.remember_token))
            .send()
            .await
            .expect("failed to get response")
            .text()
            .await?;
        if resp.contains(r#""is_success":true"#){
            print!("\rSuccess!");
            Ok(())
        } else {
            print!("\rFailed!");
            Ok(())
        }
    }
}

fn get_current_time() -> String {
    let now_utc  = Utc::now().to_rfc3339();
    let dot_index = now_utc.chars().position(|c| c == '.').unwrap();
    let now_time = format!("{}{}", &now_utc[..dot_index+4], "Z");
    return now_time;
}

