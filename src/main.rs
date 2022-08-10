mod server;
use std::process;
use ctrlc;
use serde::{Serialize, Deserialize};
use serde_json::json;
use chrono::prelude::*;
use server::Server;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use clap::{Parser, Subcommand};

const FOREST_HOST: &str = "https://forest-china.upwardsware.com";

// NOTE: arguments contruction
#[derive(Parser)]
#[clap(name = "MiniForest")]
#[clap(author = "ch4xer <ch4xer@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "A mini program which utilize Forest Api", long_about = None)]
struct ARG {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Start planting trees
    #[clap(arg_required_else_help = true)]
    Start {
        /// users' email
        #[clap(long, value_parser)]
        email: String,
        /// users' password
        #[clap(long, value_parser)]
        password: String,
        /// planting time
        #[clap(long, value_parser)]
        time: u64,
    },
    /// Read remained time
    Status {
    }
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

const SERVER:Server = Server::new();


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // display();
    let args: ARG = ARG::parse();

    match args.command {
        Commands::Start {email, password, time} => {

            login(email, password).await?;
            ctrlc::set_handler(move || {
                SERVER.destruct();
                process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");
            plant(time).await?;
        },
        Commands::Status {} => {
            println!("{}", read_status());
        }
    }

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

        USER = resp.json::<User>().await?;
    }

    Ok(())
}

async fn plant(time: u64) -> Result<(), reqwest::Error> {
    let start_time = get_current_time();

    // println!("Plant Start: {}",start_time);
    for i in 0..time*60 {

        // write time to file for reading
        let time_string = format!("{:0>2}:{:0>2}", (time*60-i)/60, (60 - i %60)%60);
        SERVER.write_status(&time_string);

        print!("\r{:0>2}:{:0>2}", (time*60-i)/60, (60 - i %60)%60);
        sleep(Duration::new(1, 0));
        #[warn(unused_must_use)]
        {std::io::stdout().flush().unwrap();}
    }

    SERVER.destruct();

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

fn read_status() -> String {
    SERVER.read_status()
}
