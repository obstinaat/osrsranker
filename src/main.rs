use reqwest;
use reqwest::Error;
use tokio;
use std::fs::File;
use std::io::Read;
use serde_json;
use serde::Deserialize;
use std::env;

const URL_BASE: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";

//Weights are subject to change; preferably configurable for each different entry, with custom milestones and custom point counts
//additionally it would be nice if milestones after level 99 were implemented, such as 25m xp. 
//by current weights a maxed player will have 3950 points from skills.


#[derive(Debug, Deserialize)]
struct HiScoreStructure(Vec<HiScoreCategory>);

#[derive(Debug, Deserialize)]
struct HiScoreCategory{
    name: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry{
    name: String,
    milestones: Vec<Milestone>
}

#[derive(Debug, Deserialize)]
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

async fn get_hiscores(username: &str) -> Result<String, Error> {
    let url = String::from(URL_BASE) + username;

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Ok(body)
}

fn calc_points(score: isize, milestones: &Vec<Milestone>) -> isize {
    let mut points = 0;
    for milestone in milestones{
        if score < milestone.0{
            return points;
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


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Open and read the configuration file
    let config = read_config().unwrap();

    let args: Vec<String> = env::args().collect();
    let mut username: &str;

    if args.len() > 1 {
        username = &args[1];
    } else {
        username = "Letharg";
    }

    let hiscoresstring = get_hiscores(username).await.unwrap();
    let mut hiscoreslines = hiscoresstring.lines();

    let mut player_points = EvaluatedHiscores{categories: Vec::new(), points: 0};

    for hiscore_category in &config.0{
        let mut evaluated_category:EvaluatedCategory = EvaluatedCategory { name: hiscore_category.name.to_string(), evaluated_entries: Vec::new(), points: 0 };
        for entry in &hiscore_category.entries{
            let name = &entry.name;
            //For each entry we parse the appropriate hiscore information.
            let line = hiscoreslines.next().unwrap();

            let mut parts = line.split(',');
            let rank = parts.next().unwrap_or("");
            let level = parts.next().unwrap_or("").parse::<isize>().unwrap();
            let level = if level < 0 {0} else {level};
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
        player_points.points += evaluated_category.points;
        player_points.categories.push(evaluated_category);
       
    }

    println!("You have a total of {:?} points!", player_points.points);


    Ok(())
} 