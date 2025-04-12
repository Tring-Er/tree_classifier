use reqwest;
use std;

#[derive(Debug)]
struct Node {
    gini_impurity: Option<f32>,
    feature_index: Option<usize>,
    on_true: Option<Box<Node>>,
    on_false: Option<Box<Node>>,
    prediction: Option<bool>,
}

fn correctness_score(
    number_of_correct: usize,
    number_of_total: usize
) -> Result<f32, String> {
    if number_of_total == 0 {
        return Err(
            String::from(
                "correctness_score argument \"number_of_total\" cannot be 0"
                )
            );
    }
    return Ok(f32::powi(number_of_correct as f32 / number_of_total as f32, 2));
}

fn gini_impurity(
    true_amounts: usize,
    false_amounts: usize,
) -> f32 {
    let total_number_in_branch: usize = true_amounts + false_amounts;
    let left_correctness_score: f32;
    match correctness_score(true_amounts, total_number_in_branch) {
        Ok(value) => left_correctness_score = value,
        Err(_) => return 1.0,
    };
    let right_corerctness_score: f32;
    match correctness_score(false_amounts, total_number_in_branch) {
        Ok(value) => right_corerctness_score = value,
        Err(_) => return 1.0,
    };
    return 1.0 - left_correctness_score - right_corerctness_score;
}

fn evaluate_data(tree: Node, data: Vec<bool>) -> bool {
    if let Some(value) = tree.prediction {
        return value;
    }
    let mut data_result: bool = false;
    if let Some(feature_index) = tree.feature_index {
        data_result = data[feature_index];
    }
    if data_result {
        if let Some(node) = tree.on_true {
            return evaluate_data(*node, data);
        }
    } else {
        if let Some(node) = tree.on_false {
            return evaluate_data(*node, data);
        }
    }
    return false;
}

fn true_false_amounts(
    target: &Vec<bool>,
    indexes: &Vec<usize>
) -> (usize, usize) {
    let mut true_amounts: usize = 0;
    let mut false_amounts: usize = 0;
    for index in indexes {
        if target[*index] {
            true_amounts += 1;
        } else {
            false_amounts += 1;
        }
    }
    return (true_amounts, false_amounts);
}

fn leaf_node(
    true_amounts: usize,
    false_amounts: usize,
    gini_impurity: f32
) -> Node {
    let is_true: bool;
    if true_amounts < false_amounts {
        is_true = false;
    } else {
        is_true = true;
    }
    return Node {
        gini_impurity: Some(gini_impurity),
        feature_index: None,
        on_true: None,
        on_false: None,
        prediction: Some(is_true),
    };
}

fn generate_nodes(
    indexes: &Vec<usize>,
    data: &Vec<Vec<bool>>,
    target: &Vec<bool>,
) -> Node {
    let (number_of_true, number_of_false) = true_false_amounts(target, indexes);
    let node_gini_impurity: f32 = gini_impurity(number_of_true, number_of_false);
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
        let (true_amounts, false_amounts) = true_false_amounts(
            target,
            &true_indexes
        );
        let true_gini_impurity: f32 = gini_impurity(true_amounts, false_amounts);
        let (true_amounts, false_amounts) = true_false_amounts(
            target,
            &false_indexes,
        );
        let false_gini_impurity: f32 = gini_impurity(
            true_amounts,
            false_amounts
        );
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
    const N: usize = 1_000_000;
    std::thread::Builder::new().stack_size(std::mem::size_of::<f64>() * 700 * N).spawn(|| {
    let mut page_html: String = String::new();
    let mut html_decks: Vec<String> = Vec::new();
    let mut counter: usize = 1;
    let urls: Vec<&str> = Vec::from(["https://www.mtgo.com/decklist/pauper-league-2025-04-018957", "https://www.mtgo.com/decklist/pauper-league-2025-04-028957", "https://www.mtgo.com/decklist/pauper-league-2025-04-038957", "https://www.mtgo.com/decklist/pauper-league-2025-04-048957", "https://www.mtgo.com/decklist/pauper-league-2025-04-058957", "https://www.mtgo.com/decklist/pauper-league-2025-04-068957", "https://www.mtgo.com/decklist/pauper-league-2025-04-078957", "https://www.mtgo.com/decklist/pauper-league-2025-04-089081", "https://www.mtgo.com/decklist/pauper-league-2025-04-088957", "https://www.mtgo.com/decklist/pauper-league-2025-04-099081", "https://www.mtgo.com/decklist/pauper-league-2025-04-109081"]);
    for urls_index_sets in [vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]] {
        html_decks = Vec::new();
    for url_index in urls_index_sets{
        println!("Requesting n: {:?}", counter);
        counter += 1;
        std::thread::sleep(std::time::Duration::from_secs(10));
    let client: reqwest::blocking::Client = match reqwest::blocking::ClientBuilder::new().build() {
        Ok(client_settings) => client_settings,
        _ => panic!("Client settings inccorrect"),
    };
    let url: &str = urls[url_index];
    let response: Result<reqwest::blocking::Response, reqwest::Error> = client
.get(url)
        .header("Host", "www.mtgo.com")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .header("Accetp", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-Us,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Referer", "https://www.mtgo.com/decklists?filter=Pauper")
        .header("Sec-GPC", "1")
        .header("Connection", "keep-alive")
        .header("Cookie", "JSESSIONID=576B6692664D751DF906E3434405398D.lvs-foyert1-3409; locale=en_US; tarteaucitron=!dgcMultiplegtagUa=wait")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Sec-Fetch-User", "?1")
        .header("Priority", "u=0, i")
        .send();
    if let Ok(resp) = response {
        let mut page_html: String = resp.text().expect("failed");

        if let (Some(first_index), Some(second_index)) = (page_html.find(
            r#"window.MTGO.decklists.data = "#
        ),
        page_html.find(
            r#"window.MTGO.decklists.type = "#
        )) {
            page_html = page_html[first_index + 29..second_index - 6].to_string();
        }
        
        let unfiltered_decks: Vec<&str> = page_html.split(r#""loginid":""#).collect();
        println!("Request recived with: {:?} games", unfiltered_decks.len() - 1);
        for deck_index in 1..unfiltered_decks.len() {
            html_decks.push(unfiltered_decks[deck_index].to_string());
        }
    }
    }
    println!("Html decks are: {:?}", html_decks.len());
    let mut has_white_data: Vec<bool> = Vec::new();
    let mut has_blue_data: Vec<bool> = Vec::new();
    let mut has_black_data: Vec<bool> = Vec::new();
    let mut has_red_data: Vec<bool> = Vec::new();
    let mut has_green_data: Vec<bool> = Vec::new();
    let mut lands_count: Vec<u8> = Vec::new();
    let mut deck_position_less_than_9_data: Vec<bool> = Vec::new();
    let random_scoring: [u8; 7] = [1, 1, 2, 5, 8, 13, 21];
    for html_deck_index in 0..html_decks.len() {
        deck_position_less_than_9_data.push(random_scoring[html_deck_index % 7] < 8);
        let html_cards: Vec<&str> =
            html_decks[html_deck_index]
            .split(r#""docid":""#).collect();
        let mut has_white_mana: bool = false;
        let mut has_blue_mana: bool = false;
        let mut has_black_mana: bool = false;
        let mut has_red_mana: bool = false;
        let mut has_green_mana: bool = false;
        for html_card in html_cards {
            if html_card.contains("COLOR_WHITE") {
                has_white_mana = true;
            }
            if html_card.contains("COLOR_BLUE") {
                has_blue_mana = true;
            }
            if html_card.contains("COLOR_BLACK") {
                has_black_mana = true;
            }
            if html_card.contains("COLOR_RED") {
                has_red_mana = true;
            }
            if html_card.contains("COLOR_GREEN") {
                has_green_mana = true;
            }
        }
        has_white_data.push(has_white_mana);
        has_blue_data.push(has_blue_mana);
        has_black_data.push(has_black_mana);
        has_red_data.push(has_red_mana);
        has_green_data.push(has_green_mana);
        lands_count.push(0);
    }
    let mut unique_lands_values: Vec<u8> = Vec::new();
    for land_amount in &lands_count {
        let mut contains_value: bool = false;
        for unique_value in &unique_lands_values {
            if *land_amount == *unique_value {
                contains_value = true;
                break;
            }
        }
        if !contains_value {
            unique_lands_values.push(*land_amount);
        }
    }
    println!("Unique lands values: {:?}\nW: {:?}\nU: {:?}\nB: {:?}\nR: {:?}\nG: {:?}", unique_lands_values, has_white_data, has_blue_data, has_black_data, has_red_data, has_green_data);
    let mut lands_count_data: Vec<Vec<bool>> = Vec::new();
    for unique_value in unique_lands_values {
        let mut unique_value_data: Vec<bool> = Vec::new();
        for land_amount in &lands_count {
            unique_value_data.push(*land_amount < unique_value);
        }
        lands_count_data.push(unique_value_data);
    }
    let mut data_array_map: Vec<Vec<bool>> = Vec::from([
        has_white_data,
        has_blue_data,
        has_black_data,
        has_red_data,
        has_green_data,
    ]);
    data_array_map.append(&mut lands_count_data);
    assert_eq!(deck_position_less_than_9_data.len(), html_decks.len());
    let generated_nodes: Node = generate_nodes(
        &Vec::from_iter(0..deck_position_less_than_9_data.len()),
        &data_array_map,
        &Vec::from(deck_position_less_than_9_data)
    );
    println!("Node: {:?}", generated_nodes);
    println!("0:W, 1:U, 2:B, 3:R, 4:G");
    
    let prediction: bool = evaluate_data(
        generated_nodes,
        Vec::from([
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            true,
            false,
        ])
    );
    println!("The prediction for data is: {:?}", prediction);
    }
    }).unwrap().join().unwrap();
}
