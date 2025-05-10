/*
 * N of decks = 480
 * Version without lands = 78.125%
 * Version with lands = 80.20833%
 * Version with creatures = 69.79167%
 */

use std::{thread, time::Duration};

use serde_json::Value;
use reqwest::blocking::{Client, Response};

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    gini_impurity: Option<f32>,
    feature_index: Option<usize>,
    on_true: Option<Box<Node>>,
    on_false: Option<Box<Node>>,
    prediction: Option<bool>,
}

fn get_training_data_from_matrix(original_data: &Vec<Vec<bool>>, max_training_index: usize) -> Vec<Vec<bool>> {
    let mut training_data: Vec<Vec<bool>> = Vec::new();
    for data_values in original_data {
        let mut data_vector: Vec<bool> = Vec::new();
        for data_index in 0..max_training_index {
            data_vector.push(data_values[data_index]);
        }
        training_data.push(data_vector);
    }
    return training_data;
}

fn get_data_values(target_data: &Vec<u8>, values: Vec<u8>) -> Vec<Vec<bool>> {
    let mut lands_data: Vec<Vec<bool>> = Vec::new();
    for unique_value in values {
        let mut unique_value_data: Vec<bool> = Vec::new();
        for target_value in target_data {
            let tmp: bool = *target_value < unique_value;
            unique_value_data.push(tmp);
        }
        lands_data.push(unique_value_data);
    }
    return lands_data;
}

fn get_unique_values(target: &Vec<u8>) -> Vec<u8> {
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
    return tmp_vector;
}

fn get_card_type_quantity(html_card: &str, card_type: &str) -> Result<Option<u8>, &'static str> {
    let amount: u8;
    if html_card.contains(&format!("\"card_type\":\"{}\",", card_type)) {
        let first_index: usize;
        if let Some(value) = html_card.find("\"qty\":\"") {
            first_index = value;
        } else {
            return Err("Unable to find card type first index");
        }
        let second_index: usize;
        if let Some(value) = html_card.find("\",\"sideboard\"") {
            second_index = value;
        } else {
            return Err("Unable to find card type second index");
        }
        if let Ok(value) = html_card[first_index + 7..second_index].parse::<u8>() {
            amount = value;
        } else {
            return Err("Unable to parse card type amount");
        }
    } else {
        return Ok(None);
    }
    return Ok(Some(amount));
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
) -> Node {
    let (node_gini_impurity, number_of_true, number_of_false) = get_node_gini_impurity(target, indexes);
    if node_gini_impurity == 0.0 {
        return leaf_node(number_of_true, number_of_false, node_gini_impurity);
    }
    let mut smaller_gini: f32 = 1.0;
    let mut smaller_feature_index: usize = usize::MAX;
    let mut on_smaller_true_indexes: Vec<usize> = Vec::new();
    let mut on_smaller_false_indexes: Vec<usize> = Vec::new();
    for feature_index in 0..data.len() {
        let mut true_indexes: Vec<usize> = Vec::new();
        let mut false_indexes: Vec<usize> = Vec::new();
        for data_index in indexes {
            if data[feature_index][*data_index] {
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
            smaller_feature_index = feature_index;
            on_smaller_true_indexes = true_indexes;
            on_smaller_false_indexes = false_indexes;
        }
    }
    if f32::abs(smaller_gini - node_gini_impurity) <= 0.000001 {
        return leaf_node(number_of_true, number_of_false, node_gini_impurity);
    }
    let on_true_node: Node = generate_nodes(
        &on_smaller_true_indexes,
        data,
        target
    );
    let on_false_node: Node = generate_nodes(
        &on_smaller_false_indexes,
        data,
        target
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
    const COOKIES: &str = "locale=en_US; tarteaucitron=!dgcMultiplegtagUa=wait; JSESSIONID=DAD72B351762EC125A1B443CDBEAC070.lvs-foyert2-3409";
    let mut html_decks: Vec<String> = Vec::new();
    let mut ranks: Vec<u64> = Vec::new();
    const HOST: &str = "www.mtgo.com";
    let mtgo_uri: String = format!("https://{}/decklist/pauper-challenge-32-2025-", HOST); 
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
    ]);
    for url_query in &url_queries {
        urls.push(format!("{}{}", mtgo_uri, url_query));
    }
    for url_index in 0..url_queries.len() {
        let url: &str = &urls[url_index];
        println!("Requesting url: {:?}", url);
        thread::sleep(Duration::from_secs(7));
        let client: Client = Client::new();
        const MAX_TRIES: usize = 5;
        let mut page_html: String = String::new();
        for try_value in 1..=MAX_TRIES {
            println!("Try {:?}", try_value);
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
                    println!("Unable to get the response: {:?}", error);
                    continue;
                }
            }
            let raw_page_html: String;
            let response_text = response.text();
            match response_text {
                Ok(value) => raw_page_html = value,
                Err(error) => {
                    println!("Unable to find text of the response: {:?}", error);
                    continue;
                }
            }
            const FIST_INDEX_STRING: &str = r#"window.MTGO.decklists.data = "#;
            let first_index: usize;
            let second_index: usize;
            if let (Some(first_value), Some(second_value)) = (
                raw_page_html.find(FIST_INDEX_STRING),
                raw_page_html.find(r#"window.MTGO.decklists.type = "#)
            ) {
                first_index = first_value;
                second_index = second_value;
            } else {
                println!("{:?}", raw_page_html);
                println!("Unable to find json in html document");
                continue;
            }
            page_html = raw_page_html[first_index + FIST_INDEX_STRING.len()..second_index - 6].to_string();
            break;
        }
        let end_decks_index: usize;
        if let Some(value) = page_html.find("\"brackets\":") {
            end_decks_index = value;
        } else {
            panic!("End decks index not found");
        }
        let raw_decks: Vec<&str> = page_html[..end_decks_index].split(r#""loginid":""#).collect();
        println!("Request recived with: {:?} games", raw_decks.len() - 1);
        if raw_decks.len() - 1 != 32 {
            panic!("N of decks not 32: {:?}", page_html);
        }
        let mut login_ids: Vec<&str> = Vec::new();
        for raw_deck in &raw_decks {
            let login_id_end_index: usize;
            if let Some(value) = raw_deck.find("\"") {
                login_id_end_index = value;
            } else {
               panic!("Value not found");
            }
            let login_id: &str = &raw_deck[..login_id_end_index];
            login_ids.push(login_id);
        }
        const STANDIGS_KEY: &str = "standings";
        match serde_json::from_str::<Value>(&page_html) {
            Ok(parsed_data) => {
                let standings: Vec<&Value>;
                if let Some(value) = parsed_data[STANDIGS_KEY].as_array() {
                    standings = Vec::from_iter(value);
                } else {
                    panic!("Unable to read key {:?} from json", STANDIGS_KEY);
                }
                let standings: Vec<&Value> = Vec::from_iter(standings);
                let mut tmp_ranks: Vec<u64> = Vec::new();
                for object in standings {
                    const RANK_KEY: &str = "rank";
                    let object_rank: &str;
                    if let Value::String(value) = &object[RANK_KEY] {
                        object_rank = value;
                    } else {
                        panic!("Unable to read key {:?} from json", RANK_KEY);
                    }
                    if let Ok(rank_value) = object_rank.parse::<u64>() {
                        ranks.push(rank_value);
                    }
                }
                ranks.append(&mut tmp_ranks);
            },
            Err(error) => panic!("Unable to parse json: {:?}", error),
        };
        for deck_index in 1..raw_decks.len() {
            html_decks.push(raw_decks[deck_index].to_string());
        }
    }
    println!("Html decks are: {:?}", html_decks.len());
    let mut has_white_data: Vec<bool> = Vec::new();
    let mut has_blue_data: Vec<bool> = Vec::new();
    let mut has_black_data: Vec<bool> = Vec::new();
    let mut has_red_data: Vec<bool> = Vec::new();
    let mut has_green_data: Vec<bool> = Vec::new();
    let mut lands_count_data: Vec<u8> = Vec::new();
    let mut creatures_count_data: Vec<u8> = Vec::new();
    let mut deck_position_less_than_9_data: Vec<bool> = Vec::new();
    for html_deck_index in 0..html_decks.len() {
        deck_position_less_than_9_data.push(ranks[html_deck_index] < 8);
        let html_cards: Vec<&str> =
            html_decks[html_deck_index]
            .split(r#""docid":""#).collect();
        let mut has_white_mana: bool = false;
        let mut has_blue_mana: bool = false;
        let mut has_black_mana: bool = false;
        let mut has_red_mana: bool = false;
        let mut has_green_mana: bool = false;
        let mut lands_count: u8 = 0;
        let mut creatures_count: u8 = 0;
        const COLOR_TAG: &str = "COLOR_";
        for html_card in html_cards {
            has_white_mana = html_card.contains(&format!("{}{}", COLOR_TAG, "WHITE"));
            has_blue_mana = html_card.contains(&format!("{}{}", COLOR_TAG, "BLUE"));
            has_black_mana = html_card.contains(&format!("{}{}", COLOR_TAG, "BLACK"));
            has_red_mana = html_card.contains(&format!("{}{}", COLOR_TAG, "RED"));
            has_green_mana = html_card.contains(&format!("{}{}", COLOR_TAG, "GREEN"));
            match get_card_type_quantity(html_card, "LAND  ") {
                Ok(Some(value)) => lands_count += value,
                Err(error) => panic!("{}", error),
                _ => (),
            }
            match get_card_type_quantity(html_card, "ISCREA") {
                Ok(Some(value)) => creatures_count += value,
                Err(error) => panic!("{}", error),
                _ => (),
            }
        }
        has_white_data.push(has_white_mana);
        has_blue_data.push(has_blue_mana);
        has_black_data.push(has_black_mana);
        has_red_data.push(has_red_mana);
        has_green_data.push(has_green_mana);
        lands_count_data.push(lands_count);
        creatures_count_data.push(creatures_count);
    }
    let unique_lands_values: Vec<u8> = get_unique_values(&lands_count_data);
    let unique_creatures_values: Vec<u8> = get_unique_values(&creatures_count_data);
    println!("0:W, 1:U, 2:B, 3:R, 4:G");
    println!("Unique lands values: {:?}", unique_lands_values);
    println!("Unique creatures values: {:?}", unique_creatures_values);
    let lands_data: Vec<Vec<bool>> = get_data_values(&lands_count_data, unique_lands_values);
    let creatures_data: Vec<Vec<bool>> = get_data_values(&creatures_count_data, unique_creatures_values);
    const TRAINING_PERCENTAGE: f32 = 0.8;
    let max_training_index: usize = (deck_position_less_than_9_data.len() as f32 * TRAINING_PERCENTAGE) as usize;
    println!("N of training decks: {:?}", max_training_index);
    let mut data_array_map: Vec<Vec<bool>> = Vec::from([
        has_white_data[0..max_training_index].to_owned(),
        has_blue_data[0..max_training_index].to_owned(),
        has_black_data[0..max_training_index].to_owned(),
        has_red_data[0..max_training_index].to_owned(),
        has_green_data[0..max_training_index].to_owned(),
    ]);
    data_array_map.append(&mut get_training_data_from_matrix(&lands_data, max_training_index));
    data_array_map.append(&mut get_training_data_from_matrix(&creatures_data, max_training_index));
    let generated_nodes: Node = generate_nodes(
        &Vec::from_iter(0..max_training_index),
        &data_array_map,
        &deck_position_less_than_9_data[0..max_training_index].to_vec(),
    );
    println!("Node: {:?}", generated_nodes);
    let mut correct_predictions: usize = 0;
    let mut total_predictions: usize = 0;
    for index in max_training_index..deck_position_less_than_9_data.len() {
        total_predictions += 1;
        let mut vector_data: Vec<bool> = Vec::from([has_white_data[index], has_blue_data[index], has_black_data[index], has_red_data[index], has_green_data[index]]);
        for land_data in &lands_data {
            vector_data.push(land_data[index]);
        }
        for creature_data in &creatures_data {
            vector_data.push(creature_data[index]);
        }
        let prediction: bool = evaluate_data(&generated_nodes, vector_data);
        println!("The prediction for data is: {:?}", prediction);
        if prediction == deck_position_less_than_9_data[index] {
            correct_predictions += 1;
        }
    }
    println!("Prediction %: {:?}\nCorect: {:?}\nTotal: {:?}", correct_predictions as f32 / total_predictions as f32 * 100.0, correct_predictions, total_predictions);
}
