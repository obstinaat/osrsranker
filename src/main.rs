use reqwest::{self, Body};
use reqwest::Error;
use tokio;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::{self, json};
use serde::Deserialize;
use std::{env, time};
use std::io::prelude::*;
use std::thread;
use tokio::task;

const HISCORES_URL_BASE: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";
const POINTS_URL: &str = "https://api.bapheads.com/bap-points";
const USERS_URL: &str = "https://api.bapheads.com/users";

//Weights are subject to change; preferably configurable for each different entry, with custom milestones and custom point counts
//additionally it would be nice if milestones after level 99 were implemented, such as 25m xp. 
//by current weights a maxed player will have 3950 points from skills.

#[derive(Debug, Deserialize, Clone)]
struct HiScoreStructure(Vec<HiScoreCategory>);

#[derive(Debug, Deserialize, Clone)]
struct HiScoreCategory{
    name: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Clone)]
struct Entry{
    name: String,
    milestones: Vec<Milestone>
}

#[derive(Debug, Deserialize, Clone)]
struct Milestone(isize,isize);

#[derive(Debug, Deserialize)]
struct EvaluatedHiscores {
    categories: Vec<EvaluatedCategory>,
    points: isize,
}

#[derive(Debug, Deserialize)]
struct EvaluatedCategory {
    name: String,
    evaluated_entries: Vec<EvaluatedEntry>,
    points: isize,
}

#[derive(Debug, Deserialize)]
struct EvaluatedEntry{
    name: String,
    score: isize,
    points: isize,
}

#[derive(Debug, Deserialize)]
struct apiScore{
    username: String,
    total: isize,
    skilling: isize,
    minigamesClues: isize,
    pvm: isize,
}

#[derive(Debug, Deserialize, Clone)]
struct PlayerList(Vec<Player>);

#[derive(Debug, Deserialize, Clone)]
struct Player{
    username: String,
}

async fn get_hiscores(username: &str) -> Result<String, Error> {
    let hiscore_url = String::from(HISCORES_URL_BASE) + &username;

    let res = reqwest::get(hiscore_url).await?;
    let body = res.text().await?;

    Ok(body)
}

fn calc_points(score: isize, milestones: &Vec<Milestone>) -> isize {
    let mut points = 0;
    for milestone in milestones{
        if milestone.0 >= 0{
            //Milestones
            if score < milestone.0{
                return points;
            }
        } else {
            //Points per kill
            points += ((score / (milestone.1 * (milestone.0 * -1))) as isize) * 3
        }
        points += milestone.1;
    }
    points
}


fn read_config() -> Result<HiScoreStructure, Box<dyn std::error::Error>> {
    let file_path = "config/config.json";

    let mut file = File::open(file_path)?;
    let mut config_json = String::new();
    file.read_to_string(&mut config_json)?;

    // Deserialize the JSON into your configuration struct
    let config: HiScoreStructure = serde_json::from_str(&config_json)?;

    Ok(config)
}

async fn send_scores(username: &str, total_score: isize, activities_score: isize, skilling_score: isize, pvm_score: isize) -> Result<&str, Error> {
    //Attempt 2
    // let json = format!(r#"{{"username": {}, "total": {}, "skilling": {}, "minigamesClues": {}, "pvm": {}}}"#, username, total_score, skilling_score, activities_score, pvm_score) ;

    let borrowed_total_score = &total_score.to_string();
    let borrowed_skilling_score = &skilling_score.to_string();
    let borrowed_activities_score = &activities_score.to_string();
    let borrowed_pvm_score = &pvm_score.to_string();

    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("total", borrowed_total_score);
    map.insert("skilling", borrowed_skilling_score);
    map.insert("minigamesClues", borrowed_activities_score);
    map.insert("pvm", borrowed_pvm_score);
    let client = reqwest::Client::new();
    let res = client.post(POINTS_URL)
    .json(&map)
    .send()
    .await?
    .text()
    .await?;

    Ok(("Done"))

}

fn writefile(text: &str) -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(text.as_bytes())?;
    Ok(())
}



async fn process(config: HiScoreStructure, usernames: Vec<String>) -> Result<(), Error>{
        for username in usernames{

        let mut hiscoresstring = get_hiscores(&username).await?;
        if hiscoresstring.starts_with("<!DOCTYPE html><html><head><title>404"){
            println!("Hiscores not found for user: {:?}.", username);
            return Ok(())
        }
        while hiscoresstring.starts_with('<') {
            thread::sleep(time::Duration::from_secs(1));
            hiscoresstring = get_hiscores(&username).await?;
            if hiscoresstring.starts_with("<!DOCTYPE html><html><head><title>404"){
            println!("Hiscores not found for user: {:?}.", username);
            return Ok(())
        }
        }

        let mut hiscoreslines = hiscoresstring.lines();

        let mut player_points = EvaluatedHiscores{categories: Vec::new(), points: 0};

        'outer: for hiscore_category in &config.0{
            let mut evaluated_category:EvaluatedCategory = EvaluatedCategory { name: hiscore_category.name.to_string(), evaluated_entries: Vec::new(), points: 0 };
            for entry in &hiscore_category.entries{
                let name = &entry.name;
                //For each entry we parse the appropriate hiscore information.
                let line = hiscoreslines.next().unwrap();

                //parse the hiscore info we have                
                let mut parts = line.split(',');
                let rank = parts.next().unwrap_or("");
                
                let lvltext = parts.next().unwrap_or("");
                let mut level:isize;

                match(lvltext.parse::<isize>()){
                    Ok(mut level) => {
                        level = if level < 0 {0} else {level};
                        let mut score = 0;
                        if (hiscore_category.name=="skills".to_string()){
                            score = parts.next().unwrap_or("").parse::<isize>().unwrap();
                            score = if score < 0 {0} else {score};
                        } else {
                            score = level;
                        } 

                        let points = calc_points(score, &entry.milestones);
                        let evaluated_entry = EvaluatedEntry{name: name.to_string(),score,points};
                        evaluated_category.points+=evaluated_entry.points;
                        evaluated_category.evaluated_entries.push(evaluated_entry);
                    }
                    Err(_) => break 'outer
                }
            }
            player_points.points += evaluated_category.points;
            player_points.categories.push(evaluated_category);
        }
        
        println!("Updated: {:?}.", username);

        let total_points = player_points.points;
        let pvm_points  = player_points.categories.pop().unwrap().points;
        let activities_points = player_points.categories.pop().unwrap().points;
        let skilling_points = player_points.categories.pop().unwrap().points;
        
        if total_points > 1 {
        let _ = send_scores(&username, total_points, activities_points, skilling_points, pvm_points).await;
        }
    }
    Ok(())
}




#[tokio::main]
async fn main() -> Result<(), Error> {
    // Open and read the configuration file
    let config = read_config().unwrap();

    let args: Vec<String> = env::args().collect();

    let body = reqwest::get(USERS_URL)
    .await?
    .json::<PlayerList>()
    .await?;

    let mut usernames: Vec<String> = Vec::new();
    
    for player in &body.0{
        usernames.push(player.username.clone());
    }

    let num_pieces = 40;
    let mut pieces = Vec::new();
    let piece_size = (usernames.len() + num_pieces - 1) / num_pieces;

    for chunk in usernames.chunks(piece_size) {
        pieces.push(chunk.to_vec());
    }

    let mut handles = Vec::with_capacity(num_pieces);

    for piece in pieces {
        let copy_config = config.clone();
         let handle = task::spawn( async move {       
            let _ = process(copy_config, piece.to_vec()).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        // Wait for the thread to finish and get its result
        let result = handle.await.unwrap();

    }
    
    
    Ok(())
} 