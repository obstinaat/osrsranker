use reqwest;
use reqwest::Error;
use tokio;
use std::fs::File;
use std::io::Read;
use serde_json;
use serde::Deserialize;
use colored::Colorize;
use std::iter;

const URL_BASE: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";

//Weights are subject to change; preferably configurable for each different entry, with custom milestones and custom point counts
//additionally it would be nice if milestones after level 99 were implemented, such as 25m xp. 
//by current weights a maxed player will have 3950 points from skills.

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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppConfig {
    //
    // SKILLS
    //
    attack: Vec<(isize, isize)>,
    defence: Vec<(isize, isize)>,
    strength: Vec<(isize, isize)>,
    hitpoints: Vec<(isize, isize)>,
    ranged: Vec<(isize, isize)>,
    prayer: Vec<(isize, isize)>,
    magic: Vec<(isize, isize)>,
    cooking: Vec<(isize, isize)>,
    woodcutting: Vec<(isize, isize)>,
    fletching: Vec<(isize, isize)>,
    fishing: Vec<(isize, isize)>,
    firemaking: Vec<(isize, isize)>,
    crafting: Vec<(isize, isize)>,
    smithing: Vec<(isize, isize)>,
    mining: Vec<(isize, isize)>,
    herblore: Vec<(isize, isize)>,
    agility: Vec<(isize, isize)>,
    thieving: Vec<(isize, isize)>,
    slayer: Vec<(isize, isize)>,
    farming: Vec<(isize, isize)>,
    runecrafting: Vec<(isize, isize)>,
    hunter: Vec<(isize, isize)>,
    construction: Vec<(isize, isize)>,
    //
    // ACTIVITIES
    //
    league_points: Vec<(isize, isize)>,
    deadman_points: Vec<(isize, isize)>,
    bounty_hunter_hunter: Vec<(isize, isize)>,
    bounty_hunter_rogue: Vec<(isize, isize)>,
    bounty_hunter_legacy_hunter: Vec<(isize, isize)>,
    bounty_hunter_legacy_rogue: Vec<(isize, isize)>,
    clue_scrolls_all: Vec<(isize, isize)>,
    clue_scrolls_beginner: Vec<(isize, isize)>,
    clue_scrolls_easy: Vec<(isize, isize)>,
    clue_scrolls_medium: Vec<(isize, isize)>,
    clue_scrolls_hard: Vec<(isize, isize)>,
    clue_scrolls_elite: Vec<(isize, isize)>,
    clue_scrolls_master: Vec<(isize, isize)>,
    lms_ranked: Vec<(isize, isize)>,
    pvp_arena: Vec<(isize, isize)>,
    soul_wars_zeal: Vec<(isize, isize)>,
    gotr_rifts: Vec<(isize, isize)>,
    //
    //PVM
    //
    abyssal_sire: Vec<(isize, isize)>,
    alchemical_hydra: Vec<(isize, isize)>,
    artio: Vec<(isize, isize)>,
    barrows: Vec<(isize, isize)>,
    bryophyta: Vec<(isize, isize)>,
    callisto: Vec<(isize, isize)>,
    calvarion: Vec<(isize, isize)>,
    cerberus: Vec<(isize, isize)>,
    cox: Vec<(isize, isize)>,
    cox_cm: Vec<(isize, isize)>,
    chaos_elemental: Vec<(isize, isize)>,
    chaos_fanatic: Vec<(isize, isize)>,
    commander_zilyana:Vec<(isize, isize)>,
    corporeal_beast: Vec<(isize, isize)>,
    crazy_archeologist: Vec<(isize, isize)>,
    dagannoth_prime: Vec<(isize, isize)>,
    dagannoth_rex: Vec<(isize, isize)>,
    dagannoth_supreme: Vec<(isize, isize)>,
    deranged_archaeologist: Vec<(isize, isize)>,
    duke_sucellus: Vec<(isize, isize)>,
    general_graardor: Vec<(isize, isize)>,
    giant_mole: Vec<(isize, isize)>,
    grotesque_guardians: Vec<(isize, isize)>,
    hespori: Vec<(isize, isize)>,
    kalphite_queen: Vec<(isize, isize)>,
    king_black_dragon: Vec<(isize, isize)>,
    kraken: Vec<(isize, isize)>,
    kreearra: Vec<(isize, isize)>,
    kril_tsutsaroth: Vec<(isize, isize)>,
    mimic: Vec<(isize, isize)>,
    nex: Vec<(isize, isize)>,
    nightmare: Vec<(isize, isize)>,
    phosani_nightmare: Vec<(isize, isize)>,
    obor: Vec<(isize, isize)>,
    phantom_muspah: Vec<(isize, isize)>,
    sarachnis: Vec<(isize, isize)>,
    scorpia: Vec<(isize, isize)>,
    skotizo: Vec<(isize, isize)>,
    spindel: Vec<(isize, isize)>,
    tempoross: Vec<(isize, isize)>,
    gauntlet: Vec<(isize, isize)>,
    corrupted_gauntlet: Vec<(isize, isize)>,
    leviathan: Vec<(isize, isize)>,
    whisperer: Vec<(isize, isize)>,
    tob: Vec<(isize, isize)>,
    tob_hard_mode: Vec<(isize, isize)>,
    thermy: Vec<(isize, isize)>,
    toa: Vec<(isize, isize)>,
    toa_expert_mode: Vec<(isize, isize)>,
    tzkal_zuk: Vec<(isize, isize)>,
    tztok_jad: Vec<(isize, isize)>,
    vardorvis: Vec<(isize, isize)>,
    venenatis: Vec<(isize, isize)>,
    vetion: Vec<(isize, isize)>,
    vorkath: Vec<(isize, isize)>,
    wintertodt: Vec<(isize, isize)>,
    zalcano: Vec<(isize, isize)>,
    zulrah: Vec<(isize, isize)>,
}

async fn get_hiscores(username: &str) -> Result<String, Error> {
    let url = String::from(URL_BASE) + username;

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Ok(body)
}

fn calc_points(score: isize, milestones: &Vec<(isize, isize)>) -> isize {
    let mut points = 0;
    for (requirement, reward) in milestones{
        if score < *requirement{
            return points;
        }
        points += reward;
    }
    points
}

fn get_points_from_skills(playerdata: &ParsedHiScoresData, config: &AppConfig) -> PointsFromSkills{
    //EASY SKILLS
    let attack = calc_points(playerdata.attack, &config.attack);
    let defence = calc_points(playerdata.defence, &config.defence);
    let strength = calc_points(playerdata.strength, &config.strength);
    let hitpoints = calc_points(playerdata.hitpoints, &config.hitpoints);
    let ranged = calc_points(playerdata.ranged, &config.ranged);
    let prayer = calc_points(playerdata.prayer, &config.prayer);
    let magic = calc_points(playerdata.magic, &config.magic);
    let cooking = calc_points(playerdata.cooking, &config.cooking);
    let woodcutting = calc_points(playerdata.woodcutting, &config.woodcutting);
    let fletching = calc_points(playerdata.fletching, &config.fletching);
    let fishing = calc_points(playerdata.fishing, &config.fishing);
    let firemaking = calc_points(playerdata.firemaking, &config.firemaking);
    let crafting = calc_points(playerdata.crafting, &config.crafting);
    let smithing = calc_points(playerdata.smithing, &config.smithing);
    let mining = calc_points(playerdata.mining, &config.mining);
    let herblore = calc_points(playerdata.herblore, &config.herblore);
    let agility = calc_points(playerdata.agility, &config.agility);
    let thieving = calc_points(playerdata.thieving, &config.thieving);
    let slayer = calc_points(playerdata.slayer, &config.slayer);
    let farming = calc_points(playerdata.farming, &config.farming);
    let runecrafting = calc_points(playerdata.runecrafting, &config.runecrafting);
    let hunter = calc_points(playerdata.hunter, &config.hunter);
    let construction = calc_points(playerdata.construction, &config.construction);
    
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

fn get_points_from_activities(playerdata: &ParsedHiScoresData, config: &AppConfig) -> PointsFromActivities{
    let league_points = calc_points(playerdata.league_points, &config.league_points);
    let deadman_points = calc_points(playerdata.deadman_points, &config.deadman_points);
    let bounty_hunter_hunter = calc_points(playerdata.bounty_hunter_hunter, &config.bounty_hunter_hunter);
    let bounty_hunter_rogue = calc_points(playerdata.bounty_hunter_rogue, &config.bounty_hunter_rogue);
    let bounty_hunter_legacy_hunter = calc_points(playerdata.bounty_hunter_legacy_hunter, &config.bounty_hunter_legacy_hunter);
    let bounty_hunter_legacy_rogue = calc_points(playerdata.bounty_hunter_legacy_rogue, &config.bounty_hunter_legacy_rogue);
    let clue_scrolls_all = calc_points(playerdata.clue_scrolls_all, &config.clue_scrolls_all);
    let clue_scrolls_beginner = calc_points(playerdata.clue_scrolls_beginner, &config.clue_scrolls_beginner);
    let clue_scrolls_easy = calc_points(playerdata.clue_scrolls_easy, &config.clue_scrolls_easy);
    let clue_scrolls_medium = calc_points(playerdata.clue_scrolls_medium, &config.clue_scrolls_medium);
    let clue_scrolls_hard = calc_points(playerdata.clue_scrolls_hard, &config.clue_scrolls_hard);
    let clue_scrolls_elite = calc_points(playerdata.clue_scrolls_elite, &config.clue_scrolls_elite);
    let clue_scrolls_master = calc_points(playerdata.clue_scrolls_master, &config.clue_scrolls_master);
    let lms_ranked = calc_points(playerdata.lms_ranked, &config.lms_ranked);
    let pvp_arena = calc_points(playerdata.pvp_arena, &config.pvp_arena);
    let soul_wars_zeal = calc_points(playerdata.soul_wars_zeal, &config.soul_wars_zeal);
    let gotr_rifts = calc_points(playerdata.gotr_rifts, &config.gotr_rifts);

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

fn get_points_from_pvm(playerdata: &ParsedHiScoresData, config: &AppConfig) -> PointsFromPVM{
    let abyssal_sire = calc_points(playerdata.abyssal_sire, &config.abyssal_sire);
    let alchemical_hydra = calc_points(playerdata.alchemical_hydra, &config.alchemical_hydra);
    let artio = calc_points(playerdata.artio, &config.artio);
    let barrows = calc_points(playerdata.barrows, &config.barrows);
    let bryophyta = calc_points(playerdata.bryophyta, &config.bryophyta);
    let callisto = calc_points(playerdata.callisto, &config.callisto);
    let calvarion = calc_points(playerdata.calvarion, &config.calvarion);
    let cerberus = calc_points(playerdata.cerberus, &config.cerberus);
    let cox = calc_points(playerdata.cox, &config.cox);
    let cox_cm = calc_points(playerdata.cox_cm, &config.cox_cm);
    let chaos_elemental = calc_points(playerdata.chaos_elemental, &config.chaos_elemental);
    let chaos_fanatic = calc_points(playerdata.chaos_fanatic, &config.chaos_fanatic);
    let commander_zilyana = calc_points(playerdata.commander_zilyana, &config.commander_zilyana);
    let corporeal_beast = calc_points(playerdata.corporeal_beast, &config.corporeal_beast);
    let crazy_archeologist = calc_points(playerdata.crazy_archeologist, &config.crazy_archeologist);
    let dagannoth_prime = calc_points(playerdata.dagannoth_prime, &config.dagannoth_prime);
    let dagannoth_rex = calc_points(playerdata.dagannoth_rex, &config.dagannoth_rex);
    let dagannoth_supreme = calc_points(playerdata.dagannoth_supreme, &config.dagannoth_supreme);
    let deranged_archaeologist = calc_points(playerdata.deranged_archaeologist, &config.deranged_archaeologist);
    let duke_sucellus = calc_points(playerdata.duke_sucellus, &config.duke_sucellus);
    let general_graardor = calc_points(playerdata.general_graardor, &config.general_graardor);
    let giant_mole = calc_points(playerdata.giant_mole, &config.giant_mole);
    let grotesque_guardians = calc_points(playerdata.grotesque_guardians, &config.grotesque_guardians);
    let hespori = calc_points(playerdata.hespori, &config.hespori);
    let kalphite_queen = calc_points(playerdata.kalphite_queen, &config.kalphite_queen);
    let king_black_dragon = calc_points(playerdata.king_black_dragon, &config.king_black_dragon);
    let kraken = calc_points(playerdata.kraken, &config.kraken);
    let kreearra = calc_points(playerdata.kreearra, &config.kreearra);
    let kril_tsutsaroth = calc_points(playerdata.kril_tsutsaroth, &config.kril_tsutsaroth);
    let mimic = calc_points(playerdata.mimic, &config.mimic);
    let nex = calc_points(playerdata.nex, &config.nex);
    let nightmare = calc_points(playerdata.nightmare, &config.nightmare);
    let phosani_nightmare = calc_points(playerdata.phosani_nightmare, &config.phosani_nightmare);
    let obor = calc_points(playerdata.obor, &config.obor);
    let phantom_muspah = calc_points(playerdata.phantom_muspah, &config.phantom_muspah);
    let sarachnis = calc_points(playerdata.sarachnis, &config.sarachnis);
    let scorpia = calc_points(playerdata.scorpia, &config.scorpia);
    let skotizo = calc_points(playerdata.skotizo, &config.skotizo);
    let spindel = calc_points(playerdata.spindel, &config.spindel);
    let tempoross = calc_points(playerdata.tempoross, &config.tempoross);
    let gauntlet = calc_points(playerdata.gauntlet, &config.gauntlet);
    let corrupted_gauntlet = calc_points(playerdata.corrupted_gauntlet, &config.corrupted_gauntlet);
    let leviathan = calc_points(playerdata.leviathan, &config.leviathan);
    let whisperer = calc_points(playerdata.whisperer, &config.whisperer);
    let tob = calc_points(playerdata.tob, &config.tob);
    let tob_hard_mode = calc_points(playerdata.tob_hard_mode, &config.tob_hard_mode);
    let thermy = calc_points(playerdata.thermy, &config.thermy);
    let toa = calc_points(playerdata.toa, &config.toa);
    let toa_expert_mode = calc_points(playerdata.toa_expert_mode, &config.toa_expert_mode);
    let tzkal_zuk = calc_points(playerdata.tzkal_zuk, &config.tzkal_zuk);
    let tztok_jad = calc_points(playerdata.tztok_jad, &config.tztok_jad);
    let vardorvis = calc_points(playerdata.vardorvis, &config.vardorvis);
    let venenatis = calc_points(playerdata.venenatis, &config.venenatis);
    let vetion = calc_points(playerdata.vetion, &config.vetion);
    let vorkath = calc_points(playerdata.vorkath, &config.vorkath);
    let wintertodt = calc_points(playerdata.wintertodt, &config.wintertodt);
    let zalcano = calc_points(playerdata.zalcano, &config.zalcano);
    let zulrah = calc_points(playerdata.zulrah, &config.zulrah);
    let total = abyssal_sire + alchemical_hydra + artio + barrows + bryophyta + callisto + calvarion +
     cerberus + cox + cox_cm + chaos_elemental + chaos_fanatic + commander_zilyana + corporeal_beast +
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

fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let file_path = "config/config.json";

    let mut file = File::open(file_path)?;
    let mut config_json = String::new();
    file.read_to_string(&mut config_json)?;

    // Deserialize the JSON into your configuration struct
    let config: AppConfig = serde_json::from_str(&config_json)?;

    Ok(config)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Open and read the configuration file
    let config = read_config().unwrap();


    //set usernames
    //let usernames = ["letharg", "bobballistic", "maccaroni", "I_M_Maarten", "preau", "Metalrule280", "Avusten", "Kepp"];
    let usernames = ["Maccaroni"];

    //get score for each username
    for username in usernames{
        //retrieve hiscore information from osrs API
        let hiscores = get_hiscores(username).await.unwrap();
        
        //parse the hiscore information
        let parsed_data = parse_hiscores(hiscores).unwrap();

        //calculate the points from this parsed data.
        let points_from_skills = get_points_from_skills(&parsed_data, &config);
        let points_from_activities = get_points_from_activities(&parsed_data, &config);
        let points_from_pvm = get_points_from_pvm(&parsed_data, &config);
        let total_points = points_from_skills.overall + points_from_activities.total + points_from_pvm.total;
        
        let TOTAL_CHARS = 60;
        let chars_from_skills = points_from_skills.overall  / 50;
        let chars_from_activities = points_from_activities.total / 50;
        let chars_from_pvm = points_from_pvm.total / 50;


        //print results
        println!("=== {} === [{} {} {}]", username,  
            iter::repeat('=').take(chars_from_skills as usize).collect::<String>().green(),
            iter::repeat('=').take(chars_from_activities as usize).collect::<String>().yellow(),
            iter::repeat('=').take(chars_from_pvm as usize).collect::<String>().red());
        println!("total points: {}, from skills: {}, from activities: {}, from pvm: {}", total_points, points_from_skills.overall, points_from_activities.total, points_from_pvm.total);
        println!("Detailed: ");
        println!("=== Skills: {:?}", points_from_skills);
        println!("=== Activities: {:?} ", points_from_activities);
        println!("=== PVM: {:?}", points_from_pvm);
    }
    Ok(())
} 