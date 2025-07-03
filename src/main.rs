/* 
 * Html players with cache = 1116
 * n training = 892
 * Tot predictions = 224
 * correct = 174
 * Html players without cache = 1116
 * n training = 892
 * Tot predictions = 224
 * correct = 173
 *
 * N of decks = 640
 * Version with everything = 77.2324%
 * Version wiht cache = 77.67857%
 * Version without lands = 77.06422%
 * Version with lands = 80.73394%
 * Version with creatures = 77.08333%
 * Version with instants = 67.70833%
 * Version with sorceries = 64.58333%
 * Version with artifacts = 78.125%
 * Version with enchantments = 77.08333%
 */

const COOKIES: &str = "locale=en_US; tarteaucitron=!dgcMultiplegtagUa=wait; JSESSIONID=26504EDCACA304DCB74AB4E6D17B477C.lvs-foyert2-3409";
const HOST: &str = "www.mtgo.com";
const MAX_TRIES: usize = 5;
const FIRST_INDEX_STRING: &str = r#"window.MTGO.decklists.data = "#;
const STANDINGS_KEY: &str = "standings";
const RANK_KEY: &str = "rank";
const JSON_DECK_LISTS_KEY: &str = "decklists";
const COLOR_TAG: &str = "COLOR_";
const TRAINING_PERCENTAGE: f32 = 0.8;
const VERBOSE_LOG: bool = false;
const USE_CACHE: bool = true;
const CACHE_PATH: &str = "cache";
const PLAYER_SEPARATOR: &str = "=-=-=PLAYER SEPARATOR=-=-=";
const RANK_SEPARATOR: &str = "=-=-=RANK SEPARATOR=-=-=";

use std::{fs as file_system, io::Write, num::ParseIntError, thread, time::Duration};

use serde_json::Value;
use reqwest::blocking::{Client, Response};
use rand::{Rng, seq::SliceRandom};

macro_rules! test {
    () => {
        println!("This is a test!!!");
    };
}

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    gini_impurity: Option<f32>,
    feature_index: Option<usize>,
    on_true: Option<Box<Node>>,
    on_false: Option<Box<Node>>,
    prediction: Option<bool>,
}

fn print_formatted_log_string(target_string: String) {
    if !VERBOSE_LOG {
        return;
    }
    let mut indentation_level: usize = 0;
    let mut line: Vec<char> = Vec::new();
    for character in target_string.chars() {
        if ['}', ']', ')'].contains(&character) {
            indentation_level -= 1;
            line.append(&mut get_new_line_string(&indentation_level));
        }
        line.push(character);
        if character == ',' {
            line.append(&mut get_new_line_string(&indentation_level));
        }
        if ['{', '[', '('].contains(&character) {
            indentation_level += 1;
            line.append(&mut get_new_line_string(&indentation_level));
        }
    }
    println!("{}", String::from_iter(line));
}

fn get_new_line_string(indentation_level: &usize) -> Vec<char> {
    let mut new_line_chars: Vec<char> = Vec::from(['\n']);
    for _ in 0..*indentation_level {
        new_line_chars.push('\t');
    }
    return new_line_chars;
}

fn panic_on_try_value_exceding_max_tries(try_value: usize, error_message: String) {
    if try_value > MAX_TRIES {
        panic!("{}", error_message);
    }
    println!("{}", error_message);
}

fn correctness_score(
    number_of_correct: usize,
    number_of_total: usize
) -> f32 {
    if number_of_total == 0 {
        return 1.0;
    }
    return f32::powi(number_of_correct as f32 / number_of_total as f32, 2);
}

fn evaluate_data(tree: &Node, data: Vec<bool>) -> bool {
    if let Some(value) = tree.prediction {
        return value;
    }
    let mut data_result: bool = false;
    if let Some(feature_index) = tree.feature_index {
        data_result = data[feature_index];
    }
    if data_result {
        if let Some(node) = &tree.on_true {
            return evaluate_data(&node, data);
        }
    } else {
        if let Some(node) = &tree.on_false {
            return evaluate_data(&node, data);
        }
    }
    return false;
}

fn leaf_node(
    true_amounts: usize,
    false_amounts: usize,
    gini_impurity: f32
) -> Node {
    return Node {
        gini_impurity: Some(gini_impurity),
        feature_index: None,
        on_true: None,
        on_false: None,
        prediction: Some(true_amounts >= false_amounts),
    };
}

fn get_node_gini_impurity(target: &Vec<bool>, indexes: &Vec<usize>) -> (f32, usize, usize) {
    let mut true_amounts: usize = 0;
    let mut false_amounts: usize = 0;
    for index in indexes {
        if target[*index] {
            true_amounts += 1;
        } else {
            false_amounts += 1;
        }
    }
    let total_number_in_branch: usize = true_amounts + false_amounts;
    let left_correctness_score: f32 = correctness_score(true_amounts, total_number_in_branch);
    let right_corerctness_score: f32 = correctness_score(false_amounts, total_number_in_branch);
    let node_gini_impurity: f32 = 1.0 - left_correctness_score - right_corerctness_score;
    return (node_gini_impurity, true_amounts, false_amounts);
}

fn generate_nodes(
    indexes: &Vec<usize>,
    data: &Vec<Vec<bool>>,
    target: &Vec<bool>,
    cut: usize,
    number_of_feature_to_consider: usize,
    mut chosen_features: Vec<usize>,
) -> Node {
    let (node_gini_impurity, number_of_true, number_of_false) = get_node_gini_impurity(target, indexes);
    if node_gini_impurity == 0.0 || number_of_true + number_of_false <= cut {
        return leaf_node(number_of_true, number_of_false, node_gini_impurity);
    }
    let mut smaller_gini: f32 = 1.0;
    let mut smaller_feature_index: usize = usize::MAX;
    let mut on_smaller_true_indexes: Vec<usize> = Vec::new();
    let mut on_smaller_false_indexes: Vec<usize> = Vec::new();
    let mut chosen_feature_indexes: Vec<usize> = Vec::new();
    let mut feature_indexes: Vec<usize> = Vec::from_iter(0..data.len());
    let mut features_indexes_to_remove: Vec<usize> = Vec::new();
    for chosen_feature in &chosen_features {
        for (index, item) in feature_indexes.iter().enumerate() {
            if item == chosen_feature {
                features_indexes_to_remove.push(index);
            }
        }
    }
    features_indexes_to_remove.sort();
    for index in 0..features_indexes_to_remove.len() {
        feature_indexes.remove(features_indexes_to_remove[index] - index);
    }
    let mut rng_thread: rand::rngs::ThreadRng = rand::thread_rng();
    feature_indexes.shuffle(&mut rng_thread);
    for index in 0..number_of_feature_to_consider {
        chosen_feature_indexes.push(feature_indexes[index]);
    }
    for feature_index in &chosen_feature_indexes {
        let mut true_indexes: Vec<usize> = Vec::new();
        let mut false_indexes: Vec<usize> = Vec::new();
        for data_index in indexes {
            if data[*feature_index][*data_index] {
                true_indexes.push(*data_index);
            } else {
                false_indexes.push(*data_index);
            }
        }
        let (true_gini_impurity, _, _) = get_node_gini_impurity(target, &true_indexes);
        let (false_gini_impurity, _, _) = get_node_gini_impurity(target, &false_indexes);
        let total_gini_impurity: f32 = 
            (
                true_gini_impurity * true_indexes.len() as f32 +
                false_gini_impurity * false_indexes.len() as f32
            ) / (true_indexes.len() as f32 + false_indexes.len() as f32);
        if total_gini_impurity < smaller_gini {
            smaller_gini = total_gini_impurity;
            smaller_feature_index = *feature_index;
            on_smaller_true_indexes = true_indexes;
            on_smaller_false_indexes = false_indexes;
        }
    }
    if f32::abs(smaller_gini - node_gini_impurity) <= 0.000001 {
        return leaf_node(number_of_true, number_of_false, node_gini_impurity);
    }
    chosen_features.push(smaller_feature_index);
    let on_true_node: Node = generate_nodes(
        &on_smaller_true_indexes,
        data,
        target,
        cut,
        number_of_feature_to_consider,
        chosen_features.clone(),
    );
    let on_false_node: Node = generate_nodes(
        &on_smaller_false_indexes,
        data,
        target,
        cut,
        number_of_feature_to_consider,
        chosen_features.clone(),
    );
    return Node {
        gini_impurity: Some(smaller_gini),
        feature_index: Some(smaller_feature_index),
        on_true: Some(Box::new(on_true_node)),
        on_false: Some(Box::new(on_false_node)),
        prediction: None,
    };
}


fn main() {
    let mut player_to_evaluate_string: String;
    match file_system::read_to_string("deck_template") {
        Ok(value) => player_to_evaluate_string = value,
        Err(error) => panic!("Unable to find or open deck_template file: {:?}", error),
    }
    let mut player_to_evaluate: Value;
    match serde_json::from_str::<Value>(&player_to_evaluate_string) {
        Ok(value) => player_to_evaluate = value,
        Err(error) => panic!("Unable to parse string to value for player_to_evaluate: {:?}", error),
    }
    let mut players: Vec<Value> = Vec::new();
    let mut decks_rank: Vec<u64> = Vec::new();
    if USE_CACHE {
        match file_system::exists(CACHE_PATH) {
            Ok(false) | Err(_) =>
                panic!("Cache file does not exists or the program has no permisions to ope it"),
            _ => ()
        }
        let cached_players: String;
        match file_system::read_to_string(CACHE_PATH) {
            Ok(file_content) => cached_players = file_content,
            Err(error) => panic!("Unable to read cache file: {:?}", error),
        }
        let mut cached_players: Vec<&str> = Vec::from_iter(cached_players.split(PLAYER_SEPARATOR));
        cached_players.pop();
        for cached_player in cached_players {
            let player_data: Vec<&str> = Vec::from_iter(cached_player.split(RANK_SEPARATOR));
            match serde_json::from_str::<Value>(player_data[0]) {
                Ok(player_value) => players.push(player_value),
                Err(error) => panic!("Unable to parse a player from cache: {:?}", error),
            }
            match player_data[1].parse::<u64>() {
                Ok(value) => decks_rank.push(value),
                Err(error) => panic!("Unable to parse &str to u64: {:?}", error),
            }
        }
    } else {
        if let Err(error) = file_system::remove_file(CACHE_PATH) {
            panic!("Unable to delete cache file: {:?}", error);
        }
        let mut cache_file: file_system::File;
        match file_system::File::create(CACHE_PATH) {
            Ok(value) => cache_file = value,
            Err(error) => panic!("Unable to create or open cache file: {}", error),
        }
        let mut urls: Vec<String> = Vec::new();
        let url_queries: Vec<&str> = Vec::from([
            "04-0412763152",
            "04-0512763169",
            "04-0612763187",
            "04-1112765765",
            "04-1212765782",
            "04-1812769888",
            "04-1912769905",
            "04-2012769922",
            "04-2512772646",
            "04-2612772667",
            "04-2712772689",
            "05-0212774478",
            "05-0312774499",
            "05-0412774521",
            "05-0912777329",
            "05-1012777346",
            "05-1112777364",
            "05-1612780132",
            "05-1712780149",
            "05-1812780167",
            "05-2312782299",
            "05-2412782313",
            "05-2512782331",
            "05-3012782641",
            "05-3112782655",
            "06-0112782673",
            "06-0612792678",
            "06-0712792692",
            "06-0812792709",
            "06-1312794581",
            "06-1412794595",
            "06-1512794613",
            "06-2012798157",
            "06-2112798171",
            "06-2212798189",
            "06-2712799984",
        ]);
        for url_query in &url_queries {
            urls.push(format!("https://{}/decklist/pauper-challenge-32-2025-{}", HOST, url_query));
        }
        for url_index in 0..url_queries.len() {
            let url: &str = &urls[url_index];
            println!("Requesting url: {:?}", url);
            let client: Client = Client::new();
            let mut raw_decks_json: String = String::new();
            for try_value in 1..=MAX_TRIES {
                println!("Try {:?}", try_value);
                thread::sleep(Duration::from_secs(7));
                let result_response: Result<Response, reqwest::Error> = client
                    .get(url)
                    .timeout(Duration::from_secs(60))
                    .header("Host", HOST)
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0")
                    .header("Accetp", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                    .header("Accept-Language", "en-Us,en;q=0.5")
                    .header("Accept-Encoding", "gzip, deflate, br, zstd")
                    .header("Referer", "https://www.mtgo.com/decklists?filter=Pauper")
                    .header("Sec-GPC", "1")
                    .header("Connection", "keep-alive")
                    .header("Cookie", COOKIES)
                    .header("Upgrade-Insecure-Requests", "1")
                    .header("Sec-Fetch-Dest", "document")
                    .header("Sec-Fetch-Mode", "navigate")
                    .header("Sec-Fetch-Site", "same-origin")
                    .header("Sec-Fetch-User", "?1")
                    .header("Priority", "u=0, i")
                    .send();
                let response: Response;
                match result_response {
                    Ok(value) => response = value,
                    Err(error) => {
                        panic_on_try_value_exceding_max_tries(
                            try_value,
                            format!("Unable to get the response: {:?}", error),
                        );
                        continue;
                    }
                }
                let page_html: String;
                match response.text() {
                    Ok(value) => page_html = value,
                    Err(error) => {
                        panic_on_try_value_exceding_max_tries(
                            try_value,
                            format!("Unable to find text of the response: {:?}", error),
                        );
                        continue;
                    }
                }
                let first_index: usize;
                match page_html.find(FIRST_INDEX_STRING) {
                    Some(value) => first_index = value,
                    None => {
                        panic_on_try_value_exceding_max_tries(
                            try_value,
                            format!("{:?}\nUnable to find json in html document", page_html),
                        );
                        continue;
                    }
                }
                let second_index: usize;
                match page_html.find(r#"window.MTGO.decklists.type = "#) {
                    Some(value) => second_index = value,
                    None => {
                        panic_on_try_value_exceding_max_tries(
                            try_value,
                            format!("{:?}\nUnable to find json in html document", page_html),
                        );
                        continue;
                    }
                }
                raw_decks_json =
                    page_html[first_index + FIRST_INDEX_STRING.len()..second_index - 6].to_string();
                break;
            }
            let json_data: Value;
            match serde_json::from_str::<Value>(&raw_decks_json) {
                Ok(parsed_data) => json_data = parsed_data,
                Err(error) => panic!("Unable to parse json: {:?}", error),
            };
            let optional_standings: Option<&Vec<Value>> = json_data[STANDINGS_KEY].as_array();
            print_formatted_log_string(format!("Standings as array: {:?}", optional_standings));
            let json_standings: Vec<&Value>;
            match optional_standings {
                Some(value) => json_standings = Vec::from_iter(value),
                None => panic!("Unable to read key {:?} from json", STANDINGS_KEY),
            }
            for json_standing in json_standings {
                let standing_value: &str;
                match &json_standing[RANK_KEY] {
                    Value::String(value) => standing_value = value,
                    _ => panic!("Unable to read key {:?} from json", RANK_KEY),
                }
                match standing_value.parse::<u64>() {
                    Ok(parsed_rank_value) => decks_rank.push(parsed_rank_value),
                    Err(error) => panic!("Failed to parse value: {:?}", error),
                }
            }
            let json_players: Vec<&Value>;
            match json_data[JSON_DECK_LISTS_KEY].as_array() {
                Some(value) => json_players = Vec::from_iter(value),
                None => panic!("Unable to read key {:?} from json", JSON_DECK_LISTS_KEY),
            }
            println!("Request recived with: {:?} players", json_players.len());
            if json_players.len() != 32 {
                panic!("N of players not 32");
            }
            for player_index in 1..json_players.len() {
                let cache_data: String = format!(
                    "{}{}{}{}",
                    json_players[player_index].to_string(),
                    RANK_SEPARATOR,
                    decks_rank[player_index],
                    PLAYER_SEPARATOR,
                );
                if let Err(error) = cache_file.write(cache_data.as_bytes()) {
                    panic!("Unable to write to cashe file: {}", error);
                }
                players.push(json_players[player_index].clone());
            }
        }
    }
    println!("Html players are: {:?}", players.len());
    players.push(player_to_evaluate);
    let mut data_matrix: Vec<Vec<bool>> = Vec::new();
    let colors_array: [String; 6] = [
        format!("{}{}", COLOR_TAG, "WHITE"),
        format!("{}{}", COLOR_TAG, "BLUE"),
        format!("{}{}", COLOR_TAG, "BLACK"),
        format!("{}{}", COLOR_TAG, "RED"),
        format!("{}{}", COLOR_TAG, "GREEN"),
        format!("{}{}", COLOR_TAG, "COLORLESS"),
    ];
    let mut color_data_matrix: Vec<Vec<bool>> = Vec::new();
    for _ in 0..colors_array.len() {
        color_data_matrix.push(Vec::new());
    }
    let cards_type_array: [&str; 6] = ["LAND  ", "ISCREA", "INSTNT", "SORCRY", "ARTFCT", "ENCHMT"];
    let mut card_type_matrix: Vec<Vec<u8>> = Vec::new();
    for _ in 0..cards_type_array.len() {
        card_type_matrix.push(Vec::new());
    }
    let mut card_average_cost_data: Vec<f32> = Vec::new();
    for player_index in 0..players.len() {
        let player_data_json: &Value;
        match players[player_index].get("main_deck") {
            Some(value) => player_data_json = value,
            None => panic!("Unable to find main_deck field in json"),
        }
        let deck: &Vec<Value>;
        let option_deck: Option<&Vec<Value>> = player_data_json.as_array();
        print_formatted_log_string(format!("Player's deck: {:?}", option_deck));
        match option_deck {
            Some(value) => deck = value,
            None => panic!("Unable to convert deck to Vec: {:?}", player_data_json),
        }
        let mut player_colors_data: Vec<bool> = Vec::new();
        for _ in 0..colors_array.len() {
            player_colors_data.push(false);
        }
        let mut player_card_type_count_data: Vec<u8> = Vec::new();
        for _ in 0..cards_type_array.len() {
            player_card_type_count_data.push(0);
        }
        let mut cards_cost: Vec<usize> = Vec::new();
        for card in deck {
            let card_attributes: &Value;
            let option_card_attributes: Option<&Value> = card.get("card_attributes");
            print_formatted_log_string(format!("Card attributes: {:?}", option_card_attributes));
            match option_card_attributes {
                Some(value) => card_attributes = value,
                None => panic!("Unable to find card_attributes field in json"),
            }
            let option_colors_vector: Option<&Vec<Value>> = card_attributes["colors"].as_array();
            print_formatted_log_string(format!("Colors in array: {:?}", option_colors_vector));
            let mut colors_vector: &Vec<Value> = &Vec::new();
            match option_colors_vector {
                Some(value) => colors_vector = value,
                None => {
                    println!("!WARNING A card without the color field has been found, data about this card will be incomplete");
                }
            }
            for color_index in 0..colors_array.len() {
                for card_color in colors_vector {
                    let card_color_string: String;
                    match serde_json::to_string(card_color) {
                        Ok(value) => card_color_string = value,
                        Err(_) => panic!("Unable to convert color to String"),
                    }
                    if card_color_string == colors_array[color_index] {
                        player_colors_data[color_index] = true;
                    }
                }
            }
            let card_quantity_string: &str;
            match &card["qty"] {
                Value::String(value) => card_quantity_string = value,
                _ => panic!("Unable to find qty in json"),
            }
            let result_card_quantity: Result<u8, ParseIntError> =
                card_quantity_string.parse::<u8>();
            print_formatted_log_string(
                format!(
                    "Result card quantity: {:?}",
                    result_card_quantity,
                )
            );
            let optional_card_type: Option<&str>;
            match &card_attributes["card_type"] {
                Value::String(value) => optional_card_type = Some(value),
                _ => optional_card_type = None,
            }
            let card_quantity: u8;
            match card_quantity_string.parse::<u8>() {
                Ok(value) => card_quantity = value,
                Err(error) => panic!("Unable to parse {:?} to u8: {:?}", card_quantity_string, error),
            }
            if let Some(card_type_value) = optional_card_type {
                for card_type_index in 0..cards_type_array.len() {
                    if card_type_value == cards_type_array[card_type_index] {
                        player_card_type_count_data[card_type_index] += card_quantity;
                    }
                }
            }
            let value_card_cost: &Value = &card_attributes["cost"];
            print_formatted_log_string(format!("Cost from card attribute: {:?}", value_card_cost));
            let option_card_cost_string: Option<&str>;
            match value_card_cost {
                Value::String(value) => option_card_cost_string = Some(value),
                _ => option_card_cost_string = None,
            }
            if let Some(card_cost_string) = option_card_cost_string {
                let result_card_cost: Result<usize, ParseIntError> = card_cost_string.parse::<usize>();
                print_formatted_log_string(format!("Parsed result card cost: {:?}", result_card_cost));
                let card_cost: usize;
                match result_card_cost {
                    Ok(value) => card_cost = value,
                    Err(error) => panic!("Unable to parse {:?} to u8: {:?}", card_cost_string, error),
                }
                for _ in 0..card_quantity {
                    cards_cost.push(card_cost);
                }
            }
        }
        for feature_index in 0..colors_array.len() {
            color_data_matrix[feature_index].push(player_colors_data[feature_index]);
        }
        for feature_index in 0..player_card_type_count_data.len() {
            card_type_matrix[feature_index].push(player_card_type_count_data[feature_index]);
        }
        let cards_cost_sum: f32 = cards_cost.iter().sum::<usize>() as f32;
        card_average_cost_data.push(cards_cost_sum / (cards_cost.len() as f32));
    }
    println!("{:?}", colors_array);
    data_matrix.append(&mut color_data_matrix);
    let mut unique_card_type_average_matrix: Vec<Vec<f32>> = Vec::new();
    for card_type_index in 0..card_type_matrix.len() {
        let mut tmp_vector: Vec<u8> = Vec::new();
        for target_value in &card_type_matrix[card_type_index] {
            let mut contains_value: bool = false;
            for unique_value in &tmp_vector {
                if target_value == unique_value {
                    contains_value = true;
                    break;
                }
            }
            if !contains_value {
                tmp_vector.push(*target_value);
            }
        }
        tmp_vector.sort();
        let mut feature_vector: Vec<f32> = Vec::new();
        for vector_index in 1..tmp_vector.len() {
            let value: f32 = (tmp_vector[vector_index - 1] as f32 + tmp_vector[vector_index] as f32) / 2.0;
            feature_vector.push(value);
        }
        unique_card_type_average_matrix.push(feature_vector);
    }
    for card_type_index in 0..cards_type_array.len() {
        println!(
            "Unique {} values: {:?}",
            &cards_type_array[card_type_index],
            &unique_card_type_average_matrix[card_type_index],
        );
    }
    let mut card_type_data_matrix: Vec<Vec<bool>> = Vec::new();
    for card_type_index in 0..card_type_matrix.len() {
        for unique_value_index in 0..unique_card_type_average_matrix[card_type_index].len() {
            let mut tmp_vector: Vec<bool> = Vec::new();
            for player_index in 0..card_type_matrix[card_type_index].len() {
                tmp_vector.push(
                    (card_type_matrix[card_type_index][player_index] as f32) <
                    unique_card_type_average_matrix[card_type_index][unique_value_index]
                );
            }
            card_type_data_matrix.push(tmp_vector);
        }
    }
    data_matrix.append(&mut card_type_data_matrix);
    let mut tmp_vector: Vec<f32> = Vec::new();
    for target_value in &card_average_cost_data {
        let mut contains_value: bool = false;
        for unique_value in &tmp_vector {
            if *target_value == *unique_value {
                contains_value = true;
                break;
            }
        }
        if !contains_value {
            tmp_vector.push(*target_value);
        }
    }
    for vector_index in 0..tmp_vector.len() {
        for second_vector_index in 0..tmp_vector.len() {
            if tmp_vector[vector_index] < tmp_vector[second_vector_index] {
                let tmp_value: f32 = tmp_vector[vector_index];
                tmp_vector[vector_index] = tmp_vector[second_vector_index];
                tmp_vector[second_vector_index] = tmp_value;
            }
        }
    }
    let mut feature_vector: Vec<f32> = Vec::new();
    for vector_index in 1..tmp_vector.len() {
        let value: f32 = (tmp_vector[vector_index - 1] as f32 + tmp_vector[vector_index] as f32) / 2.0;
        feature_vector.push(value);
    }
    println!("Unique Mana Value Average values: {:?}", feature_vector);
    let mut feature_data: Vec<Vec<bool>> = Vec::new();
    for unique_value in feature_vector {
        let mut unique_value_data: Vec<bool> = Vec::new();
        for target_value in &card_average_cost_data {
            unique_value_data.push((*target_value as f32) < unique_value);
        }
        feature_data.push(unique_value_data);
    }
    data_matrix.append(&mut feature_data);
    let mut deck_position_less_than_9_data: Vec<bool> = Vec::new();
    for player_index in 0..players.len() - 1 {
        deck_position_less_than_9_data.push(decks_rank[player_index] < 8);
    }
    let max_training_index: usize =
        (deck_position_less_than_9_data.len() as f32 * TRAINING_PERCENTAGE) as usize;
    println!("N of training decks: {:?}", max_training_index);
    let mut rng_thread: rand::rngs::ThreadRng = rand::thread_rng();
    let mut random_vector: Vec<usize> = Vec::from_iter(0..deck_position_less_than_9_data.len());
    random_vector.shuffle(&mut rng_thread);
    for (index, value) in random_vector.iter().enumerate() {
        for data_vector in &mut data_matrix {
            let tmp: bool = data_vector[index];
            data_vector[index] = data_vector[*value];
            data_vector[*value] = tmp;
        }
    }
    let mut forest: Vec<Node> = Vec::new();
    for _ in 0..100 {
        let mut bootstrapped_data: Vec<Vec<bool>> = Vec::new();
        let mut random_numbers: Vec<usize> = Vec::new();
        for _ in 0..max_training_index {
            random_numbers.push(rng_thread.gen_range(0..max_training_index));
        }
        for data_vector in &data_matrix {
            let mut bootstrapped_vector: Vec<bool> = Vec::new();
            for random_number in &random_numbers {
                bootstrapped_vector.push(data_vector[*random_number]);
            }
            bootstrapped_data.push(bootstrapped_vector);
        }
        let generated_tree: Node = generate_nodes(
            &Vec::from_iter(0..max_training_index),
            &bootstrapped_data,
            &deck_position_less_than_9_data[0..max_training_index].to_vec(),
            (max_training_index as f32 * 0.01) as usize,
            f32::sqrt(data_matrix.len() as f32) as usize,
            Vec::new(),
        );
        forest.push(generated_tree);
    }
    let mut correct_predictions: usize = 0;
    let mut total_predictions: usize = 0;
    for index in max_training_index..deck_position_less_than_9_data.len() {
        let mut number_of_true: usize = 0;
        let mut number_of_false: usize = 0;
        total_predictions += 1;
        for tree in &forest {
            let mut vector_data: Vec<bool> = Vec::new();
            for data_vector in &data_matrix {
                vector_data.push(data_vector[index]);
            }
            if evaluate_data(&tree, vector_data) {
                number_of_true += 1;
            } else {
                number_of_false += 1;
            }
        }
        if (number_of_true >= number_of_false) && deck_position_less_than_9_data[index] {
            correct_predictions += 1;
        } else if (number_of_false >= number_of_true) && !deck_position_less_than_9_data[index] {
            correct_predictions += 1;
        }
    }
    println!(
        "Total number of prediction: {:?}\nNumber of correct predictions: {:?}\nAccuracy: {:?}%",
        total_predictions,
        correct_predictions,
        correct_predictions as f32 / total_predictions as f32 * 100.0,
    );
    let mut number_of_true: usize = 0;
    let mut number_of_false: usize = 0;
    for tree in &forest {
        let mut vector_data: Vec<bool> = Vec::new();
        for data_vector in &data_matrix {
            vector_data.push(data_vector[data_vector.len() - 1]);
        }
        if evaluate_data(&tree, vector_data) {
            number_of_true += 1;
        } else {
            number_of_false += 1;
        }
    }
    println!("For Evaluation... True: {} False: {}", number_of_true, number_of_false);
    test!();
}
