mod server;
use chrono::prelude::Utc;
use clap::{Parser, Subcommand};
use ctrlc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use server::Server;
use std::io::Write;
use std::process;
use std::thread::sleep;
use std::time::Duration;

const FOREST_HOST: &str = "https://forest-china.upwardsware.com";

// NOTE: arguments contruction
#[derive(Parser)]
#[clap(name = "MiniForest")]
#[clap(author = "ch4xer <ch4xer@gmail.com>")]
#[clap(version = "1.1")]
#[clap(about = "A mini program which utilize Forest Api", long_about = None)]
struct ARG {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Start planting trees
    #[clap(arg_required_else_help = true)]
    Start {
        // users' email
        #[clap(long, value_parser)]
        email: String,
        // users' password
        #[clap(long, value_parser)]
        password: String,
        // planting time
        #[clap(long, value_parser)]
        time: u64,
    },
    // Read remained time
    Status {},
    #[clap(arg_required_else_help = true)]
    CheckCoin {
        // users' email
        #[clap(long, value_parser)]
        email: String,
        // users' password
        #[clap(long, value_parser)]
        password: String,
    },
    #[clap(arg_required_else_help = true)]
    CheckTotalTime {
        // users' email
        #[clap(long, value_parser)]
        email: String,
        // users' password
        #[clap(long, value_parser)]
        password: String,
    },
    #[clap(arg_required_else_help = true)]
    CheckHealthTree {
        // users' email
        #[clap(long, value_parser)]
        email: String,
        // users' password
        #[clap(long, value_parser)]
        password: String,
    },
    // check user's info
    #[clap(arg_required_else_help = true)]
    CheckDeadTree {
        // users' email
        #[clap(long, value_parser)]
        email: String,
        // users' password
        #[clap(long, value_parser)]
        password: String,
    },
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: u32,
    user_name: String,
    remember_token: String,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    coin: u32,
    total_minutes: u32,
    health_tree_count: u32,
    dead_tree_count: u32,
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

const SERVER: Server = Server::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // display();
    let args: ARG = ARG::parse();

    match args.command {
        Commands::Start {
            email,
            password,
            time,
        } => {
            let user = login(email, password).await?;
            ctrlc::set_handler(move || {
                SERVER.destruct();
                process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");
            plant(user, time).await?;
        }
        Commands::Status {} => {
            println!("{}", read_status());
        }
        Commands::CheckCoin { email, password } => {
            let user = login(email, password).await?;
            println!("{}",get_plant_info(user, "coin").await?);
        }
        Commands::CheckDeadTree { email, password } => {

            let user = login(email, password).await?;
            println!("{}",get_plant_info(user, "deadtree").await?);
        }
        Commands::CheckTotalTime { email, password } => {
            let user = login(email, password).await?;
            println!("{}",get_plant_info(user, "totaltime").await?);
        }
        Commands::CheckHealthTree { email, password } => {
            let user = login(email, password).await?;
            println!("{}",get_plant_info(user, "healthtree").await?);
        }
    }

    Ok(())
}

async fn login(email: String, password: String) -> Result<User, reqwest::Error> {
    let auth = Auth { email, password };

    let login_data = LoginData { session: auth };
    let login_route = format!("{}{}", FOREST_HOST, "/api/v1/sessions");

    let http_client = reqwest::Client::new();
    let resp = http_client.post(login_route).json(&login_data).send().await?;

    let user = resp.json::<User>().await?;
    return Ok(user);
}

async fn get_plant_info(user: User, key: &str) -> Result<String, reqwest::Error> {
    let info_route = format!("{}{}{}", FOREST_HOST, "/api/v1/users/", user.user_id);
    let http_client = reqwest::Client::new();
    let resp = http_client
        .get(info_route)
        .header("Cookie", format!("remember_token={}", user.remember_token))
        .send()
        .await?;
    let info = resp.json::<UserInfo>().await?;
    match key {
        "coin" => {return Ok(info.coin.to_string())}
        "totaltime" => {return Ok(info.total_minutes.to_string())}
        "deadtree" => {return Ok(info.dead_tree_count.to_string())}
        "healthtree" => {return Ok(info.health_tree_count.to_string());}
        _ => {return Ok("".to_string())}
    }
}

async fn plant(user: User, time: u64) -> Result<(), reqwest::Error> {
    let start_time = get_current_time();

    // println!("Plant Start: {}",start_time);
    for i in 0..time * 60 {
        // write time to file for reading
        let time_string = format!("{:0>2}:{:0>2}", (time * 60 - i) / 60, (60 - i % 60) % 60);
        SERVER.write_status(&time_string);

        print!("\r{:0>2}:{:0>2}", (time * 60 - i) / 60, (60 - i % 60) % 60);
        sleep(Duration::new(1, 0));
        #[warn(unused_must_use)]
        {
            std::io::stdout().flush().unwrap();
        }
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
    let resp = http_client
        .post(plant_url)
        .json(&plant_data)
        .header("Cookie", format!("remember_token={}", user.remember_token))
        .send()
        .await?
        .text()
        .await?;
    if resp.contains(r#""is_success":true"#) {
        print!("\rSuccess!");
        Ok(())
    } else {
        print!("\rFailed!");
        Ok(())
    }
}

fn get_current_time() -> String {
    let now_utc = Utc::now().to_rfc3339();
    let dot_index = now_utc.chars().position(|c| c == '.').unwrap();
    let now_time = format!("{}{}", &now_utc[..dot_index + 4], "Z");
    return now_time;
}

fn read_status() -> String {
    SERVER.read_status()
}
