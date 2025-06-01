/* 
 * N of decks = 640
 * Version with everything = 77.06422% 78.125
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

use std::{thread, time::Duration};

use serde_json::Value;
use reqwest::blocking::{Client, Response};
use rand::{Rng, seq::SliceRandom};

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    gini_impurity: Option<f32>,
    feature_index: Option<usize>,
    on_true: Option<Box<Node>>,
    on_false: Option<Box<Node>>,
    prediction: Option<bool>,
}

fn has_card_color(deck: &Vec<Value>, color_tag: String) -> Result<bool, &'static str> {
    for card in deck {
        let card_attributes: &Value;
        match card.get("card_attributes") {
            Some(value) => card_attributes = value,
            None => return Err("Unable to find card_attributes field in json"),
        }
        let colors_vector: &Vec<Value>;
        match card_attributes["colors"].as_array() {
            Some(value) => colors_vector = value,
            None => return Err("Unable to find colors tag on json"),
        }
        for card_color in colors_vector {
            let card_color_string: String;
            match serde_json::to_string(card_color) {
                Ok(value) => card_color_string = value,
                Err(_) => return Err("Unable to convert color to String"),
            }
            if card_color_string == color_tag {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

fn panic_on_try_value_exceding_max_tries(try_value: usize, error_message: String) {
    if try_value > MAX_TRIES {
        panic!("{}", error_message);
    }
    println!("{}", error_message);
}

fn get_training_data_from_matrix(original_data: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut training_data: Vec<Vec<bool>> = Vec::new();
    for data_values in original_data {
        let mut data_vector: Vec<bool> = Vec::new();
        for value in data_values {
            data_vector.push(*value);
        }
        training_data.push(data_vector);
    }
    return training_data;
}

fn get_data_values(target_data: &Vec<u8>, values: Vec<f32>) -> Vec<Vec<bool>> {
    let mut lands_data: Vec<Vec<bool>> = Vec::new();
    for unique_value in values {
        let mut unique_value_data: Vec<bool> = Vec::new();
        for target_value in target_data {
            let tmp: bool = (*target_value as f32) < unique_value;
            unique_value_data.push(tmp);
        }
        lands_data.push(unique_value_data);
    }
    return lands_data;
}

fn get_feature_values(target: &Vec<u8>) -> Vec<f32> {
    let mut tmp_vector: Vec<u8> = Vec::new();
    for target_value in target {
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
    tmp_vector.sort();
    let mut final_vector: Vec<f32> = Vec::new();
    for vector_index in 1..tmp_vector.len() {
        let value: f32 = (tmp_vector[vector_index - 1] as f32 + tmp_vector[vector_index] as f32) / 2.0;
        final_vector.push(value);
    }
    return final_vector;
}

fn get_card_type_quantity(card: &Value, card_type_target: &str) -> Result<u8, &'static str> {
    let card_attributes: &Value;
    match card.get("card_attributes") {
        Some(value) => card_attributes = value,
        None => return Err("Unable to find card_attributes field in json"),
    }
    let card_type: &str;
    match &card_attributes["card_type"] {
        Value::String(value) => card_type = value,
        _ => return Err("Unable to find card_type in json"),
    }
    if card_type == card_type_target {
        let card_quantity_string: &str;
        match &card["qty"] {
            Value::String(value) => card_quantity_string = value,
            _ => return Err("Unable to find qty in json"),
        }
        match card_quantity_string.parse::<u8>() {
            Ok(value) => return Ok(value),
            Err(_) => return Err("Unable to parse u8 card quantity"),
        }
    }
    return Ok(0);
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
    let mut urls: Vec<String> = Vec::new();
    let url_queries: Vec<&str> = Vec::from([
        "04-0412763152",
        "04-0512763169",
        "04-0612763187",
        "04-1112765765",
        "04-1212765782",
        "04-1812769888",
        "04-1912769905",
/*        "04-2012769922",
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
        "05-1812780167", */
    ]);
    for url_query in &url_queries {
        urls.push(format!("https://{}/decklist/pauper-challenge-32-2025-{}", HOST, url_query));
    }
    let mut players: Vec<Value> = Vec::new();
    let mut decks_rank: Vec<u64> = Vec::new();
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
        let json_standings: Vec<&Value>;
        match json_data[STANDINGS_KEY].as_array() {
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
            println!("{}", json_players[player_index].to_string());
            players.push(json_players[player_index].clone());
        }
    }
    println!("Html players are: {:?}", players.len());
    let mut has_white_data: Vec<bool> = Vec::new();
    let mut has_blue_data: Vec<bool> = Vec::new();
    let mut has_black_data: Vec<bool> = Vec::new();
    let mut has_red_data: Vec<bool> = Vec::new();
    let mut has_green_data: Vec<bool> = Vec::new();
    let mut lands_count_data: Vec<u8> = Vec::new();
    let mut creatures_count_data: Vec<u8> = Vec::new();
    let mut instants_count_data: Vec<u8> = Vec::new();
    let mut sorceries_count_data: Vec<u8> = Vec::new();
    let mut artifacts_count_data: Vec<u8> = Vec::new();
    let mut enchantments_count_data: Vec<u8> = Vec::new();
    let mut deck_position_less_than_9_data: Vec<bool> = Vec::new();
    for player_index in 0..players.len() {
        deck_position_less_than_9_data.push(decks_rank[player_index] < 8);
        let player_data: &Value;
        match players[player_index].get("main_deck") {
            Some(value) => player_data = value,
            None => panic!("Unable to find main_deck field in json"),
        }
        let deck: &Vec<Value>;
        match player_data.as_array() {
            Some(value) => deck = value,
            None => panic!("Unable to convert deck to Vec: {:?}", player_data), 
        }
        match has_card_color(deck, format!("{}{}", COLOR_TAG, "WHITE")) {
            Ok(value) => has_white_data.push(value),
            Err(error) => panic!("{}", error),
        }
        match has_card_color(deck, format!("{}{}", COLOR_TAG, "BLUE")) {
            Ok(value) => has_blue_data.push(value),
            Err(error) => panic!("{}", error),
        }
        match has_card_color(deck, format!("{}{}", COLOR_TAG, "BLACK")) {
            Ok(value) => has_black_data.push(value),
            Err(error) => panic!("{}", error),
        }
        match has_card_color(deck, format!("{}{}", COLOR_TAG, "RED")) {
            Ok(value) => has_red_data.push(value),
            Err(error) => panic!("{}", error),
        }
        match has_card_color(deck, format!("{}{}", COLOR_TAG, "GREEN")) {
            Ok(value) => has_green_data.push(value),
            Err(error) => panic!("{}", error),
        }
        let mut lands_count: u8 = 0;
        let mut creatures_count: u8 = 0;
        let mut instants_count: u8 = 0;
        let mut sorceries_count: u8 = 0;
        let mut artifacts_count: u8 = 0;
        let mut enchantments_count: u8 = 0;
        for card in deck {
            match get_card_type_quantity(card, "LAND  ") {
                Ok(value) => lands_count += value,
                Err(error) => panic!("{}", error),
            }
            match get_card_type_quantity(card, "ISCREA") {
                Ok(value) => creatures_count += value,
                Err(error) => panic!("{}", error),
            }
            match get_card_type_quantity(card, "INSTNT") {
                Ok(value) => instants_count += value,
                Err(error) => panic!("{}", error),
            }
            match get_card_type_quantity(card, "SORCRY") {
                Ok(value) => sorceries_count += value,
                Err(error) => panic!("{}", error),
            }
            match get_card_type_quantity(card, "ARTFCT") {
                Ok(value) => artifacts_count += value,
                Err(error) => panic!("{}", error),
            }
            match get_card_type_quantity(card, "ENCHMT") {
                Ok(value) => enchantments_count += value,
                Err(error) => panic!("{}", error),
            }
        }
        lands_count_data.push(lands_count);
        creatures_count_data.push(creatures_count);
        instants_count_data.push(instants_count);
        sorceries_count_data.push(sorceries_count);
        artifacts_count_data.push(artifacts_count);
        enchantments_count_data.push(enchantments_count);
    }
    let feature_lands_values: Vec<f32> = get_feature_values(&lands_count_data);
    let feature_creatures_values: Vec<f32> = get_feature_values(&creatures_count_data);
    let feature_instants_values: Vec<f32> = get_feature_values(&instants_count_data);
    let feature_sorceries_values: Vec<f32> = get_feature_values(&sorceries_count_data);
    let feature_artifacts_values: Vec<f32> = get_feature_values(&artifacts_count_data);
    let feature_enchantments_values: Vec<f32> = get_feature_values(&enchantments_count_data);
    println!("0:W, 1:U, 2:B, 3:R, 4:G");
    println!("Unique lands values: {:?}", feature_lands_values);
    println!("Unique creatures values: {:?}", feature_creatures_values);
    println!("Unique instants values: {:?}", feature_instants_values);
    println!("Unique sorceries values: {:?}", feature_sorceries_values);
    println!("Unique artifacts values: {:?}", feature_artifacts_values);
    println!("Unique enchantments values: {:?}", feature_enchantments_values);
    let lands_data: Vec<Vec<bool>> = get_data_values(&lands_count_data, feature_lands_values);
    let creatures_data: Vec<Vec<bool>> = get_data_values(&creatures_count_data, feature_creatures_values);
    let instants_data: Vec<Vec<bool>> = get_data_values(&instants_count_data, feature_instants_values);
    let sorceries_data: Vec<Vec<bool>> = get_data_values(&sorceries_count_data, feature_sorceries_values);
    let artifacts_data: Vec<Vec<bool>> = get_data_values(&artifacts_count_data, feature_artifacts_values);
    let enchantments_data: Vec<Vec<bool>> =
        get_data_values(&enchantments_count_data, feature_enchantments_values);
    let max_training_index: usize =
        (deck_position_less_than_9_data.len() as f32 * TRAINING_PERCENTAGE) as usize;
    println!("N of training decks: {:?}", max_training_index);
    let mut data_matrix: Vec<Vec<bool>> = Vec::from([
        has_white_data.to_owned(),
        has_blue_data.to_owned(),
        has_black_data.to_owned(),
        has_red_data.to_owned(),
        has_green_data.to_owned(),
    ]);
    data_matrix.append(&mut get_training_data_from_matrix(&lands_data));
    data_matrix.append(&mut get_training_data_from_matrix(&creatures_data));
    data_matrix.append(&mut get_training_data_from_matrix(&instants_data));
    data_matrix.append(&mut get_training_data_from_matrix(&sorceries_data));
    data_matrix.append(&mut get_training_data_from_matrix(&artifacts_data));
    data_matrix.append(&mut get_training_data_from_matrix(&enchantments_data));
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
}
