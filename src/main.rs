use std::env;
use reqwest;
use reqwest::Error;
use tokio;

const URL_BASE: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";



//Weights are subject to change; preferably configurable for each different entry, with custom milestones and custom point counts
//additionally it would be nice if milestones after level 99 were implemented, such as 25m xp. 
//by current weights a maxed player will have 3950 points from skills.

//Milestones: level 50, 60, 70, 75, 80, 85, 90, 92 ,95, 99.
const SKILL_MILESTONES: [isize; 10] = [10133, 273742, 737627, 1210421, 1986068, 3258594, 5346332, 6517253, 8771558, 13034431];
const EASY_SKILL: [isize; 10] = [1, 1, 2, 3, 5, 7, 10, 10, 15, 25];
const MEDIUM_SKILL: [isize; 10] = [2, 2, 4, 6, 10, 14, 20, 20, 30, 50];
const HARD_SKILL: [isize; 10] = [4, 4, 8, 12, 20, 28, 40, 40, 60, 100];

#[derive(Debug)]
#[allow(dead_code)]
struct ParsedHiScoresData {
    //SKILLS
    overall: isize,
    attack: isize,
    defence: isize,
    strength: isize,
    hitpoints: isize,
    ranged: isize,
    prayer: isize,
    magic: isize,
    cooking: isize,
    woodcutting: isize,
    fletching: isize,
    fishing: isize,
    firemaking: isize,
    crafting: isize,
    smithing: isize,
    mining: isize,
    herblore: isize,
    agility: isize,
    thieving: isize,
    slayer: isize,
    farming: isize,
    runecrafting: isize,
    hunter: isize,
    construction: isize,
    //activities
    league_points: isize,
    deadman_points: isize,
    bounty_hunter_hunter: isize,
    bounty_hunter_rogue: isize,
    bounty_hunter_legacy_hunter: isize,
    bounty_hunter_legacy_rogue: isize,
    clue_scrolls_all: isize,
    clue_scrolls_beginner: isize,
    clue_scrolls_easy: isize,
    clue_scrolls_medium: isize,
    clue_scrolls_hard: isize,
    clue_scrolls_elite: isize,
    clue_scrolls_master: isize,
    lms_ranked: isize,
    pvp_arena: isize,
    soul_wars_zeal: isize,
    gotr_rifts: isize,
    //PVM
    abyssal_sire: isize,
    alchemical_hydra: isize,
    artio: isize,
    barrows: isize,
    bryophyta: isize,
    callisto: isize,
    calvarion: isize,
    cerberus: isize,
    cox: isize,
    cox_cm: isize,
    chaos_elemental: isize,
    chaos_fanatic: isize,
    commander_zilyana:isize,
    corporeal_beast: isize,
    crazy_archeologist: isize,
    dagannoth_prime: isize,
    dagannoth_rex: isize,
    dagannoth_supreme: isize,
    deranged_archaeologist: isize,
    duke_sucellus: isize,
    general_graardor: isize,
    giant_mole: isize,
    grotesque_guardians: isize,
    hespori: isize,
    kalphite_queen: isize,
    king_black_dragon: isize,
    kraken: isize,
    kreearra: isize,
    kril_tsutsaroth: isize,
    mimic: isize,
    nex: isize,
    nightmare: isize,
    phosani_nightmare: isize,
    obor: isize,
    phantom_muspah: isize,
    sarachnis: isize,
    scorpia: isize,
    skotizo: isize,
    spindel: isize,
    tempoross: isize,
    gauntlet: isize,
    corrupted_gauntlet: isize,
    leviathan: isize,
    whisperer: isize,
    tob: isize,
    tob_hard_mode: isize,
    thermy: isize,
    toa: isize,
    toa_expert_mode: isize,
    tzkal_zuk: isize,
    tztok_jad: isize,
    vardorvis: isize,
    venenatis: isize,
    vetion: isize,
    vorkath: isize,
    wintertodt: isize,
    zalcano: isize,
    zulrah: isize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PointsFromSkills{
    overall: isize,
    attack: isize,
    defence: isize,
    strength: isize,
    hitpoints: isize,
    ranged: isize,
    prayer: isize,
    magic: isize,
    cooking: isize,
    woodcutting: isize,
    fletching: isize,
    fishing: isize,
    firemaking: isize,
    crafting: isize,
    smithing: isize,
    mining: isize,
    herblore: isize,
    agility: isize,
    thieving: isize,
    slayer: isize,
    farming: isize,
    runecrafting: isize,
    hunter: isize,
    construction: isize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PointsFromActivities{
    total: isize,
    league_points: isize,
    deadman_points: isize,
    bounty_hunter_hunter: isize,
    bounty_hunter_rogue: isize,
    bounty_hunter_legacy_hunter: isize,
    bounty_hunter_legacy_rogue: isize,
    clue_scrolls_all: isize,
    clue_scrolls_beginner: isize,
    clue_scrolls_easy: isize,
    clue_scrolls_medium: isize,
    clue_scrolls_hard: isize,
    clue_scrolls_elite: isize,
    clue_scrolls_master: isize,
    lms_ranked: isize,
    pvp_arena: isize,
    soul_wars_zeal: isize,
    gotr_rifts: isize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PointsFromPVM{
    total: isize,
    abyssal_sire: isize,
    alchemical_hydra: isize,
    artio: isize,
    barrows: isize,
    bryophyta: isize,
    callisto: isize,
    calvarion: isize,
    cerberus: isize,
    cox: isize,
    cox_cm: isize,
    chaos_elemental: isize,
    chaos_fanatic: isize,
    commander_zilyana:isize,
    corporeal_beast: isize,
    crazy_archeologist: isize,
    dagannoth_prime: isize,
    dagannoth_rex: isize,
    dagannoth_supreme: isize,
    deranged_archaeologist: isize,
    duke_sucellus: isize,
    general_graardor: isize,
    giant_mole: isize,
    grotesque_guardians: isize,
    hespori: isize,
    kalphite_queen: isize,
    king_black_dragon: isize,
    kraken: isize,
    kreearra: isize,
    kril_tsutsaroth: isize,
    mimic: isize,
    nex: isize,
    nightmare: isize,
    phosani_nightmare: isize,
    obor: isize,
    phantom_muspah: isize,
    sarachnis: isize,
    scorpia: isize,
    skotizo: isize,
    spindel: isize,
    tempoross: isize,
    gauntlet: isize,
    corrupted_gauntlet: isize,
    leviathan: isize,
    whisperer: isize,
    tob: isize,
    tob_hard_mode: isize,
    thermy: isize,
    toa: isize,
    toa_expert_mode: isize,
    tzkal_zuk: isize,
    tztok_jad: isize,
    vardorvis: isize,
    venenatis: isize,
    vetion: isize,
    vorkath: isize,
    wintertodt: isize,
    zalcano: isize,
    zulrah: isize,
}

async fn get_hiscores(username: &str) -> Result<String, Error> {
    let url = String::from(URL_BASE) + username;

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Ok(body)
}

fn get_points_easy_skill(experience: isize) -> isize{
    let mut points: isize = 0;
    for (i, milestone) in SKILL_MILESTONES.iter().enumerate(){
        if experience < *milestone {
            return points
        }
        points += EASY_SKILL[i];
    }
    points
}

fn get_points_medium_skill(experience: isize) -> isize{
    let mut points: isize = 0;
    for (i, milestone) in SKILL_MILESTONES.iter().enumerate(){
        if experience < *milestone {
            return points
        }
        points += MEDIUM_SKILL[i];
    }
    points
}

fn get_points_hard_skill(experience: isize) -> isize{
    let mut points: isize = 0;
    for (i, milestone) in SKILL_MILESTONES.iter().enumerate(){
        if experience < *milestone {
            return points
        }
        points += HARD_SKILL[i];
    }
    points
}

fn get_points_from_skills(playerdata: &ParsedHiScoresData) -> PointsFromSkills{
    //EASY SKILLS
    let attack = get_points_easy_skill(playerdata.attack);
    let defence = get_points_easy_skill(playerdata.defence);
    let strength = get_points_easy_skill(playerdata.strength);
    let hitpoints = get_points_easy_skill(playerdata.hitpoints);
    let ranged = get_points_easy_skill(playerdata.ranged);
    let cooking = get_points_easy_skill(playerdata.cooking);
    let woodcutting = get_points_easy_skill(playerdata.woodcutting);
    let firemaking = get_points_easy_skill(playerdata.firemaking);
    let thieving = get_points_easy_skill(playerdata.thieving);
    let hunter = get_points_easy_skill(playerdata.hunter);


    //MEDIUM SKILLS
    let magic = get_points_medium_skill(playerdata.magic);
    let fletching = get_points_medium_skill(playerdata.fletching);
    let fishing = get_points_medium_skill(playerdata.fishing);
    let crafting = get_points_medium_skill(playerdata.crafting);
    let smithing = get_points_medium_skill(playerdata.smithing);
    let farming = get_points_medium_skill(playerdata.farming);

    //HARD SKILLS
    let prayer = get_points_hard_skill(playerdata.prayer);
    let mining = get_points_hard_skill(playerdata.mining);
    let herblore = get_points_hard_skill(playerdata.herblore);
    let agility = get_points_hard_skill(playerdata.agility);
    let slayer = get_points_hard_skill(playerdata.slayer);
    let runecrafting = get_points_hard_skill(playerdata.runecrafting);
    let construction = get_points_hard_skill(playerdata.construction);
    
    let total = attack + defence + strength + hitpoints + ranged + cooking +
    woodcutting + firemaking + thieving + hunter + magic + fletching + fishing + 
    crafting + smithing + farming + prayer + mining + herblore + agility + 
    slayer + runecrafting + construction;

    PointsFromSkills{
        overall: total,
        attack: attack,
        defence: defence,
        strength: strength,
        hitpoints: hitpoints,
        ranged: ranged,
        prayer: prayer,
        magic: magic,
        cooking: cooking,
        woodcutting: woodcutting,
        fletching: fletching,
        fishing: fishing,
        firemaking: firemaking,
        crafting: crafting,
        smithing: smithing,
        mining: mining,
        herblore: herblore,
        agility: agility,
        thieving: thieving,
        slayer: slayer,
        farming: farming,
        runecrafting: runecrafting,
        hunter: hunter,
        construction: construction,
    }
}

fn get_points_from_activities(playerdata: &ParsedHiScoresData) -> PointsFromActivities{
    let league_points = 0;
    let deadman_points = 0;
    let bounty_hunter_hunter = 0;
    let bounty_hunter_rogue = 0;
    let bounty_hunter_legacy_hunter = 0;
    let bounty_hunter_legacy_rogue = 0;
    let clue_scrolls_beginner = (playerdata.clue_scrolls_beginner as f64 * 0.05).floor() as isize;
    let clue_scrolls_easy = (playerdata.clue_scrolls_easy as f64 * 0.05).floor() as isize;
    let clue_scrolls_medium = (playerdata.clue_scrolls_medium as f64 * 0.1).floor() as isize;
    let clue_scrolls_hard = (playerdata.clue_scrolls_hard as f64 * 0.2).floor() as isize;
    let clue_scrolls_elite = (playerdata.clue_scrolls_elite as f64 * 0.25).floor() as isize;
    let clue_scrolls_master = (playerdata.clue_scrolls_master as f64 * 0.5).floor() as isize;
    let clue_scrolls_all = 0; // no additional need to give more points.
    let lms_ranked = 0;
    let pvp_arena = 0;
    let soul_wars_zeal = 0;
    let gotr_rifts = (playerdata.gotr_rifts as f64 * 0.2).floor() as isize;
    let total = league_points + deadman_points + bounty_hunter_hunter + bounty_hunter_rogue + bounty_hunter_legacy_hunter +
     bounty_hunter_legacy_rogue + clue_scrolls_all + clue_scrolls_beginner + clue_scrolls_easy + clue_scrolls_medium +
      clue_scrolls_hard + clue_scrolls_elite + clue_scrolls_master + lms_ranked + pvp_arena + soul_wars_zeal + gotr_rifts;


    PointsFromActivities{
        total: total,
        league_points: league_points,
        deadman_points: deadman_points,
        bounty_hunter_hunter: bounty_hunter_hunter,
        bounty_hunter_rogue: bounty_hunter_rogue,
        bounty_hunter_legacy_hunter: bounty_hunter_legacy_hunter,
        bounty_hunter_legacy_rogue: bounty_hunter_legacy_rogue,
        clue_scrolls_all: clue_scrolls_all,
        clue_scrolls_beginner: clue_scrolls_beginner,
        clue_scrolls_easy: clue_scrolls_easy,
        clue_scrolls_medium: clue_scrolls_medium,
        clue_scrolls_hard: clue_scrolls_hard,
        clue_scrolls_elite: clue_scrolls_elite,
        clue_scrolls_master: clue_scrolls_master,
        lms_ranked: lms_ranked,
        pvp_arena: pvp_arena,
        soul_wars_zeal: soul_wars_zeal,
        gotr_rifts: gotr_rifts,
    }
}

fn get_points_from_pvm(playerdata: &ParsedHiScoresData) -> PointsFromPVM{
    let abyssal_sire = (playerdata.abyssal_sire as f64 * 1 as f64/45 as f64).floor() as isize;
    let alchemical_hydra = (playerdata.alchemical_hydra as f64 * 1 as f64/33 as f64).floor() as isize;
    let artio = (playerdata.artio as f64 * 1 as f64/65 as f64).floor() as isize;
    let barrows = (playerdata.barrows as f64 * 1 as f64/20 as f64).floor() as isize;
    let bryophyta = (playerdata.bryophyta as f64 * 1 as f64/5 as f64).floor() as isize;
    let callisto = (playerdata.callisto as f64 * 1 as f64/85 as f64).floor() as isize;
    let calvarion = (playerdata.calvarion as f64 * 1 as f64/55 as f64).floor() as isize;
    let cerberus = (playerdata.cerberus as f64 * 1 as f64/61 as f64).floor() as isize;
    let cox = (playerdata.cox as f64 * 1 as f64/3 as f64).floor() as isize;
    let cox_cm = (playerdata.cox_cm as f64 * 1 as f64/2.4 as f64).floor() as isize;
    let chaos_elemental = (playerdata.chaos_elemental as f64 * 1 as f64/60 as f64).floor() as isize;
    let chaos_fanatic = (playerdata.chaos_fanatic as f64 * 1 as f64/100 as f64).floor() as isize;
    let commander_zilyan = (playerdata.commander_zilyana as f64 * 1 as f64/55 as f64).floor() as isize;
    let corporeal_beast = (playerdata.corporeal_beast as f64 * 1 as f64/60 as f64).floor() as isize;
    let crazy_archeologist = (playerdata.crazy_archeologist as f64 * 1 as f64/60 as f64).floor() as isize;
    let dagannoth_prime = (playerdata.dagannoth_prime as f64 * 1 as f64/100 as f64).floor() as isize;
    let dagannoth_rex = (playerdata.dagannoth_rex as f64 * 1 as f64/100 as f64).floor() as isize;
    let dagannoth_supreme = (playerdata.dagannoth_supreme as f64 * 1 as f64/100 as f64).floor() as isize;
    let deranged_archaeologist = (playerdata.deranged_archaeologist as f64 * 1 as f64/100 as f64).floor() as isize;
    let duke_sucellus = (playerdata.duke_sucellus as f64 * 1 as f64/24 as f64).floor() as isize;
    let general_graardor = (playerdata.general_graardor as f64 * 1 as f64/55 as f64).floor() as isize;
    let giant_mole = (playerdata.giant_mole as f64 * 1 as f64/100 as f64).floor() as isize;
    let grotesque_guardians = (playerdata.grotesque_guardians as f64 * 1 as f64/36 as f64).floor() as isize;
    let hespori = (playerdata.hespori as f64 * 0.5).floor() as isize;
    let kalphite_queen = (playerdata.kalphite_queen as f64 * 1 as f64/50 as f64).floor() as isize;
    let king_black_dragon = (playerdata.king_black_dragon as f64 * 1 as f64/120 as f64).floor() as isize;
    let kraken = (playerdata.kraken as f64 * 1 as f64/100 as f64).floor() as isize;
    let kreearra = (playerdata.kreearra as f64 * 1 as f64/40 as f64).floor() as isize;
    let kril_tsutsaroth = (playerdata.kril_tsutsaroth as f64 * 1 as f64/65 as f64).floor() as isize;
    let mimic = (playerdata.mimic as f64 * 1 as f64/3 as f64).floor() as isize;
    let nex = (playerdata.nex as f64 * 1 as f64/13 as f64).floor() as isize;
    let nightmare = (playerdata.nightmare as f64 * 1 as f64/30 as f64).floor() as isize;
    let phosani_nightmare = (playerdata.phosani_nightmare as f64 * 1 as f64/7 as f64).floor() as isize;
    let obor = (playerdata.obor as f64 * 0.5).floor() as isize;
    let phantom_muspah = (playerdata.phantom_muspah as f64 * 1 as f64/25 as f64).floor() as isize;
    let sarachnis = (playerdata.sarachnis as f64 * 1 as f64/80 as f64).floor() as isize;
    let scorpia = (playerdata.scorpia as f64 * 1 as f64/130 as f64).floor() as isize;
    let skotizo = (playerdata.skotizo as f64 * 1 as f64/45 as f64).floor() as isize;
    let spindel = (playerdata.spindel as f64 * 1 as f64/55 as f64).floor() as isize;
    let tempoross = (playerdata.tempoross as f64 * 0.06).floor() as isize;
    let gauntlet = (playerdata.gauntlet as f64 * 1 as f64/10 as f64).floor() as isize;
    let corrupted_gauntlet = (playerdata.corrupted_gauntlet as f64 * 1 as f64/7 as f64).floor() as isize;
    let leviathan = (playerdata.leviathan as f64 * 1 as f64/30 as f64).floor() as isize;
    let whisperer = (playerdata.whisperer as f64 * 1 as f64/21 as f64).floor() as isize;
    let tob = (playerdata.tob as f64 * 1 as f64/3 as f64).floor() as isize;
    let tob_hard_mode = (playerdata.tob_hard_mode as f64 * 1 as f64/3 as f64).floor() as isize;
    let thermy = (playerdata.thermy as f64 * 1 as f64/125 as f64).floor() as isize;
    let toa = (playerdata.toa as f64 * 1 as f64/3.5 as f64).floor() as isize;
    let toa_expert_mode = (playerdata.toa_expert_mode as f64 * 1 as f64/3 as f64).floor() as isize;
    let tzkal_zuk = (playerdata.tzkal_zuk as f64 * 1 as f64/0.8 as f64).floor() as isize;
    let tztok_jad = (playerdata.tztok_jad as f64 * 1 as f64/2 as f64).floor() as isize;
    let vardorvis = (playerdata.vardorvis as f64 * 1 as f64/37 as f64).floor() as isize;
    let venenatis = (playerdata.venenatis as f64 * 1 as f64/20 as f64).floor() as isize;
    let vetion = (playerdata.vetion as f64 * 1 as f64/39 as f64).floor() as isize;
    let vorkath = (playerdata.vorkath as f64 * 1 as f64/34 as f64).floor() as isize;
    let wintertodt = (playerdata.wintertodt as f64 * 1 as f64/30 as f64).floor() as isize;
    let zalcano = (playerdata.zalcano as f64 * 0.05).floor() as isize;
    let zulrah = (playerdata.zulrah as f64 * 1 as f64/40 as f64).floor() as isize;

    let total = abyssal_sire + alchemical_hydra + artio + barrows + bryophyta + callisto + calvarion +
     cerberus + cox + cox_cm + chaos_elemental + chaos_fanatic + commander_zilyan + corporeal_beast +
     crazy_archeologist + dagannoth_prime + dagannoth_rex + dagannoth_supreme + deranged_archaeologist +
     duke_sucellus + general_graardor + giant_mole + grotesque_guardians + hespori + kalphite_queen +
     king_black_dragon + kraken + kreearra + kril_tsutsaroth + mimic + nex + nightmare + phosani_nightmare +
     obor + phantom_muspah + sarachnis + scorpia + skotizo + spindel + tempoross + gauntlet +
     corrupted_gauntlet + leviathan + whisperer + tob + tob_hard_mode + thermy + toa + toa_expert_mode +
     tzkal_zuk + tztok_jad + vardorvis + venenatis + vetion + vorkath + wintertodt + zalcano + zulrah; 

    PointsFromPVM{
        total: total,
        abyssal_sire: abyssal_sire,
        alchemical_hydra: alchemical_hydra,
        artio: artio,
        barrows: barrows,
        bryophyta: bryophyta,
        callisto: callisto,
        calvarion: calvarion,
        cerberus: cerberus,
        cox: cox,
        cox_cm: cox_cm,
        chaos_elemental: chaos_elemental,
        chaos_fanatic: chaos_fanatic,
        commander_zilyana:commander_zilyan,
        corporeal_beast: corporeal_beast,
        crazy_archeologist: crazy_archeologist,
        dagannoth_prime: dagannoth_prime,
        dagannoth_rex: dagannoth_rex,
        dagannoth_supreme: dagannoth_supreme,
        deranged_archaeologist: deranged_archaeologist,
        duke_sucellus: duke_sucellus,
        general_graardor: general_graardor,
        giant_mole: giant_mole,
        grotesque_guardians: grotesque_guardians,
        hespori: hespori,
        kalphite_queen: kalphite_queen,
        king_black_dragon: king_black_dragon,
        kraken: kraken,
        kreearra: kreearra,
        kril_tsutsaroth: kril_tsutsaroth,
        mimic: mimic,
        nex: nex,
        nightmare: nightmare,
        phosani_nightmare: phosani_nightmare,
        obor: obor,
        phantom_muspah: phantom_muspah,
        sarachnis: sarachnis,
        scorpia: scorpia,
        skotizo: skotizo,
        spindel: spindel,
        tempoross: tempoross,
        gauntlet: gauntlet,
        corrupted_gauntlet: corrupted_gauntlet,
        leviathan: leviathan,
        whisperer: whisperer,
        tob: tob,
        tob_hard_mode: tob_hard_mode,
        thermy: thermy,
        toa: toa,
        toa_expert_mode: toa_expert_mode,
        tzkal_zuk: tzkal_zuk,
        tztok_jad: tztok_jad,
        vardorvis: vardorvis,
        venenatis: venenatis,
        vetion: vetion,
        vorkath: vorkath,
        wintertodt: wintertodt,
        zalcano: zalcano,
        zulrah: zulrah,
    }

}

#[allow(unused_variables)]
fn parse_hiscores(input:String) -> Result<ParsedHiScoresData, Error> {
    let mut lines = input.lines();

    let count = lines.clone().count();

    //Overall
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let overall = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Attack
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let attack = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Defence
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let defence = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Strength
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let strength = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Hitpoints
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let hitpoints = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Ranged
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let ranged = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Prayer
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let prayer = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Magic
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let magic = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Cooking
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let cooking = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Woodcutting
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let woodcutting = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Fletching
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let fletching = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Fishing
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let fishing = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Firemaking
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let firemaking = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Crafting
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let crafting = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Smithing
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let smithing = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Mining
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let mining = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Herblore
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let herblore = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Agility
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let agility = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Thieving
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let thieving = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Slayer
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let slayer = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Farming
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let farming = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Runecrafting
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let runecrafting = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Hunter
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let hunter = parts.next().unwrap_or("").parse::<isize>().unwrap();
    //Construction
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let level = parts.next().unwrap_or("");
    let construction = parts.next().unwrap_or("").parse::<isize>().unwrap();

    //League Points
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let league_points = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let league_points = if league_points < 0 {0} else {league_points};

    //Deadman Points
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let deadman_points = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let deadman_points = if deadman_points < 0 {0} else {deadman_points};

    //Bounty Hunter - Hunter
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let bounty_hunter_hunter = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let bounty_hunter_hunter = if bounty_hunter_hunter < 0 {0} else {bounty_hunter_hunter};

    //Bounty Hunter - Rogue
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let bounty_hunter_rogue = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let bounty_hunter_rogue = if bounty_hunter_rogue < 0 {0} else {bounty_hunter_rogue};

    //Bounty Hunter (Legacy) - Hunter
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let bounty_hunter_legacy_hunter = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let bounty_hunter_legacy_hunter = if bounty_hunter_legacy_hunter < 0 {0} else {bounty_hunter_legacy_hunter};

    //Bounty Hunter (Legacy) - Rogue
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let bounty_hunter_legacy_rogue = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let bounty_hunter_legacy_rogue = if bounty_hunter_legacy_rogue < 0 {0} else {bounty_hunter_legacy_rogue};

    //Clue Scrolls (all)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_all = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_all = if clue_scrolls_all < 0 {0} else {clue_scrolls_all};

    //Clue Scrolls (beginner)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_beginner = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_beginner = if clue_scrolls_beginner < 0 {0} else {clue_scrolls_beginner};

    //Clue Scrolls (easy)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_easy = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_easy = if clue_scrolls_easy < 0 {0} else {clue_scrolls_easy};

    //Clue Scrolls (medium)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_medium = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_medium = if clue_scrolls_medium < 0 {0} else {clue_scrolls_medium};

    //Clue Scrolls (hard)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_hard = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_hard = if clue_scrolls_hard < 0 {0} else {clue_scrolls_hard};

    //Clue Scrolls (elite)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_elite = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_elite = if clue_scrolls_elite < 0 {0} else {clue_scrolls_elite};

    //Clue Scrolls (master)
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let clue_scrolls_master = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let clue_scrolls_master = if clue_scrolls_master < 0 {0} else {clue_scrolls_master};

    //LMS - Rank
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let lms_ranked = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let lms_ranked = if lms_ranked < 0 {0} else {lms_ranked};

    //PvP Arena - Rank
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let pvp_arena = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let pvp_arena = if pvp_arena < 0 {0} else {pvp_arena};

    //Soul Wars Zeal
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let soul_wars_zeal = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let soul_wars_zeal = if soul_wars_zeal < 0 {0} else {soul_wars_zeal};

    //Rifts closed
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let gotr_rifts = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let gotr_rifts = if gotr_rifts < 0 {0} else {gotr_rifts};


    //Abyssal Sire
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let abyssal_sire = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let abyssal_sire = if abyssal_sire < 0 {0} else {abyssal_sire};

    //Alchemical Hydra
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let alchemical_hydra = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let alchemical_hydra = if alchemical_hydra < 0 {0} else {alchemical_hydra};
    
    //Artio
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let artio = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let artio = if artio < 0 {0} else {artio};

    //Barrows Chests
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let barrows = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let barrows = if barrows < 0 {0} else {barrows};

    //Bryophyta
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let bryophyta = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let bryophyta = if bryophyta < 0 {0} else {bryophyta};

    //Callisto
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let callisto = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let callisto = if callisto < 0 {0} else {callisto};

    //Cal'varion
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let calvarion = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let calvarion = if calvarion < 0 {0} else {calvarion};

    //Cerberus
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let cerberus = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let cerberus = if cerberus < 0 {0} else {cerberus};

    //Chambers of Xeric
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let cox = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let cox = if cox < 0 {0} else {cox};

    //Chambers of Xeric: Challenge Mode
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let cox_cm = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let cox_cm = if cox_cm < 0 {0} else {cox_cm};

    //Chaos Elemental
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let chaos_elemental = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let chaos_elemental = if chaos_elemental < 0 {0} else {chaos_elemental};

    //Chaos Fanatic
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let chaos_fanatic = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let chaos_fanatic = if chaos_fanatic < 0 {0} else {chaos_fanatic};

    //Commander Zilyana
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let commander_zilyana = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let commander_zilyana = if commander_zilyana < 0 {0} else {commander_zilyana};

    //Corporeal Beast
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let corporeal_beast = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let corporeal_beast = if corporeal_beast < 0 {0} else {corporeal_beast};

    //Crazy Archaeologist
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let crazy_archeologist = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let crazy_archeologist = if crazy_archeologist < 0 {0} else {crazy_archeologist};

    //Dagannoth Prime
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let dagannoth_prime = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let dagannoth_prime = if dagannoth_prime < 0 {0} else {dagannoth_prime};

    //Dagannoth Rex
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let dagannoth_rex = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let dagannoth_rex = if dagannoth_rex < 0 {0} else {dagannoth_rex};

    //Dagannoth Supreme
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let dagannoth_supreme = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let dagannoth_supreme = if dagannoth_supreme < 0 {0} else {dagannoth_supreme};

    //Deranged Archaeologist
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let deranged_archaeologist = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let deranged_archaeologist = if deranged_archaeologist < 0 {0} else {deranged_archaeologist};

    //Duke Sucellus
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let duke_sucellus = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let duke_sucellus = if duke_sucellus < 0 {0} else {duke_sucellus};

    //General Graardor
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let general_graardor = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let general_graardor = if general_graardor < 0 {0} else {general_graardor};

    //Giant Mole
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let giant_mole = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let giant_mole = if giant_mole < 0 {0} else {giant_mole};

    //Grotesque Guardians
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let grotesque_guardians = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let grotesque_guardians = if grotesque_guardians < 0 {0} else {grotesque_guardians};

    //Hespori
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let hespori = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let hespori = if hespori < 0 {0} else {hespori};

    //Kalphite Queen
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let kalphite_queen = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let kalphite_queen = if kalphite_queen < 0 {0} else {kalphite_queen};

    //King Black Dragon
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let king_black_dragon = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let king_black_dragon = if king_black_dragon < 0 {0} else {king_black_dragon};

    //Kraken
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let kraken = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let kraken = if kraken < 0 {0} else {kraken};

    //Kree'Arra
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let kreearra = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let kreearra = if kreearra < 0 {0} else {kreearra};

    //K'ril Tsutsaroth
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let kril_tsutsaroth = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let kril_tsutsaroth = if kril_tsutsaroth < 0 {0} else {kril_tsutsaroth};

    //Mimic
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let mimic = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let mimic = if mimic < 0 {0} else {mimic};

    //Nex
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let nex = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let nex = if nex < 0 {0} else {nex};

    //Nightmare
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let nightmare = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let nightmare = if nightmare < 0 {0} else {nightmare};

    //Phosani's Nightmare
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let phosani_nightmare = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let phosani_nightmare = if phosani_nightmare < 0 {0} else {phosani_nightmare};

    //Obor
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let obor = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let obor = if obor < 0 {0} else {obor};

    //Phantom Muspah
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let phantom_muspah = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let phantom_muspah = if phantom_muspah < 0 {0} else {phantom_muspah};

    //Sarachnis
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let sarachnis = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let sarachnis = if sarachnis < 0 {0} else {sarachnis};

    //Scorpia
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let scorpia = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let scorpia = if scorpia < 0 {0} else {scorpia};

    //Skotizo
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let skotizo = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let skotizo = if skotizo < 0 {0} else {skotizo};

    //Spindel
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let spindel = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let spindel = if spindel < 0 {0} else {spindel};

    //Tempoross
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let tempoross = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let tempoross = if tempoross < 0 {0} else {tempoross};

    //The Gauntlet
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let gauntlet = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let gauntlet = if gauntlet < 0 {0} else {gauntlet};

    //The Corrupted Gauntlet
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let corrupted_gauntlet = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let corrupted_gauntlet = if corrupted_gauntlet < 0 {0} else {corrupted_gauntlet};

    //The Leviathan
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let leviathan = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let leviathan = if leviathan < 0 {0} else {leviathan};

    //The Whisperer
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let whisperer = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let whisperer = if whisperer < 0 {0} else {whisperer};

    //Theatre of Blood
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let tob = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let tob = if tob < 0 {0} else {tob};

    //Theatre of Blood: Hard Mode
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let tob_hard_mode = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let tob_hard_mode = if tob_hard_mode < 0 {0} else {tob_hard_mode};

    //Thermonuclear Smoke Devil
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let thermy = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let thermy = if thermy < 0 {0} else {thermy};

    //Tombs of Amascut
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let toa = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let toa = if toa < 0 {0} else {toa};

    //Tombs of Amascut: Expert Mode
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let toa_expert_mode = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let toa_expert_mode = if toa_expert_mode < 0 {0} else {toa_expert_mode};

    //TzKal-Zuk
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let tzkal_zuk = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let tzkal_zuk = if tzkal_zuk < 0 {0} else {tzkal_zuk};

    //TzTok-Jad
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let tztok_jad = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let tztok_jad = if tztok_jad < 0 {0} else {tztok_jad};

    //Vardorvis
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let vardorvis = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let vardorvis = if vardorvis < 0 {0} else {vardorvis};

    //Venenatis
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let venenatis = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let venenatis = if venenatis < 0 {0} else {venenatis};

    //Vet'ion
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let vetion = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let vetion = if vetion < 0 {0} else {vetion};

    //Vorkath
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let vorkath = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let vorkath = if vorkath < 0 {0} else {vorkath};

    //Wintertodt
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let wintertodt = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let wintertodt = if wintertodt < 0 {0} else {wintertodt};

    //Zalcano
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let zalcano = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let zalcano = if zalcano < 0 {0} else {zalcano};

    //Zulrah
    let line = lines.next().unwrap();

    let mut parts = line.split(',');
    let rank = parts.next().unwrap_or("");
    let zulrah = parts.next().unwrap_or("").parse::<isize>().unwrap();
    let zulrah = if zulrah < 0 {0} else {zulrah};




    let parsed_data = ParsedHiScoresData{
        overall: overall,
        attack: attack,
        defence: defence,
        strength: strength,
        hitpoints: hitpoints,
        ranged: ranged,
        prayer: prayer,
        magic: magic,
        cooking: cooking,
        woodcutting: woodcutting,
        fletching: fletching,
        fishing: fishing,
        firemaking: firemaking,
        crafting: crafting,
        smithing: smithing,
        mining: mining,
        herblore: herblore,
        agility: agility,
        thieving: thieving,
        slayer: slayer,
        farming: farming,
        runecrafting: runecrafting,
        hunter: hunter,
        construction: construction,
        //Activities
        league_points: league_points,
        deadman_points: deadman_points,
        bounty_hunter_hunter: bounty_hunter_hunter,
        bounty_hunter_rogue: bounty_hunter_rogue,
        bounty_hunter_legacy_hunter: bounty_hunter_legacy_hunter,
        bounty_hunter_legacy_rogue: bounty_hunter_legacy_rogue,
        clue_scrolls_all: clue_scrolls_all,
        clue_scrolls_beginner: clue_scrolls_beginner,
        clue_scrolls_easy: clue_scrolls_easy,
        clue_scrolls_medium: clue_scrolls_medium,
        clue_scrolls_hard: clue_scrolls_hard,
        clue_scrolls_elite: clue_scrolls_elite,
        clue_scrolls_master: clue_scrolls_master,
        lms_ranked: lms_ranked,
        pvp_arena: pvp_arena,
        soul_wars_zeal: soul_wars_zeal,
        gotr_rifts: gotr_rifts,
        //PVM
        abyssal_sire: abyssal_sire,
        alchemical_hydra: alchemical_hydra,
        artio: artio,
        barrows: barrows,
        bryophyta: bryophyta,
        callisto: callisto,
        calvarion: calvarion,
        cerberus: cerberus,
        cox: cox,
        cox_cm: cox_cm,
        chaos_elemental: chaos_elemental,
        chaos_fanatic: chaos_fanatic,
        commander_zilyana:commander_zilyana,
        corporeal_beast: corporeal_beast,
        crazy_archeologist: crazy_archeologist,
        dagannoth_prime: dagannoth_prime,
        dagannoth_rex: dagannoth_rex,
        dagannoth_supreme: dagannoth_supreme,
        deranged_archaeologist: deranged_archaeologist,
        duke_sucellus: duke_sucellus,
        general_graardor: general_graardor,
        giant_mole: giant_mole,
        grotesque_guardians: grotesque_guardians,
        hespori: hespori,
        kalphite_queen: kalphite_queen,
        king_black_dragon: king_black_dragon,
        kraken: kraken,
        kreearra: kreearra,
        kril_tsutsaroth: kril_tsutsaroth,
        mimic: mimic,
        nex: nex,
        nightmare: nightmare,
        phosani_nightmare: phosani_nightmare,
        obor: obor,
        phantom_muspah: phantom_muspah,
        sarachnis: sarachnis,
        scorpia: scorpia,
        skotizo: skotizo,
        spindel: spindel,
        tempoross: tempoross,
        gauntlet: gauntlet,
        corrupted_gauntlet: corrupted_gauntlet,
        leviathan: leviathan,
        whisperer: whisperer,
        tob: tob,
        tob_hard_mode: tob_hard_mode,
        thermy: thermy,
        toa: toa,
        toa_expert_mode: toa_expert_mode,
        tzkal_zuk: tzkal_zuk,
        tztok_jad: tztok_jad,
        vardorvis: vardorvis,
        venenatis: venenatis,
        vetion: vetion,
        vorkath: vorkath,
        wintertodt: wintertodt,
        zalcano: zalcano,
        zulrah: zulrah,
    };
    Ok(parsed_data)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut username: &str;

    if args.len() > 1 {
        username = &args[1];
    } else {
        username = "Letharg";
    }

    let hiscores = get_hiscores(username).await.unwrap();
    
    let parsed_data = parse_hiscores(hiscores).unwrap();
    let points_from_skills = get_points_from_skills(&parsed_data);
    let points_from_activities = get_points_from_activities(&parsed_data);
    let points_from_pvm = get_points_from_pvm(&parsed_data);
    let total_points = points_from_skills.overall + points_from_activities.total + points_from_pvm.total;
    
    println!("=== {} ===", username);
    println!("total points: {}, from skills: {}, from activities: {}, from pvm: {}", total_points, points_from_skills.overall, points_from_activities.total, points_from_pvm.total);
    println!("Detailed: ");
    println!("=== Skills: {:?}", points_from_skills);
    println!("=== Activities: {:?} ", points_from_activities);
    println!("=== PVM: {:?}", points_from_pvm);
    Ok(())
} 