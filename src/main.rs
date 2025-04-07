/*
https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694
|ID|LANDS|WHITE|BLUE|BLACK|RED|GREEN|POSITION|
|--|-----|-----|----|-----|---|-----|--------|
|0 |20   |no   |no  |yes  |yes|yes  |1       |
|1 |17   |no   |yes |no   |no |no   |2       |
|2 |19   |yes  |no  |no   |no |no   |3       |
|3 |19   |no   |yes |yes  |yes|no   |4       |
|4 |18   |no   |yes |no   |yes|no   |5       |
|5 |19   |no   |yes |yes  |no |no   |6       |
|6 |20   |no   |yes |yes  |yes|no   |7       |
|7 |18   |no   |yes |yes  |no |no   |8       |
|8 |19   |yes  |no  |yes  |no |yes  |9       |
|9 |14   |no   |yes |yes  |yes|yes  |10      |
|10|18   |no   |yes |no   |no |no   |11      |
|11|17   |no   |no  |no   |yes|no   |12      |
|12|18   |no   |yes |yes  |no |no   |13      |
|13|17   |yes  |no  |no   |no |no   |14      |
|14|20   |no   |no  |yes  |yes|yes  |15      |
|15|18   |no   |yes |no   |no |no   |16      |
|16|16   |no   |yes |yes  |yes|yes  |17      |
|17|18   |no   |yes |yes  |no |no   |18      |
|18|17   |no   |no  |no   |yes|no   |19      |
|19|15   |no   |yes |no   |no |no   |20      |
|20|18   |no   |yes |yes  |no |no   |21      |
|21|17   |no   |no  |no   |yes|yes  |22      |
|22|17   |yes  |no  |no   |no |yes  |23      |
|23|17   |no   |no  |yes  |no |yes  |24      |
|24|20   |no   |no  |yes  |yes|yes  |25      |
|25|16   |no   |yes |yes  |yes|yes  |26      |
|26|18   |no   |yes |yes  |no |no   |27      |
|27|13   |no   |no  |no   |yes|yes  |28      |
|28|17   |no   |no  |no   |no |yes  |29      |
|29|17   |no   |no  |no   |yes|yes  |30      |
|30|19   |no   |yes |yes  |yes|no   |31      |
|31|15   |no   |no  |yes  |yes|yes  |32      |
 */

use reqwest;

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
    if smaller_gini == node_gini_impurity {
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
    let mut page_html: &str = "";
    let client: reqwest::blocking::Client = match reqwest::blocking::ClientBuilder::new().build() {
        Ok(client_settings) => client_settings,
        _ => panic!("Client settings inccorrect"),
    };
    let response: Result<reqwest::blocking::Response, reqwest::Error> = client
.get("https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694")
        .header("Host", "mtgdecks.net")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .header("Accetp", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-Us,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Sec-GPC", "1")
        .header("Connection", "keep-alive")
        .header("Cookie", "cf_clearance=FlMz9EUIaWL7JWS893cDr8cwHY1xQ2165agwfpE1ofQ-1743974560-1.2.1.1-__2Ftpa28IPnHbrjZC4xpUufP9i4_KCw.JZCrAoHIusVM65PS4WskZ_EOUFjfBhjXP1yy_NxvHqGfzxHV_7ko7RadoKn7HX.OgUXb_mnXLcCcUeFs9RptsTJL8LbGS.T6NEggZfbC7gdFPZfErdCck3RENTspnR.a6xLw1O.UDtnb.1quYs5mdNGqMCswDV.a7kW3ZcvTBAKWOgm.qWss2TrvzXpDGP.5mA8LP_ShNO0bMHs94a09P9OQT5e4LvZwocm9cBruoYvu55jLTaFDE0G1sudiyJRg2GzrwKD7ek0Net9_xiaX6w6VCiqCeyiQyLR9o40C4TnQqyAs5nmm4hQHqI5c1ApN6.ss_jsPs8; PHPSESSID=h7q2cidav2jtk3m2npi8jo1pr8")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "cross-site")
        .header("Priority", "u=0, i")
        .header("TE", "trailers")
        .send();
    let text;
    if let Ok(resp) = response {
        println!("Here");
        text = resp.text().expect("failed");
        println!("{:?}", text);
        page_html = &text;
    }
    if let (Some(first_index), Some(second_index)) = (page_html.find(
        r#"<th class="hidden-xs">Archetype</th>
<th class="hidden-xs">Spiciness</th>"#
    ),
    page_html.find(
        r#"<h2 id="event-archetypes">Tournament Archetype breakdown</h2>"#
    )) {
        page_html = &page_html[first_index..second_index];
    }
    let html_decks: Vec<&str> =
        page_html.split(r#"<tr style="" "#).collect();
    let mut has_white_data: Vec<bool> = Vec::new();
    let mut has_blue_data: Vec<bool> = Vec::new();
    let mut has_black_data: Vec<bool> = Vec::new();
    let mut has_red_data: Vec<bool> = Vec::new();
    let mut has_green_data: Vec<bool> = Vec::new();
    let mut lands_count: Vec<u8> = Vec::new();
    let mut deck_position_less_than_9_data: Vec<bool> = Vec::new();
    for html_deck_index in 1..html_decks.len() {
        deck_position_less_than_9_data.push(html_deck_index < 9);
        let html_mana_symbols: Vec<&str> =
            html_decks[html_deck_index]
            .split(r#"<span class="ms ms-cost ms-"#).collect();
        let mut has_white_mana: bool = false;
        let mut has_blue_mana: bool = false;
        let mut has_black_mana: bool = false;
        let mut has_red_mana: bool = false;
        let mut has_green_mana: bool = false;
        for html_mana_symbol in html_mana_symbols {
            if let Some(char) = html_mana_symbol.chars().nth(0) {
                if char == 'w' {
                    has_white_mana = true;
                } else if char == 'u' {
                    has_blue_mana = true;
                } else if char == 'b' {
                    has_black_mana = true;
                } else if char == 'r' {
                    has_red_mana = true;
                } else if char == 'g' {
                    has_green_mana = true;
                }
            }
        }
        has_white_data.push(has_white_mana);
        has_blue_data.push(has_blue_mana);
        has_black_data.push(has_black_mana);
        has_red_data.push(has_red_mana);
        has_green_data.push(has_green_mana);
        lands_count.push(0);
    }
    /*
    let lands_count: Vec<u8> = Vec::from([
        20, 17, 19, 19, 18, 19, 20, 18, 19, 14,
        18, 17, 18, 17, 20, 18, 16, 18, 17, 15,
        18, 17, 17, 17, 20, 16, 18, 13, 17, 17,
        19, 15,
    ]);
    */
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
    println!("Unique lands values: {:?}", unique_lands_values);
    let mut lands_count_data: Vec<Vec<bool>> = Vec::new();
    for unique_value in unique_lands_values {
        let mut unique_value_data: Vec<bool> = Vec::new();
        for land_amount in &lands_count {
            unique_value_data.push(*land_amount < unique_value);
        }
        lands_count_data.push(unique_value_data);
    }
    let deck_position_less_than_8_data: Vec<bool> = Vec::from([
        true, true, true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false,
    ]);
    let mut data_array_map: Vec<Vec<bool>> = Vec::from([
        has_white_data,
        has_blue_data,
        has_black_data,
        has_red_data,
        has_green_data,
    ]);
    data_array_map.append(&mut lands_count_data);
    let generated_nodes: Node = generate_nodes(
        &Vec::from_iter(0..deck_position_less_than_8_data.len()),
        &data_array_map,
        &Vec::from(deck_position_less_than_8_data)
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
