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
    let mut page_html: &str = r##"<!DOCTYPE html>
<html lang="en-US" prefix="og: http://ogp.me/ns# fb: http://ogp.me/ns/fb# mtgdecks: http://ogp.me/ns/fb/mtgdecks#">
<head>
<!-- Raptive Head Tag Manual -->
<script data-no-optimize="1" data-cfasync="false" pagespeed_no_defer="" data-pagespeed-no-defer="">(function(w,d){w.adthrive=w.adthrive||{};w.adthrive.cmd=w.adthrive.cmd||[];w.adthrive.plugin='adthrive-ads-manual';w.adthrive.host='ads.adthrive.com';var s=d.createElement('script');s.async=true;s.referrerpolicy='no-referrer-when-downgrade';s.src='https://'+w.adthrive.host+'/sites/66df087a6a7bf3530bc75e20/ads.min.js?referrer='+w.encodeURIComponent(w.location.href)+'&cb='+(Math.floor(Math.random()*100)+1);var n=d.getElementsByTagName('script')[0];n.parentNode.insertBefore(s,n);})(window,document);</script>
<!-- End of Raptive Head Tag -->
<meta name="robots" content="max-snippet:-1">
<meta name="robots" content="max-video-preview:-1">
<meta name="robots" content="max-image-preview:large">
<script data-pagespeed-orig-type="text/javascript" type="text/psajs" data-pagespeed-orig-index="0">window.base_url='https://mtgdecks.net/';</script>
<meta name="description" content="Top 32 Pauper Decks from  Tropical Pauper 227  on 2025-03-22. Winner of the event: kakotibs_Pato playing Jundão Mol"/>
<meta property="og:type" content="article">
<meta property="og:site_name" content="MTG Decks">
<meta property="og:title" content=" Tropical Pauper 227  Pauper Decks">
<meta property="og:url" content="https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694">
<meta property="og:image" content="https://static.mtgdecks.net/decks/2419621.jpg">
<meta property="og:image:secure_url" content="https://static.mtgdecks.net/decks/2419621.jpg">
<meta property="og:image:url" content="https://static.mtgdecks.net/decks/2419621.jpg">
<meta property="og:image:secure" content="https://static.mtgdecks.net/decks/2419621.jpg">
<meta property="og:description" content="Top 32 Pauper Decks from  Tropical Pauper 227  on 2025-03-22. Winner of the event: kakotibs_Pato playing Jundão Mol">
<meta property="article:author" content="MTG Decks">
<meta property="article:published_time" content="2025-03-22 19:00:34">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:site" content="mtgdecks.net">
<meta name="twitter:title" content=" Tropical Pauper 227  Pauper Decks">
<meta name="twitter:description" content="Top 32 Pauper Decks from  Tropical Pauper 227  on 2025-03-22. Winner of the event: kakotibs_Pato playing Jundão Mol">
<meta name="twitter:image" content="https://static.mtgdecks.net/decks/2419621.jpg">
<style>@font-face{font-display:swap;font-family:'Roboto';font-style:normal;font-weight:300;src:url(https://mtgdecks.net/fonts/roboto-v30-latin-300.woff2) format('woff2') , url(https://mtgdecks.net/fonts/roboto-v30-latin-300.woff) format('woff')}@font-face{font-display:swap;font-family:'Roboto';font-style:normal;font-weight:400;src:url(https://mtgdecks.net/fonts/roboto-v30-latin-regular.woff2) format('woff2') , url(https://mtgdecks.net/fonts/roboto-v30-latin-regular.woff) format('woff')}@font-face{font-display:swap;font-family:'Roboto';font-style:normal;font-weight:700;src:url(https://mtgdecks.net/fonts/roboto-v30-latin-700.woff2) format('woff2') , url(https://mtgdecks.net/fonts/roboto-v30-latin-700.woff) format('woff')}</style>
<link rel="preload" as="font" crossorigin="anonymous" href="https://mtgdecks.net/fonts/glyphicons-halflings-regular.woff">
<link rel="preload" as="font" crossorigin="anonymous" href="https://mtgdecks.net/fonts/mana.woff?v=1.15.9">
<!-- Chrome, Firefox OS and Opera -->
<meta name="theme-color" content="#212b34">
<!-- Windows Phone -->
<meta name="msapplication-navbutton-color" content="#212b34">
<!-- iOS Safari -->
<meta name="apple-mobile-web-app-status-bar-style" content="#212b34">
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
<title> Tropical Pauper 227 | March 22 • MTG DECKS </title>
<link rel="icon" href="/favicons/xapple-icon-57x57.png.pagespeed.ic.zcx3ImtG2J.png" type="image/x-icon"/>
<link rel="shortcut icon" href="/favicons/xapple-icon-57x57.png.pagespeed.ic.zcx3ImtG2J.png" type="image/x-icon"/>
<link rel="apple-touch-icon" sizes="57x57" href="/favicons/xapple-icon-57x57.png.pagespeed.ic.zcx3ImtG2J.png">
<link rel="apple-touch-icon" sizes="60x60" href="/favicons/xapple-icon-60x60.png.pagespeed.ic.8VtRVIoiKb.png">
<link rel="apple-touch-icon" sizes="72x72" href="/favicons/xapple-icon-72x72.png.pagespeed.ic.sLiBEVfMS_.png">
<link rel="apple-touch-icon" sizes="76x76" href="/favicons/xapple-icon-76x76.png.pagespeed.ic.ygF7IlavDN.png">
<link rel="apple-touch-icon" sizes="114x114" href="/favicons/xapple-icon-114x114.png.pagespeed.ic.eADtdLP4Wn.png">
<link rel="apple-touch-icon" sizes="120x120" href="/favicons/xapple-icon-120x120.png.pagespeed.ic.d46W0uUWit.png">
<link rel="apple-touch-icon" sizes="144x144" href="/favicons/xapple-icon-144x144.png.pagespeed.ic.UEq0OnqaYE.png">
<link rel="apple-touch-icon" sizes="152x152" href="/favicons/xapple-icon-152x152.png.pagespeed.ic.2G-Y1GODkq.png">
<link rel="apple-touch-icon" sizes="180x180" href="/favicons/xapple-icon-180x180.png.pagespeed.ic.6rRNRZfd76.png">
<link rel="icon" type="image/png" sizes="192x192" href="/favicons/xandroid-icon-192x192.png.pagespeed.ic.oqYDIOiUDZ.png">
<link rel="icon" type="image/png" sizes="32x32" href="/favicons/xfavicon-32x32.png.pagespeed.ic.CFSGMOBLO3.png">
<link rel="icon" type="image/png" sizes="96x96" href="/favicons/xfavicon-96x96.png.pagespeed.ic.TA0foAQ4hH.png">
<link rel="icon" type="image/png" sizes="16x16" href="/favicons/xfavicon-16x16.png.pagespeed.ic.n0f5JKPEKl.png">
<meta name="msapplication-TileColor" content="#ffffff">
<meta name="msapplication-TileImage" content="/favicons/ms-icon-144x144.png">
<meta name="theme-color" content="#ffffff">
<link rel="alternate" type="application/rss+xml" title="MTG Decks Tournaments" href="https://mtgdecks.net/events/index.rss?256429175"/>
<link rel="alternate" type="application/rss+xml" title="MTG Decks latest news and articles" href="https://mtgdecks.net/articles/feed.rss"/>
<link rel="manifest" href="/manifest.json">
<script type="text/psajs" data-pagespeed-orig-index="1">if('serviceWorker'in navigator){window.addEventListener('load',()=>{navigator.serviceWorker.register('/sw.js');});}</script>
<meta name="clckd" content="7aa7f89ce7493ec72f57ab11a3c5f9be"/>
<script type="application/ld+json">
		{
			"@id": "https://mtgdecks.net/#website",
			"@context": "http://schema.org",
			"@type": "WebSite",
			"name": "MTG DECKS",
			"alternateName": "Magic the Gathering Top Decks Database",
			"url": "https://mtgdecks.net",
			"publisher": {
				"@id": "https://mtgdecks.net/#organization"
			},
			"isAccessibleForFree": true
		}
	</script>
<script type="application/ld+json">
		{
			"@id": "https://mtgdecks.net/#organization",
			"@context": "http://schema.org",
			"@type": "Organization",
			"name": "MTG DECKS",
			"description": "MTG Decks is focused on compiling the top Magic the Gathering decks around the world and make them easily accessible.",
			"url": "https://mtgdecks.net",
			"email": "admin@mtgdecks.net",
			"logo": "https://mtgdecks.net/img/logoSmall2.svg",
			"foundingDate": "2010-12-10",
			"sameAs": [
				"https://www.facebook.com/mtgdecks",
				"https://twitter.com/mtgdecks",
				"https://plus.google.com/+mtgdecksnet",
				"https://www.wikidata.org/wiki/Q104518471"
			],
			"contactPoint": [{
				"@type": "ContactPoint",
				"contactType": "Site owner",
				"email": "admin@mtgdecks.net",
				"url": "https://mtgdecks.net"
			}]
		}
	</script>
<script async src="https://cdn.jsdelivr.net/npm/js-cookie@rc/dist/js.cookie.min.js" type="text/psajs" data-pagespeed-orig-index="2"></script>
<link rel="canonical" href="https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694">
<meta property="fb:pages" content="161079100614009"/>
<meta name="viewport" content="width=device-width,height=device-height, initial-scale=1, user-scalable=yes"/>
<meta property="fb:app_id" content="175905932426460"/>
<meta property="fb:admins" content="1344195300"/>
<script data-pagespeed-orig-type="text/javascript" src="/js/jquery-1.11.1.min.js.pagespeed.jm.YSzgc-BSX9.js" type="text/psajs" data-pagespeed-orig-index="3"></script>
<script data-pagespeed-orig-type="text/javascript" type="text/psajs" data-pagespeed-orig-index="4">$(function(){$('.clickable td').css('cursor','pointer');$("table.clickable").on('click','a',function(e){e.stopPropagation();});$(".clickable").on('click','td',function(){url=$(this).closest("tr").find("a").attr('href');if(url!=undefined){window.location.href=url;}else{url=$(this).closest("tr").attr('url');if(url!=undefined){mtgdecks(url);}else{base64url=$(this).closest("tr").find('.linker').attr('goto');if(typeof base64url==='string'||base64url instanceof String){window.location.href=base64.decode(base64url);}}}});});</script>
<link rel="stylesheet" type="text/css" href="/css/A.bootstrap.min.css+bootstrap-theme.min.css+custom.css+dark.css,Mcc.TOo9BgmfO4.css.pagespeed.cf.IBg4og-zgV.css"/>
<style type="text/css">.menu-header{font-size:14px;text-transform:uppercase;padding:5px 10px;color:#fff}</style><style>.fixed-table-container .bs-checkbox,.fixed-table-container .no-records-found{text-align:center}.fixed-table-body thead th .th-inner,.table td,.table th{box-sizing:border-box}.bootstrap-table .table{margin-bottom:0!important;border-bottom:1px solid #ddd;border-collapse:collapse!important;border-radius:1px}.bootstrap-table .table:not(.table-condensed),.bootstrap-table .table:not(.table-condensed)>tbody>tr>td,.bootstrap-table .table:not(.table-condensed)>tbody>tr>th,.bootstrap-table .table:not(.table-condensed)>tfoot>tr>td,.bootstrap-table .table:not(.table-condensed)>tfoot>tr>th,.bootstrap-table .table:not(.table-condensed)>thead>tr>td{padding:8px}.bootstrap-table .table.table-no-bordered>tbody>tr>td,.bootstrap-table .table.table-no-bordered>thead>tr>th{border-right:2px solid transparent}.bootstrap-table .table.table-no-bordered>tbody>tr>td:last-child{border-right:none}.fixed-table-container{position:relative;clear:both;border:1px solid #ddd;border-radius:4px;-webkit-border-radius:4px;-moz-border-radius:4px}.fixed-table-container.table-no-bordered{border:1px solid transparent}.fixed-table-footer,.fixed-table-header{overflow:hidden}.fixed-table-footer{border-top:1px solid #ddd}.fixed-table-body{overflow-x:auto;overflow-y:auto;height:100%}.fixed-table-container table{width:100%}.fixed-table-container thead th{height:0;padding:0;margin:0;border-left:1px solid #ddd}.fixed-table-container thead th:focus{outline:transparent solid 0}.fixed-table-container thead th:first-child{border-left:none;border-top-left-radius:4px;-webkit-border-top-left-radius:4px;-moz-border-radius-topleft:4px}.fixed-table-container tbody td .th-inner,.fixed-table-container thead th .th-inner{padding:8px;line-height:24px;vertical-align:top;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.fixed-table-container thead th .sortable{cursor:pointer;background-position:right;background-repeat:no-repeat;padding-right:30px}.fixed-table-container thead th .both{background-image:url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABMAAAATCAQAAADYWf5HAAAAkElEQVQoz7X QMQ5AQBCF4dWQSJxC5wwax1Cq1e7BAdxD5SL+Tq/QCM1oNiJidwox0355mXnG/DrEtIQ6azioNZQxI0ykPhTQIwhCR+BmBYtlK7kLJYwWCcJA9M4qdrZrd8pPjZWPtOqdRQy320YSV17OatFC4euts6z39GYMKRPCTKY9UnPQ6P+GtMRfGtPnBCiqhAeJPmkqAAAAAElFTkSuQmCC')}.fixed-table-container thead th .asc{background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABMAAAATCAYAAAByUDbMAAAAZ0lEQVQ4y2NgGLKgquEuFxBPAGI2ahhWCsS/gDibUoO0gPgxEP8H4ttArEyuQYxAPBdqEAxPBImTY5gjEL9DM+wTENuQahAvEO9DMwiGdwAxOymGJQLxTyD+jgWDxCMZRsEoGAVoAADeemwtPcZI2wAAAABJRU5ErkJggg==)}.fixed-table-container thead th .desc{background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABMAAAATCAYAAAByUDbMAAAAZUlEQVQ4y2NgGAWjYBSggaqGu5FA/BOIv2PBIPFEUgxjB+IdQPwfC94HxLykus4GiD+hGfQOiB3J8SojEE9EM2wuSJzcsFMG4ttQgx4DsRalkZENxL+AuJQaMcsGxBOAmGvopk8AVz1sLZgg0bsAAAAASUVORK5CYII=)}.fixed-table-container th.detail{width:30px}.fixed-table-container tbody td{border-left:1px solid #ddd}.fixed-table-container tbody tr:first-child td{border-top:none}.fixed-table-container tbody td:first-child{border-left:none}.fixed-table-container tbody .selected td{background-color:#f5f5f5}.fixed-table-container .bs-checkbox .th-inner{padding:8px 0}.fixed-table-container input[type=radio],.fixed-table-container input[type=checkbox]{margin:0 auto!important}.fixed-table-pagination .pagination-detail,.fixed-table-pagination div.pagination{margin-top:10px;margin-bottom:10px}.fixed-table-pagination div.pagination .pagination{margin:0}.fixed-table-pagination .pagination a{padding:6px 12px;line-height:1.428571429}.fixed-table-pagination .pagination-info{line-height:34px;margin-right:5px}.fixed-table-pagination .btn-group{position:relative;display:inline-block;vertical-align:middle}.fixed-table-pagination .dropup .dropdown-menu{margin-bottom:0}.fixed-table-pagination .page-list{display:inline-block}.fixed-table-toolbar .columns-left{margin-right:5px}.fixed-table-toolbar .columns-right{margin-left:5px}.fixed-table-toolbar .columns label{display:block;padding:3px 20px;clear:both;font-weight:400;line-height:1.428571429}.fixed-table-toolbar .bs-bars,.fixed-table-toolbar .columns,.fixed-table-toolbar .search{position:relative;margin-top:10px;margin-bottom:10px;line-height:34px}.fixed-table-pagination li.disabled a{pointer-events:none;cursor:default}.fixed-table-loading{display:none;position:absolute;top:42px;right:0;bottom:0;left:0;z-index:99;background-color:#fff;text-align:center}.fixed-table-body .card-view .title{font-weight:700;display:inline-block;min-width:30%;text-align:left!important}.table td,.table th{vertical-align:middle}.fixed-table-toolbar .dropdown-menu{text-align:left;max-height:300px;overflow:auto}.fixed-table-toolbar .btn-group>.btn-group{display:inline-block;margin-left:-1px!important}.fixed-table-toolbar .btn-group>.btn-group>.btn{border-radius:0}.fixed-table-toolbar .btn-group>.btn-group:first-child>.btn{border-top-left-radius:4px;border-bottom-left-radius:4px}.fixed-table-toolbar .btn-group>.btn-group:last-child>.btn{border-top-right-radius:4px;border-bottom-right-radius:4px}.bootstrap-table .table>thead>tr>th{vertical-align:bottom;border-bottom:1px solid #ddd}.bootstrap-table .table thead>tr>th{padding:0;margin:0}.bootstrap-table .fixed-table-footer tbody>tr>td{padding:0!important}.bootstrap-table .fixed-table-footer .table{border-bottom:none;border-radius:0;padding:0!important}.bootstrap-table .pull-right .dropdown-menu{right:0;left:auto}p.fixed-table-scroll-inner{width:100%;height:200px}div.fixed-table-scroll-outer{top:0;left:0;visibility:hidden;width:200px;height:150px;overflow:hidden}.fixed-table-pagination:after,.fixed-table-toolbar:after{content:"";display:block;clear:both}
span.multiselect-native-select{position:relative}span.multiselect-native-select select{border:0!important;clip:rect(0 0 0 0)!important;height:1px!important;margin:-1px -1px -1px -3px!important;overflow:hidden!important;padding:0!important;position:absolute!important;width:1px!important;left:50%;top:30px}.multiselect-container{position:absolute;list-style-type:none;margin:0;padding:0}.multiselect-container .input-group{margin:5px}.multiselect-container>li{padding:0}.multiselect-container>li>a.multiselect-all label{font-weight:700}.multiselect-container>li.multiselect-group label{margin:0;padding:3px 20px 3px 20px;height:100%;font-weight:700}.multiselect-container>li.multiselect-group-clickable label{cursor:pointer}.multiselect-container>li>a{padding:0}.multiselect-container>li>a>label{margin:0;height:100%;cursor:pointer;font-weight:400;padding:3px 20px 3px 40px}.multiselect-container>li>a>label.radio,.multiselect-container>li>a>label.checkbox{margin:0}.multiselect-container>li>a>label>input[type=checkbox]{margin-bottom:5px}.btn-group>.btn-group:nth-child(2)>.multiselect.btn{border-top-left-radius:4px;border-bottom-left-radius:4px}.form-inline .multiselect-container label.checkbox,.form-inline .multiselect-container label.radio{padding:3px 20px 3px 40px}.form-inline .multiselect-container li a label.checkbox input[type=checkbox],.form-inline .multiselect-container li a label.radio input[type=radio]{margin-left:-20px;margin-right:0}
</style><link rel="dns-prefetch" href="//images.dmca.com"><link rel="dns-prefetch" href="//www.googletagmanager.com"><link rel="dns-prefetch" href="//static.cloudflareinsights.com"><link rel="dns-prefetch" href="//btloader.com"></head>
<body class="">
<div id="alert-area" up-hungry up-if-layer="any" style="z-index: 99999999!important;">
</div>
<div class="container container-fluid" style="padding:0;overflow:hidden" id="main-wrapper">
<div id="app">
<div id="content">
<script type="application/ld+json">
	{
		"@context": "http://schema.org",
		"@type": "BreadcrumbList",
		"itemListElement": [{
			"@type": "ListItem",
			"position": 1,
			"item": {
				"@id": "https://mtgdecks.net/Pauper",
				"name": "Pauper decks"
			}
		}, {
			"@type": "ListItem",
			"position": 2,
			"item": {
				"@id": "https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694",
				"name": " Tropical Pauper 227  decks"
			}
		}]
	}
</script>
<script type="application/ld+json">
	{
		"@context": "https://schema.org",
		"@type": "NewsArticle",
		"mainEntityOfPage": {
			"@type": "WebPage",
			"@id": "https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694"
		},
		"headline": " Tropical Pauper 227  decks",

		
		"datePublished": "2025-03-22T19:00:34+01:00",
		"dateModified": "2025-03-22T19:00:34+01:00",
		"author": {
			"@type": "Person",
			"name": "MTG Decks"
		},
		"publisher": {
			"@type": "Organization",
			"name": "MTG decks",
			"logo": {
				"@type": "ImageObject",
				"url": "https://mtgdecks.net/img/logo.png"
			}
		},
		 "description": "Top 32 Pauper Decks from  Tropical Pauper 227  on 2025-03-22. Winner of the event: kakotibs_Pato playing Jundão Mol"
			}
</script>
<div class="heading">
<div class="breadcrumbs pull-left">
<a href="/">Home</a>	<span class="glyphicon glyphicon-chevron-right"></span>
<a href="/Pauper">MTG Pauper</a>	<span class="glyphicon glyphicon-chevron-right"></span>
<a href="/Pauper/tournaments">Tournaments</a>	<span class="glyphicon glyphicon-chevron-right"></span>
</div>
<div class="omniSwitch pull-right hidden-xs">
<span class="small" style="display: inline-block;margin-top: 3px;">SHOW PRICES FOR&nbsp;</span>
<div class="btn-group navbar-right priceSwitcher" role="group">
<div id="onlineButton" data-format="mtgo" class="btn btn-xs btn-default">
<script data-pagespeed-no-defer>//<![CDATA[
(function(){for(var g="function"==typeof Object.defineProperties?Object.defineProperty:function(b,c,a){if(a.get||a.set)throw new TypeError("ES3 does not support getters and setters.");b!=Array.prototype&&b!=Object.prototype&&(b[c]=a.value)},h="undefined"!=typeof window&&window===this?this:"undefined"!=typeof global&&null!=global?global:this,k=["String","prototype","repeat"],l=0;l<k.length-1;l++){var m=k[l];m in h||(h[m]={});h=h[m]}var n=k[k.length-1],p=h[n],q=p?p:function(b){var c;if(null==this)throw new TypeError("The 'this' value for String.prototype.repeat must not be null or undefined");c=this+"";if(0>b||1342177279<b)throw new RangeError("Invalid count value");b|=0;for(var a="";b;)if(b&1&&(a+=c),b>>>=1)c+=c;return a};q!=p&&null!=q&&g(h,n,{configurable:!0,writable:!0,value:q});var t=this;function u(b,c){var a=b.split("."),d=t;a[0]in d||!d.execScript||d.execScript("var "+a[0]);for(var e;a.length&&(e=a.shift());)a.length||void 0===c?d[e]?d=d[e]:d=d[e]={}:d[e]=c};function v(b){var c=b.length;if(0<c){for(var a=Array(c),d=0;d<c;d++)a[d]=b[d];return a}return[]};function w(b){var c=window;if(c.addEventListener)c.addEventListener("load",b,!1);else if(c.attachEvent)c.attachEvent("onload",b);else{var a=c.onload;c.onload=function(){b.call(this);a&&a.call(this)}}};var x;function y(b,c,a,d,e){this.h=b;this.j=c;this.l=a;this.f=e;this.g={height:window.innerHeight||document.documentElement.clientHeight||document.body.clientHeight,width:window.innerWidth||document.documentElement.clientWidth||document.body.clientWidth};this.i=d;this.b={};this.a=[];this.c={}}function z(b,c){var a,d,e=c.getAttribute("data-pagespeed-url-hash");if(a=e&&!(e in b.c))if(0>=c.offsetWidth&&0>=c.offsetHeight)a=!1;else{d=c.getBoundingClientRect();var f=document.body;a=d.top+("pageYOffset"in window?window.pageYOffset:(document.documentElement||f.parentNode||f).scrollTop);d=d.left+("pageXOffset"in window?window.pageXOffset:(document.documentElement||f.parentNode||f).scrollLeft);f=a.toString()+","+d;b.b.hasOwnProperty(f)?a=!1:(b.b[f]=!0,a=a<=b.g.height&&d<=b.g.width)}a&&(b.a.push(e),b.c[e]=!0)}y.prototype.checkImageForCriticality=function(b){b.getBoundingClientRect&&z(this,b)};u("pagespeed.CriticalImages.checkImageForCriticality",function(b){x.checkImageForCriticality(b)});u("pagespeed.CriticalImages.checkCriticalImages",function(){A(x)});function A(b){b.b={};for(var c=["IMG","INPUT"],a=[],d=0;d<c.length;++d)a=a.concat(v(document.getElementsByTagName(c[d])));if(a.length&&a[0].getBoundingClientRect){for(d=0;c=a[d];++d)z(b,c);a="oh="+b.l;b.f&&(a+="&n="+b.f);if(c=!!b.a.length)for(a+="&ci="+encodeURIComponent(b.a[0]),d=1;d<b.a.length;++d){var e=","+encodeURIComponent(b.a[d]);131072>=a.length+e.length&&(a+=e)}b.i&&(e="&rd="+encodeURIComponent(JSON.stringify(B())),131072>=a.length+e.length&&(a+=e),c=!0);C=a;if(c){d=b.h;b=b.j;var f;if(window.XMLHttpRequest)f=new XMLHttpRequest;else if(window.ActiveXObject)try{f=new ActiveXObject("Msxml2.XMLHTTP")}catch(r){try{f=new ActiveXObject("Microsoft.XMLHTTP")}catch(D){}}f&&(f.open("POST",d+(-1==d.indexOf("?")?"?":"&")+"url="+encodeURIComponent(b)),f.setRequestHeader("Content-Type","application/x-www-form-urlencoded"),f.send(a))}}}function B(){var b={},c;c=document.getElementsByTagName("IMG");if(!c.length)return{};var a=c[0];if(!("naturalWidth"in a&&"naturalHeight"in a))return{};for(var d=0;a=c[d];++d){var e=a.getAttribute("data-pagespeed-url-hash");e&&(!(e in b)&&0<a.width&&0<a.height&&0<a.naturalWidth&&0<a.naturalHeight||e in b&&a.width>=b[e].o&&a.height>=b[e].m)&&(b[e]={rw:a.width,rh:a.height,ow:a.naturalWidth,oh:a.naturalHeight})}return b}var C="";u("pagespeed.CriticalImages.getBeaconData",function(){return C});u("pagespeed.CriticalImages.Run",function(b,c,a,d,e,f){var r=new y(b,c,a,e,f);x=r;d&&w(function(){window.setTimeout(function(){A(r)},0)})});})();pagespeed.CriticalImages.Run('/mod_pagespeed_beacon','https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694','cLFF33dPjd',true,false,'imxV3aCeuWA');
//]]></script><img src="/img/icons/xmtgo_black.png.pagespeed.ic.H3srSg8h7S.png" height="12px;" class="whitened" alt="" data-pagespeed-url-hash="1373742378" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	MTGO
</div>
<div id="mtgaButton" data-format="arena" class="btn btn-xs btn-default">
<img src="/img/icons/xarena.png.pagespeed.ic.LsuMbI_Aty.png" height="12px;" class="whitened" alt="" data-pagespeed-url-hash="1152051722" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	ARENA
</div>
<div id="paperButton" data-format="paper" class="btn btn-xs btn-primary">
<img src="/img/icons/xmtg.png.pagespeed.ic.S2FSPZPuOM.png" height="12px;" class="whitened" alt="" data-pagespeed-url-hash="62060803" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	PAPER
</div>
</div>
</div>
</div>
<div class="mon-placement loading-mon " style="width:100%;color:white; "><div id="placement-header-desktop" class="raptive-placeholder-header raptive-placement hidden-xs" style="min-height:90px;max-height:90px;overflow:hidden;"></div></div>	<div class="mon-placement loading-mon " style="width:100%;color:white; "><div id="placement-header-mobile" class="raptive-placeholder-header raptive-placement visible-xs" style="min-height:250px;max-height:250px;overflow:hidden;width:100%;"></div></div>
<ul class="nav nav-pills text-uppercase" style="height:37px;max-width: 100%;overflow: hidden;overflow-x: auto;white-space: nowrap;display: flex;flex-wrap: nowrap;">
<li role="presentation" class="">
<a href="/Pauper"> <span class="hidden-xs"><i class="glyphicon glyphicon-signal"></i> Pauper</span> <span class="hidden-xs">meta</span> <span class="visible-xs">Meta</span></a>
</li>
<li role="presentation" class="active">
<a href="/Pauper/tournaments"><span class="hidden-xs"><i class="glyphicon glyphicon-list-alt"></i> Pauper </span> <span class="hidden-xs">Tournaments</span><span class="visible-xs">Events</span></a>
</li>
<li style="cursor: pointer" role="presentation" class=" visible-xs">
<span><span goto="L1BhdXBlci9kZWNrbGlzdHM" class="linker">Decks</span></span> </li>
<li style="cursor: pointer" role="presentation" class=" hidden-xs">
<span><span goto="L1BhdXBlci9kZWNrbGlzdHM" class="linker"><i class="glyphicon glyphicon-th-large"></i> Pauper Decklists</span></span> </li>
<li role="presentation" class="">
<a href="/Pauper/staples"><span class="hidden-xs"><i class="fa fa-tags"></i> Pauper </span> Staples</a>
</li>
<li role="presentation" class="">
<a href="/Pauper/winrates"><span class="hidden-xs"> <i class="fa fa-percent"></i> </span> Winrates</a>
</li>
<!--              
      <li role="presentation" class="">
        <span><span goto="L1BhdXBlci9jcmFmdA" class="linker"><span class="hidden-xs"><i class="ms ms-rarity"></i> </span> Craft <span class="hidden-xs">Assistant</span></span></span>      </li>
      
                  -->
<!--
      <li role="presentation" class="">
        <a href="/Pauper/lands-guide"><span class="hidden-xs"><i class="ms ms-land"></i> Pauper </span>  Lands <span class="hidden-xs">Guide</span></a>
      </li>

      <li role="presentation" class="">
        <a href="/Pauper/budget-decks"><span class="hidden-xs"><i class="glyphicon glyphicon-certificate"></i> Pauper</span> Budget Decks </a>
      </li>

      <li role="presentation" class="">
        <a href="/Pauper/prices"><span class="hidden-xs"><i class="glyphicon glyphicon-tag"></i> Pauper </span>  <span class="hidden-xs">Card</span> Prices</a>
      </li>
      -->
</ul>
<div id="event">
<h1>
Tropical Pauper 227
</h1>
<ul class="nav nav-tabs" role="tablist" style="margin-top:5px;">
<li role="presentation" class="active"><a href="https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694">Tournament details</a>
<li role="presentation" class=""><a href="https://mtgdecks.net/Pauper/tropical-pauper-227-tournament-188694/winrates">Archetypes winrate matrix</a>
</ul>
<div class="card-item">
<div class="row">
<div class="col-xs-12 col-lg-12">
<div class="text-white">
<span style="font-weight: bold;"> Tropical Pauper 227 </span>
<div style="display:inline-block"><div style="font-size:8px;color: rgb(245, 159, 0, 0.7);"><span class="glyphicon glyphicon-star"></span><span class="glyphicon glyphicon-star"></span></div></div>
</div>
<div style="margin:5px 0 10px 0;">
<a class="btn btn-xs btn-default Pauper" href="/Pauper" class=" Pauper"> Pauper</a>
<div class="btn btn-xs btn-default">
32 Players
</div>
<a class="btn btn-xs btn-default" href="https://mtg.cardsrealm.com/en-us/tournament/1k2df-tropical-pauper-227" rel="nofollow">cardsrealm.com</a>
<div style="margin-top: 5px;display:inline-block;">
<div class="small text-uppercase" style="display: inline-block;">
Tournament |
2025-03-22	</div>
</div>
</div>
<a target="_blank" class="btn-outline btn-xs" href="https://mtgdecks.net/Pauper/tropical-pauper-227-story-188694">View in story Mode</a>
</div>
</div>
</div>
<h2 id="event-decks" style="margin-top:20px;"> Tropical Pauper 227 Decks</h2>
<div class="row">
<div class="col-sm-12">
<table class="clickable table table-striped">
<tr>
<th>Rank</th>
<th>Deck</th>
<th class="hidden-xs">Archetype</th>
<th class="hidden-xs">Spiciness</th>
<th></th>
<th>Price</th>
<th></th>
</tr>
<tr style="" class="">
<td><strong>1st</strong></td>
<td>
<strong>
<a href="/Pauper/jundao-mol-decklist-by-kakotibs-pato-2419621">Jundão Mol</a>	</strong>
<br/>
<span class="text-capitalize">by kakotibs_pato</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$74
</span>
<span class="mtgo option" style="display:none">
$116
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Jund Wildfire</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">1%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="12" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:1%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$74
</span>
<span class="mtgo option" style="display:none">
$116
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">1%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="12" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:1%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9qdW5kYW8tbW9sLWRlY2tsaXN0LWJ5LWtha290aWJzLXBhdG8tMjQxOTYyMQ" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9qdW5kYW8tbW9sLWRlY2tsaXN0LWJ5LWtha290aWJzLXBhdG8tMjQxOTYyMS92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>2nd</strong></td>
<td>
<strong>
<a href="/Pauper/terrorismo-decklist-by-haoya-2419622">Terrorismo</a>	</strong>
<br/>
<span class="text-capitalize">by haoya</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$38
</span>
<span class="mtgo option" style="display:none">
$1
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Blue Tempo</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">25%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:25%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$38
</span>
<span class="mtgo option" style="display:none">
$1
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">25%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:25%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci90ZXJyb3Jpc21vLWRlY2tsaXN0LWJ5LWhhb3lhLTI0MTk2MjI" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci90ZXJyb3Jpc21vLWRlY2tsaXN0LWJ5LWhhb3lhLTI0MTk2MjIvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top4</strong></td>
<td>
<strong>
<a href="/Pauper/white-weenie-decklist-by-carrodapolicia-2419623">White Weenie</a>	</strong>
<br/>
<span class="text-capitalize">by carrodapolicia</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$61
</span>
<span class="mtgo option" style="display:none">
$43
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">White Weenie</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-w ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">7%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:7%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$61
</span>
<span class="mtgo option" style="display:none">
$43
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-w ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">7%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:7%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci93aGl0ZS13ZWVuaWUtZGVja2xpc3QtYnktY2Fycm9kYXBvbGljaWEtMjQxOTYyMw" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci93aGl0ZS13ZWVuaWUtZGVja2xpc3QtYnktY2Fycm9kYXBvbGljaWEtMjQxOTYyMy92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top4</strong></td>
<td>
<strong>
<a href="/Pauper/affinitoso-decklist-by-gusisdreaming-2419624">Affinitoso</a>	</strong>
<br/>
<span class="text-capitalize">by gusisdreaming</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$153
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Grixis Affinity</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">5%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:5%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$153
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">5%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:5%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9hZmZpbml0b3NvLWRlY2tsaXN0LWJ5LWd1c2lzZHJlYW1pbmctMjQxOTYyNA" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9hZmZpbml0b3NvLWRlY2tsaXN0LWJ5LWd1c2lzZHJlYW1pbmctMjQxOTYyNC92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top8</strong></td>
<td>
<strong>
<a href="/Pauper/ur-mystic-monarch-terror-decklist-by-gringop-2419625">UR mystic monarch te...</a>	</strong>
<br/>
<span class="text-capitalize">by gringop</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$107
</span>
<span class="mtgo option" style="display:none">
$96
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Izzet Terror</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">8%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:8%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$107
</span>
<span class="mtgo option" style="display:none">
$96
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">8%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:8%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci91ci1teXN0aWMtbW9uYXJjaC10ZXJyb3ItZGVja2xpc3QtYnktZ3JpbmdvcC0yNDE5NjI1" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci91ci1teXN0aWMtbW9uYXJjaC10ZXJyb3ItZGVja2xpc3QtYnktZ3JpbmdvcC0yNDE5NjI1L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top8</strong></td>
<td>
<strong>
<a href="/Pauper/dimir-faeries-decklist-by-garcia-edu-2419626">Dimir faeries</a>	</strong>
<br/>
<span class="text-capitalize">by garcia_edu</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$68
</span>
<span class="mtgo option" style="display:none">
$19
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">UB Faeries</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">45%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="41" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:45%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$68
</span>
<span class="mtgo option" style="display:none">
$19
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">45%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="41" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:45%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9kaW1pci1mYWVyaWVzLWRlY2tsaXN0LWJ5LWdhcmNpYS1lZHUtMjQxOTYyNg" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9kaW1pci1mYWVyaWVzLWRlY2tsaXN0LWJ5LWdhcmNpYS1lZHUtMjQxOTYyNi92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top8</strong></td>
<td>
<strong>
<a href="/Pauper/affinity-decklist-by-hericllys-2419627">Affinity</a>	</strong>
<br/>
<span class="text-capitalize">by hericllys</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$96
</span>
<span class="mtgo option" style="display:none">
$115
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Grixis Affinity</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">6%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:6%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$96
</span>
<span class="mtgo option" style="display:none">
$115
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">6%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:6%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9hZmZpbml0eS1kZWNrbGlzdC1ieS1oZXJpY2xseXMtMjQxOTYyNw" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9hZmZpbml0eS1kZWNrbGlzdC1ieS1oZXJpY2xseXMtMjQxOTYyNy92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top8</strong></td>
<td>
<strong>
<a href="/Pauper/dimir-affinity-whale-decklist-by-miguelmezher-2419628">Dimir Affinity Whale</a>	</strong>
<br/>
<span class="text-capitalize">by miguelmezher</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$42
</span>
<span class="mtgo option" style="display:none">
$19
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Dimir Control</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">92%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="104" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:92%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$42
</span>
<span class="mtgo option" style="display:none">
$19
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">92%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="104" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:92%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9kaW1pci1hZmZpbml0eS13aGFsZS1kZWNrbGlzdC1ieS1taWd1ZWxtZXpoZXItMjQxOTYyOA" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9kaW1pci1hZmZpbml0eS13aGFsZS1kZWNrbGlzdC1ieS1taWd1ZWxtZXpoZXItMjQxOTYyOC92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/bw-pestilencia-decklist-by-tiagofuguete-2419629">BW Pestilencia</a>	</strong>
<br/>
<span class="text-capitalize">by tiagofuguete</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$144
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Abzan Food</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">2%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="28" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:2%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$144
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">2%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="28" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:2%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9idy1wZXN0aWxlbmNpYS1kZWNrbGlzdC1ieS10aWFnb2Z1Z3VldGUtMjQxOTYyOQ" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9idy1wZXN0aWxlbmNpYS1kZWNrbGlzdC1ieS10aWFnb2Z1Z3VldGUtMjQxOTYyOS92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/pauper-ug-walls-combo-decklist-by-qikfix-2419630">Pauper - UG Walls Co...</a>	</strong>
<br/>
<span class="text-capitalize">by qikfix</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$47
</span>
<span class="mtgo option" style="display:none">
$10
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Defender Combo</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-u ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-b ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">9%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:9%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$47
</span>
<span class="mtgo option" style="display:none">
$10
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-u ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-b ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">9%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:9%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9wYXVwZXItdWctd2FsbHMtY29tYm8tZGVja2xpc3QtYnktcWlrZml4LTI0MTk2MzA" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9wYXVwZXItdWctd2FsbHMtY29tYm8tZGVja2xpc3QtYnktcWlrZml4LTI0MTk2MzAvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/u-fadas-decklist-by-avcovardes-2419631">U Fadas</a>	</strong>
<br/>
<span class="text-capitalize">by avcovardes</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$54
</span>
<span class="mtgo option" style="display:none">
$64
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Blue Faeries</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">1%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="9" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:1%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$54
</span>
<span class="mtgo option" style="display:none">
$64
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">1%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="9" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:1%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci91LWZhZGFzLWRlY2tsaXN0LWJ5LWF2Y292YXJkZXMtMjQxOTYzMQ" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci91LWZhZGFzLWRlY2tsaXN0LWJ5LWF2Y292YXJkZXMtMjQxOTYzMS92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/mono-red-decklist-by-renanbiko-2419632">mono red</a>	</strong>
<br/>
<span class="text-capitalize">by renanbiko</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$46
</span>
<span class="mtgo option" style="display:none">
$2
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Red Kuldotha</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">13%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="22" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:13%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$46
</span>
<span class="mtgo option" style="display:none">
$2
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">13%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="22" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:13%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vLXJlZC1kZWNrbGlzdC1ieS1yZW5hbmJpa28tMjQxOTYzMg" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vLXJlZC1kZWNrbGlzdC1ieS1yZW5hbmJpa28tMjQxOTYzMi92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/ub-snacker-decklist-by-conceder-2419633">UB Snacker</a>	</strong>
<br/>
<span class="text-capitalize">by conceder</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$113
</span>
<span class="mtgo option" style="display:none">
$153
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Dimir Control</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">22%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="42" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:22%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$113
</span>
<span class="mtgo option" style="display:none">
$153
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">22%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="42" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:22%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci91Yi1zbmFja2VyLWRlY2tsaXN0LWJ5LWNvbmNlZGVyLTI0MTk2MzM" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci91Yi1zbmFja2VyLWRlY2tsaXN0LWJ5LWNvbmNlZGVyLTI0MTk2MzMvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/monowhite-heroic-2-decklist-by-t4sfr31-2419634">Monowhite heroic 2</a>	</strong>
<br/>
<span class="text-capitalize">by t4sfr31</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$18
</span>
<span class="mtgo option" style="display:none">
$1
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono White Heroic</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-w ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">28%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:28%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$18
</span>
<span class="mtgo option" style="display:none">
$1
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-w ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">28%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:28%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vd2hpdGUtaGVyb2ljLTItZGVja2xpc3QtYnktdDRzZnIzMS0yNDE5NjM0" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vd2hpdGUtaGVyb2ljLTItZGVja2xpc3QtYnktdDRzZnIzMS0yNDE5NjM0L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/combinho-da-alegria-decklist-by-fitta-333-2419635">Combinho da Alegria</a>	</strong>
<br/>
<span class="text-capitalize">by fitta_333</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$117
</span>
<span class="mtgo option" style="display:none">
$187
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Glee Combo</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">16%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="31" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:16%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$117
</span>
<span class="mtgo option" style="display:none">
$187
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">16%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="31" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:16%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9jb21iaW5oby1kYS1hbGVncmlhLWRlY2tsaXN0LWJ5LWZpdHRhLTMzMy0yNDE5NjM1" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9jb21iaW5oby1kYS1hbGVncmlhLWRlY2tsaXN0LWJ5LWZpdHRhLTMzMy0yNDE5NjM1L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="">
<td><strong>Top16</strong></td>
<td>
<strong>
<a href="/Pauper/mono-blue-faeries-decklist-by-gabriel-l-2419636">Mono Blue Faeries</a>	</strong>
<br/>
<span class="text-capitalize">by gabriel_l</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$49
</span>
<span class="mtgo option" style="display:none">
$26
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Blue Faeries</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">4%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="11" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:4%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$49
</span>
<span class="mtgo option" style="display:none">
$26
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">4%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="11" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:4%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vLWJsdWUtZmFlcmllcy1kZWNrbGlzdC1ieS1nYWJyaWVsLWwtMjQxOTYzNg" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vLWJsdWUtZmFlcmllcy1kZWNrbGlzdC1ieS1nYWJyaWVsLWwtMjQxOTYzNi92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/red-dread-redemption-decklist-by-drjozeca-2419637">Red Dread Redemption</a>	</strong>
<br/>
<span class="text-capitalize">by drjozeca</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$36
</span>
<span class="mtgo option" style="display:none">
$22
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Red Madness</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">11%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="13" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:11%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$36
</span>
<span class="mtgo option" style="display:none">
$22
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">11%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="13" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:11%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9yZWQtZHJlYWQtcmVkZW1wdGlvbi1kZWNrbGlzdC1ieS1kcmpvemVjYS0yNDE5NjM3" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9yZWQtZHJlYWQtcmVkZW1wdGlvbi1kZWNrbGlzdC1ieS1kcmpvemVjYS0yNDE5NjM3L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/ub-fada-4x4-decklist-by-hstopas-2419638">UB fada 4x4</a>	</strong>
<br/>
<span class="text-capitalize">by hstopas</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$100
</span>
<span class="mtgo option" style="display:none">
$139
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Dimir Control</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">3%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="26" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:3%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$100
</span>
<span class="mtgo option" style="display:none">
$139
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">3%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="26" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:3%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci91Yi1mYWRhLTR4NC1kZWNrbGlzdC1ieS1oc3RvcGFzLTI0MTk2Mzg" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci91Yi1mYWRhLTR4NC1kZWNrbGlzdC1ieS1oc3RvcGFzLTI0MTk2MzgvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/mono-red-2024-decklist-by-vat69-2419639">Mono Red 2024</a>	</strong>
<br/>
<span class="text-capitalize">by vat69</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$91
</span>
<span class="mtgo option" style="display:none">
$61
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Red Kuldotha</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">8%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:8%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$91
</span>
<span class="mtgo option" style="display:none">
$61
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">8%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="17" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:8%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vLXJlZC0yMDI0LWRlY2tsaXN0LWJ5LXZhdDY5LTI0MTk2Mzk" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vLXJlZC0yMDI0LWRlY2tsaXN0LWJ5LXZhdDY5LTI0MTk2MzkvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/mono-bluer-decklist-by-duartekelvyn-2419640">Mono bluer</a>	</strong>
<br/>
<span class="text-capitalize">by duartekelvyn</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$63
</span>
<span class="mtgo option" style="display:none">
$62
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Blue Tempo</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">11%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="18" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:11%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$63
</span>
<span class="mtgo option" style="display:none">
$62
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">11%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="18" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:11%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vLWJsdWVyLWRlY2tsaXN0LWJ5LWR1YXJ0ZWtlbHZ5bi0yNDE5NjQw" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vLWJsdWVyLWRlY2tsaXN0LWJ5LWR1YXJ0ZWtlbHZ5bi0yNDE5NjQwL3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/fadas-ub-decklist-by-joaolucas-2419641">Fadas ub</a>	</strong>
<br/>
<span class="text-capitalize">by joaolucas</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$103
</span>
<span class="mtgo option" style="display:none">
$127
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Izzet Faeries</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">65%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="58" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:65%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$103
</span>
<span class="mtgo option" style="display:none">
$127
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">65%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="58" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:65%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9mYWRhcy11Yi1kZWNrbGlzdC1ieS1qb2FvbHVjYXMtMjQxOTY0MQ" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9mYWRhcy11Yi1kZWNrbGlzdC1ieS1qb2FvbHVjYXMtMjQxOTY0MS92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/gruul-brave-decklist-by-antonio-junior-aj-2419642">Gruul Brave</a>	</strong>
<br/>
<span class="text-capitalize">by antonio_junior_aj</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$69
</span>
<span class="mtgo option" style="display:none">
$12
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Green Stompy</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">100%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="95" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:100%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$69
</span>
<span class="mtgo option" style="display:none">
$12
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">100%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="95" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:100%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9ncnV1bC1icmF2ZS1kZWNrbGlzdC1ieS1hbnRvbmlvLWp1bmlvci1hai0yNDE5NjQy" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9ncnV1bC1icmF2ZS1kZWNrbGlzdC1ieS1hbnRvbmlvLWp1bmlvci1hai0yNDE5NjQyL3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/bogao-deformado-2-0-decklist-by-pehbaldoo-2419643">Bogão Deformado 2.0</a>	</strong>
<br/>
<span class="text-capitalize">by pehbaldoo</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$64
</span>
<span class="mtgo option" style="display:none">
$27
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">GW Bogles</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">16%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="22" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:16%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$64
</span>
<span class="mtgo option" style="display:none">
$27
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">16%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="22" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:16%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9ib2dhby1kZWZvcm1hZG8tMi0wLWRlY2tsaXN0LWJ5LXBlaGJhbGRvby0yNDE5NjQz" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9ib2dhby1kZWZvcm1hZG8tMi0wLWRlY2tsaXN0LWJ5LXBlaGJhbGRvby0yNDE5NjQzL3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/altar-decklist-by-felipe1983-2419644">altar</a>	</strong>
<br/>
<span class="text-capitalize">by felipe1983</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$65
</span>
<span class="mtgo option" style="display:none">
$31
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Altar Tron</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">55%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="34" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:55%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$65
</span>
<span class="mtgo option" style="display:none">
$31
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">55%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="34" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:55%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9hbHRhci1kZWNrbGlzdC1ieS1mZWxpcGUxOTgzLTI0MTk2NDQ" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9hbHRhci1kZWNrbGlzdC1ieS1mZWxpcGUxOTgzLTI0MTk2NDQvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/jund-wildfire-decklist-by-davi00-2419645">Jund Wildfire</a>	</strong>
<br/>
<span class="text-capitalize">by davi00</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$79
</span>
<span class="mtgo option" style="display:none">
$84
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Jund Wildfire</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">5%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:5%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$79
</span>
<span class="mtgo option" style="display:none">
$84
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">5%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="16" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:5%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9qdW5kLXdpbGRmaXJlLWRlY2tsaXN0LWJ5LWRhdmkwMC0yNDE5NjQ1" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9qdW5kLXdpbGRmaXJlLWRlY2tsaXN0LWJ5LWRhdmkwMC0yNDE5NjQ1L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/monoreddredge-decklist-by-santossantos-2419646">monoreddredge</a>	</strong>
<br/>
<span class="text-capitalize">by santossantos</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$36
</span>
<span class="mtgo option" style="display:none">
$22
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Mono Red Madness</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">3%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:3%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$36
</span>
<span class="mtgo option" style="display:none">
$22
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">3%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:3%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9tb25vcmVkZHJlZGdlLWRlY2tsaXN0LWJ5LXNhbnRvc3NhbnRvcy0yNDE5NjQ2" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9tb25vcmVkZHJlZGdlLWRlY2tsaXN0LWJ5LXNhbnRvc3NhbnRvcy0yNDE5NjQ2L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/dimir-control-decklist-by-tolarian-academy-2419647">Dimir Control</a>	</strong>
<br/>
<span class="text-capitalize">by tolarian_academy</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$133
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Dimir Control</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">7%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:7%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$102
</span>
<span class="mtgo option" style="display:none">
$133
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">7%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="29" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:7%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9kaW1pci1jb250cm9sLWRlY2tsaXN0LWJ5LXRvbGFyaWFuLWFjYWRlbXktMjQxOTY0Nw" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9kaW1pci1jb250cm9sLWRlY2tsaXN0LWJ5LXRvbGFyaWFuLWFjYWRlbXktMjQxOTY0Ny92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/elfos-tia-cris-decklist-by-tiomax-2419648">Elfos+Tia Cris</a>	</strong>
<br/>
<span class="text-capitalize">by tiomax</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$45
</span>
<span class="mtgo option" style="display:none">
$34
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Elves</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">19%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="26" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:19%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$45
</span>
<span class="mtgo option" style="display:none">
$34
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">19%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="26" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:19%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9lbGZvcy10aWEtY3Jpcy1kZWNrbGlzdC1ieS10aW9tYXgtMjQxOTY0OA" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9lbGZvcy10aWEtY3Jpcy1kZWNrbGlzdC1ieS10aW9tYXgtMjQxOTY0OC92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/jaula-tron-decklist-by-zewill-2419649">Jaula Tron</a>	</strong>
<br/>
<span class="text-capitalize">by zewill</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$38
</span>
<span class="mtgo option" style="display:none">
$28
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Monster Tron</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">40%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="35" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:40%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$38
</span>
<span class="mtgo option" style="display:none">
$28
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">40%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="35" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:40%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9qYXVsYS10cm9uLWRlY2tsaXN0LWJ5LXpld2lsbC0yNDE5NjQ5" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9qYXVsYS10cm9uLWRlY2tsaXN0LWJ5LXpld2lsbC0yNDE5NjQ5L3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/grull-sem-amizades-decklist-by-jukaknight-2419650">Grull Sem Amizades</a>	</strong>
<br/>
<span class="text-capitalize">by jukaknight</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$85
</span>
<span class="mtgo option" style="display:none">
$125
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Gruul Ponza</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">28%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="37" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:28%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$85
</span>
<span class="mtgo option" style="display:none">
$125
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">28%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="37" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:28%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9ncnVsbC1zZW0tYW1pemFkZXMtZGVja2xpc3QtYnktanVrYWtuaWdodC0yNDE5NjUw" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9ncnVsbC1zZW0tYW1pemFkZXMtZGVja2xpc3QtYnktanVrYWtuaWdodC0yNDE5NjUwL3Zpc3VhbA" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/rakdos-madness-burn-decklist-by-seether7-2419651">Rakdos Madness Burn</a>	</strong>
<br/>
<span class="text-capitalize">by seether7</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$73
</span>
<span class="mtgo option" style="display:none">
$93
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small">Rakdos Madness</span>
<br/>
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">21%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:21%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$73
</span>
<span class="mtgo option" style="display:none">
$93
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span></div>
<div class="visible-xs">
<span style="font-size: 10px;color: rgb(245, 159, 0);">21%</span>
<div class="progress" style="height: 8px;margin-top:0px;margin-bottom: 0;max-width:60px;border-radius: 3px;">
<div class="progress-bar progress-bar-info" role="progressbar" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100" style="line-height: 10px;width:21%;background:rgb(245, 159, 0, 0.6); /* Chrome10-25,Safari5.1-6 */">
</div>
</div>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9yYWtkb3MtbWFkbmVzcy1idXJuLWRlY2tsaXN0LWJ5LXNlZXRoZXI3LTI0MTk2NTE" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9yYWtkb3MtbWFkbmVzcy1idXJuLWRlY2tsaXN0LWJ5LXNlZXRoZXI3LTI0MTk2NTEvdmlzdWFs" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
<tr style="" class="hidden collapsible_deck">
<td><strong>Top32</strong></td>
<td>
<strong>
<a href="/Pauper/bg-ponza-groselhao-decklist-by-anper-2419652">BG Ponza Groselhão</a>	</strong>
<br/>
<span class="text-capitalize">by anper</span>
<br/>
<div class="price visible-xs">
<span style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$55
</span>
<span class="mtgo option" style="display:none">
$5
</span>
<span class="arena option" style="display:none">
</span>
</span></span>
</div>
</td>
<td class="hidden-xs">
<span class="small" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></span>
</td>
<td class="hidden-xs">
<span class="small">Not available</span><br/>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</td>
<td>
</td>
<td>
<div class="price hidden-xs">
<strong style="color: rgb(95, 130, 95); cursor: pointer;">
<span class="price">
<span class="paper option">
$55
</span>
<span class="mtgo option" style="display:none">
$5
</span>
<span class="arena option" style="display:none">
</span>
</span></strong>
</div>
<div class="small visible-xs" style="font-size: 10px;white-space: nowrap;"> <span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span></div>
<div class="visible-xs">
<span class="small">Not available</span><br/>
<div class="visible-xs" style="color: rgb(245, 159, 0);font-size:10px;">
spiciness
</div>	</div>
</td>
<td class="" style="text-align: center;">
<span><span goto="L1BhdXBlci9iZy1wb256YS1ncm9zZWxoYW8tZGVja2xpc3QtYnktYW5wZXItMjQxOTY1Mg" class="linker"><span class="btn btn-primary btn-sm">List view</span></span></span>	<br>
<span><span goto="L1BhdXBlci9iZy1wb256YS1ncm9zZWxoYW8tZGVja2xpc3QtYnktYW5wZXItMjQxOTY1Mi92aXN1YWw" class="linker"><span class="small">Visual view</span></span></span>	</td>
</tr>
</table>
<div id="decks-loader" class="btn btn-primary btn-md btn-block">Show all (32)</div>
<div class="mon-placement loading-mon " style="width:100%;color:white; "><div id="placement-mpu" class="raptive-placeholder-mpu raptive-placement" style="min-height:250px;max-height:250px;overflow:hidden;width:100%;display:flex;"></div></div>
<script data-pagespeed-orig-type="text/javascript" type="text/psajs" data-pagespeed-orig-index="5">$(document).ready(function(){$("#decks-loader").on('click',function(){$(".collapsible_deck").removeClass('hidden');$(this).remove();});});</script>
</div>
</div>
<div class="row">
<div class="col-md-6">
<h2 id="event-archetypes">Tournament Archetype breakdown</h2>
<table class="clickable table table-striped table-condensed" id="archetype-breakdown">
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xBrainstorm.jpg.pagespeed.ic.56MEEDo434.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="766584497" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/dimir-control">Dimir Control</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="13" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 26%;background:#959595;/*background: #0000aa*/">
13%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xCounterspell.jpg.pagespeed.ic.eFORsXlmnV.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1944308166" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-blue-tempo">Mono Blue Tempo</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #4444dd*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xChainer,27s,P20Edict.jpg.pagespeed.ic.n1xIPty6ev.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="2110215510" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/jund-wildfire">Jund Wildfire</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #aaaa00*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xMyr,P20Enforcer.jpg.pagespeed.ic.S_Sicd_ZOY.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="2353680075" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/grixis-affinity">Grixis Affinity</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="ms ms-cost ms-u ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #aa00aa*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xFaerie,P20Miscreant.jpg.pagespeed.ic.NfwOsIxF8M.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="4126895331" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-blue-faeries">Mono Blue Faeries</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #4444dd*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xKuldotha,P20Rebirth.jpg.pagespeed.ic.XEGflsHODN.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="3920795351" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-red-kuldotha">Mono Red Kuldotha</a>
<br/>
<span class="ms ms-cost ms-r ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #dd4444*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xSneaky,P20Snacker.jpg.pagespeed.ic.luo5Z-oc9V.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="3768810695" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-red-madness">Mono Red Madness</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-g ms-shadow"></span></span><span class="ms ms-cost ms-u ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="7" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 14%;background:#959595;/*background: #aa55aa*/">
7%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xVoldaren,P20Epicure.jpg.pagespeed.ic.qjQYAdbUjC.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="2115359581" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/rakdos-madness">Rakdos Madness</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="ms ms-cost ms-u ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #aa00aa*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xArbor,P20Elf.jpg.pagespeed.ic.Loi5uIEhP4.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1337569992" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/gruul-ponza">Gruul Ponza</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #dddd44*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xBoulderbranch,P20Golem.jpg.pagespeed.ic.IJyjrPBxza.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1177515780" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/monster-tron">Monster Tron</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-b ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #66bb22*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xQuirion,P20Ranger.jpg.pagespeed.ic.MQ5Bepe9UH.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="3379318631" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/elves">Elves</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #44dd88*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xUrza,27s,P20Mine.jpg.pagespeed.ic.TDkQXI_kt6.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1607177730" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/altar-tron">Altar Tron</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #00aa00*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xSlippery,P20Bogle.jpg.pagespeed.ic.es2fhYLI9X.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="4227711646" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/gw-bogles">GW Bogles</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #55ff55*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xBurning-Tree,P20Emissary.jpg.pagespeed.ic.-Ku6S9TH1y.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="249663250" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-green-stompy">Mono Green Stompy</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #44dd44*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xSkred.jpg.pagespeed.ic.vmPkWl-geL.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1481363041" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/izzet-faeries">Izzet Faeries</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #0000aa*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xSadistic,P20Glee.jpg.pagespeed.ic.qoNGto4bRi.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="539597646" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/glee-combo">Glee Combo</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #55aa00*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xEthereal,P20Armor.jpg.pagespeed.ic.9YV1AKKckg.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="3216044288" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/mono-white-heroic">Mono White Heroic</a>
<br/>
<span class="ms ms-cost ms-w ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #555555*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xAxebane,P20Guardian.jpg.pagespeed.ic.AZe65mcO54.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="3281184950" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/defender-combo">Defender Combo</a>
<br/>
<span class="ms ms-cost ms-g ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-u ms-shadow"></span></span><span class="small-icon"><span class="ms ms-cost ms-b ms-shadow"></span></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #bbbb66*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xCarrot,P20Cake.jpg.pagespeed.ic.jGi74fgnUJ.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="247265860" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/abzan-food">Abzan Food</a>
<br/>
<span class="ms ms-cost ms-b ms-shadow"></span><span class="ms ms-cost ms-w ms-shadow"></span><span class="ms ms-cost ms-g ms-shadow"></span><span class="small-icon"><span class="ms ms-cost ms-r ms-shadow"></span></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #55aa00*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xFaerie,P20Seer.jpg.pagespeed.ic.1iZlzOG68S.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="2661955974" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/ub-faeries">UB Faeries</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-b ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #0000aa*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xVolatile,P20Fjord.jpg.pagespeed.ic.qbgA0hbbEM.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="1515962820" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/izzet-terror">Izzet Terror</a>
<br/>
<span class="ms ms-cost ms-u ms-shadow"></span><span class="ms ms-cost ms-r ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #dd44dd*/">
4%
</div>
</div>
</td>
</tr>
<tr>
<td class="col-xs-1 th">
<img src="/img/cropped/xRaffine,27s,P20Informant.jpg.pagespeed.ic.NDx4-JJACY.jpg" class="img-responsive" style="opacity:0.5;height:20px;" alt="" data-pagespeed-url-hash="2257507572" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>
</td>
<td class="col-xs-4 th">
<a href="/Pauper/white-weenie">White Weenie</a>
<br/>
<span class="ms ms-cost ms-w ms-shadow"></span>	</td>
<td class="col-xs-6">
<div class="progress">
<div class="progress-bar" role="progressbar" aria-valuenow="4" aria-valuemin="0" aria-valuemax="100" style="font-size:10px;width: 8%;background:#959595;/*background: #555555*/">
4%
</div>
</div>
</td>
</tr>
</table>
</div>
<div class="col-md-6">
<h2 id="event-cards">Tournament Most Played Cards</h2>
<table class="clickable table table-striped table-condensed">
<tr>
<th>#</th>
<th>Card Name</th>
<th>Price</th>
<th>Image</th>
</tr>
<tr>
<td><strong>1st</strong></td>
<td><strong><a href="/prices/cast-down">Cast Down</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.79
</span>
<span class="mtgo option" style="display:none">
$1.24
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xCast,P20Down.jpg.pagespeed.ic.rt2l666XZS.jpg" width="40px" alt="" data-pagespeed-url-hash="4182411902" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>2nd</strong></td>
<td><strong><a href="/prices/nihil-spellbomb">Nihil Spellbomb</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$2.29
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xNihil,P20Spellbomb.jpg.pagespeed.ic.RRVRyQDFDS.jpg" width="40px" alt="" data-pagespeed-url-hash="3892551535" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>3rd</strong></td>
<td><strong><a href="/prices/dispel">Dispel</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.39
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/Dispel.jpg" width="40px" alt="" data-pagespeed-url-hash="32621435" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>4th</strong></td>
<td><strong><a href="/prices/counterspell">Counterspell</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$1.99
</span>
<span class="mtgo option" style="display:none">
$0.02
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xCounterspell.jpg.pagespeed.ic.eFORsXlmnV.jpg" width="40px" alt="" data-pagespeed-url-hash="1944308166" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>5th</strong></td>
<td><strong><a href="/prices/deadly-dispute">Deadly Dispute</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$1.49
</span>
<span class="mtgo option" style="display:none">
$0.01
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xDeadly,P20Dispute.jpg.pagespeed.ic.g_sKsmdIbC.jpg" width="40px" alt="" data-pagespeed-url-hash="2646249128" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>6th</strong></td>
<td><strong><a href="/prices/annul">Annul</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.35
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xAnnul.jpg.pagespeed.ic.vXwuCtPuHQ.jpg" width="40px" alt="" data-pagespeed-url-hash="1831740292" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>7th</strong></td>
<td><strong><a href="/prices/snuff-out">Snuff Out</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$11.99
</span>
<span class="mtgo option" style="display:none">
$14.44
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xSnuff,P20Out.jpg.pagespeed.ic.yl8a_-RUmL.jpg" width="40px" alt="" data-pagespeed-url-hash="301906837" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>8th</strong></td>
<td><strong><a href="/prices/suffocating-fumes">Suffocating Fumes</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.59
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/Suffocating Fumes.jpg" width="40px" alt="" data-pagespeed-url-hash="3143034678" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>9th</strong></td>
<td><strong><a href="/prices/ichor-wellspring">Ichor Wellspring</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.79
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/xIchor,P20Wellspring.jpg.pagespeed.ic.RHuj5ofr5X.jpg" width="40px" alt="" data-pagespeed-url-hash="402212131" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
<tr>
<td><strong>10th</strong></td>
<td><strong><a href="/prices/murmuring-mystic">Murmuring Mystic</a></strong></td>
<td class="price">
<span class="price">
<span class="paper option">
$0.49
</span>
<span class="mtgo option" style="display:none">
$0.03
</span>
<span class="arena option" style="display:none">
</span>
</span></td>
<td><img src="/img/cropped/Murmuring Mystic.jpg" width="40px" alt="" data-pagespeed-url-hash="466979974" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></td>
</tr>
</table>
</div>
<div class="col-xs-12">
<div class="mon-placement loading-mon " style="width:100%;color:white; "><div id="placement-after-content" class="raptive-placeholder-after-content raptive-placement" style="min-height:250px;max-height:250px;overflow:hidden;"></div></div>
</div>
<div class="col-md-12">
<h2>Table of contents</h2>
<ul style="list-style: square;">
<li><a href="#event-decks" class="text-uppercase">Decklist from the event</span></a></li>
<li><a href="#event-archetypes" class="text-uppercase">Archetype Breakdown</span></a></li>
<li><a href="#event-cards" class="text-uppercase">Most played Cards</a></li>
</ul>
</div>
</div>
</div>
<div id="status">
</div>
<div class="modal fade" id="remoteModal" tabindex="-1" role="dialog" style="z-index: 999999999;">
<div class="modal-dialog" role="document">
<div class="modal-content">
<div class="modal-header">
<button type="button" class="close" data-dismiss="modal">&times;</button>
<h4 class="modal-title"></h4>
</div>
<div class="modal-body">
</div>
</div><!-- /.modal-content -->
</div><!-- /.modal-dialog -->
</div><!-- /.modal -->
</div>
</div>
</div>
<div id="footer">
<div class="container">
<div class="row">
<div class="col-md-2 col-sm-3 col-xs-12">
<img src="/img/logo.svg" loading="lazy" height="40px" alt="Mtg decks" data-pagespeed-url-hash="1923278652" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	<br class="visible-xs"><br class="visible-xs">
<div class="small">
<i class="glyphicon glyphicon-map-marker" aria-hidden="true"></i> MTGDecks.net<br/>
Calle Pintor Espinosa 21 <br/>
Córdoba, 14004 <br/>
Spain
</div>
<br/><br/>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Competitive Formats</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><a href="/Standard">Standard Decks</a></li>
<li><a href="/Pioneer">Pioneer Decks </a></li>
<li><a href="/Modern">Modern Decks</a></li>
<li><a href="/Pauper">Pauper Decks </a></li>
</ul>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Arena exclusive formats</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><a href="/Alchemy">Alchemy Decks </a></li>
<li><a href="/Explorer">Explorer Decks </a></li>
<li><a href="/Historic">Historic Decks </a></li>
<li><a href="/Timeless">Timeless Decks </a></li>
<li><a href="/Brawl">Brawl Decks </a></li>
<li><a href="/Historic-Brawl">Historic-Brawl Decks </a></li>
</ul>
<b>Commander Formats</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><a href="/Commander">Commander Decks </a></li>
<li><a href="/Duel-Commander">Duel-Commander Decks </a></li>
<li><span><span goto="L1JlYWwtQ29tbWFuZGVy" class="linker">Real Commander decks</span></span></li>
</ul>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Eternal & Old School</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><a href="/Legacy">Legacy Decks </a></li>
<li><a href="/Vintage">Vintage Decks </a></li>
<li><a href="/Premodern">Premodern Decks </a></li>
<li><a href="/Old-school">Old School Decks </a></li>
<li><span><span goto="L0NsYXNzaWMtTGVnYWN5" class="linker">Classic-Legacy Decks</span></span></li>
</ul>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Content & Prices</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><a href="/articles">MTGDecks Articles</a></li>
<li><a href="/players">Best deckbuilders</a></li>
<li><a href="/prices">Card Prices</a></li>
</ul>
<br class="visible-sm"/>
<b>Terms & Privacy</b>
<ul class="footer-list" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li><span><span goto="L3BhZ2VzL3ByaXZhY3k" class="linker">Privacy &amp; Cookies policy</span></span></li>
<li><span><span goto="L3BhZ2VzL2d1aWRlbGluZXM" class="linker">Submission guidelines</span></span></li>
<li><span><span goto="L3BhZ2VzL3Rlcm1z" class="linker">Terms of service</span></span></li>
<li><span><span goto="L3BhZ2VzL2ZhcQ" class="linker">FAQ</span></span></li>
</ul>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Contact & Collaborate</b>
<ul class="footer-list">
<li><span><span goto="L2NvbnRhY3Q" class="linker">Contact Form</span></span></li>
<li><a href="/cdn-cgi/l/email-protection#197a76776d787a6d59746d7e7d7c7a726a37777c6d">Email Us</a></li>
<li><span><span goto="L2V2ZW50cy9hZGQ" class="linker">Submit an event</span></span></li>
<li><span><span goto="L3BhZ2VzL3N1cHBvcnQ" class="linker">Make a donation</span></span></li>
</ul>
</div>
<div class="col-md-2 col-sm-3 col-xs-6">
<br class="visible-sm"/>
<b>Partner sites</b>
<ul class="footer-list">
<li><a rel="nofollow" href="https://inkdecks.com">Inkdecks.com - Lorcana decks</a></li>
</ul>
</div>
</div>
<div class="row">
<br class="visible-xs">
<div class="col-xs-12">
<b><span data-ccpa-link="1"></span></b><br/>
Mtgdecks.net &copy;2009-2025. This site provides accurate and independent information on more than 500.000 Magic the Gathering Decks, tournaments and magic singles prices. You can learn more about our database at our <a href="https://datasetsearch.research.google.com/search?docid=et8X1leyAPiU%2FJE9AAAAAA%3D%3D">Google Dataset</a>. This material is provided "as is", with absolutely no warranty expressed or implied. Data sources include Tcgplayer, Mtgtop8, Mtgmelee, Starcitygames, Wizards of the Coast, The Gatherer and many others. To view content sources and attributions, please refer to each tournament info.
</div>
<div class="col-xs-12">
<p>
<a href="https://facebook.com/mtgdecks"><img src="/img/xfb.png.pagespeed.ic.BkCepJYVdp.png" class="img-responsive pull-left" loading="lazy" width="32" alt="facebook" style="margin-right:10px;" data-pagespeed-url-hash="3873147398" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></a>
<a href="https://twitter.com/mtgdecks"><img src="/img/xtwitter.png.pagespeed.ic.3RgOOExp1U.png" class="img-responsive pull-left" loading="lazy" width="32" alt="twitter" style="margin-right:10px;" data-pagespeed-url-hash="3565025811" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></a>
<a href="https://www.dmca.com/Protection/Status.aspx?ID=df78bee2-de42-427e-94fa-99b4a92be11d" title="DMCA.com Protection Status" class="dmca-badge"> <img src="//images.dmca.com/Badges/dmca-badge-w100-5x1-08.png?ID=df78bee2-de42-427e-94fa-99b4a92be11d" loading="lazy" alt="DMCA.com Protection Status" data-pagespeed-url-hash="3291580540" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"></a>
</p>
<p style="font-size:10px;color:gray">The information presented on this site about Magic: The Gathering, both literal and graphical, is copyrighted by Wizards of the Coast. This website is not produced, endorsed, supported, or affiliated with Wizards of the Coast.
</p>
</div>
</div>
</div>
</div>
<div class="menus">
<div class="navbar navbar-inverse navbar-fixed-top" style="position: absolute!important;" role="navigation" id="navigation">
<div class="container">
<div class="navbar-header">
<button class="navbar-toggle" type="button" data-toggle="collapse" data-target="#mainmenu" aria-expanded="false" aria-controls="mainmenu">
<span class="sr-only">Toggle navigation</span>
<span class="icon-bar"></span>
<span class="icon-bar"></span>
<span class="icon-bar"></span>
</button>
<button class="navbar-toggle" style="margin:0" type="button" aria-expanded="false">
<div itemprop="name" class="label-prime">
<a href="/users/users/login" class="box"><span class="fa fa-sign-in"></span> Login</a>	</div>
</button>
<a class="navbar-brand" href="/">
<span class="hidden-xs"><img src="/img/logo.svg" style="margin-top:9px;height:30px;margin-left:10px;" alt="mtg decks" data-pagespeed-url-hash="1923278652" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></span>
<span class="visible-xs" style="padding: 12px 5px 0 5px;"><img src="/img/logoMobile.svg" height="25" alt="mtg decks" data-pagespeed-url-hash="754650962" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/></span>
</a>
<form class="navbar-form navbar-left hidden-sm ahidden-xs" role="search">
<div class="input-group">
<input type="text" class="form-control typeahead" style="width: 100%;" id="cardSearchTop" placeholder="Search a Card">
</div>
</form>
<span class="navbar-left hidden-xs">
<div itemprop="name" class="label-prime btn" style="padding: 6px 0;">
<a href="/users/users/login" class="box"><span class="fa fa-sign-in"></span> Login</a>	</div>
</span>
</div>
<div class="collapse navbar-collapse js-navbar-collapse navbar-right" id="mainmenu">
<nav>
<ul class="nav navbar-nav" itemscope itemtype="http://www.schema.org/SiteNavigationElement">
<li itemprop="name" class="hidden-xs"><a href="/Standard" itemprop="url">Standard</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Pioneer" itemprop="url">Pioneer</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Explorer" itemprop="url">Explorer</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Modern" itemprop="url">Modern</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Legacy" itemprop="url">Legacy</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Pauper" itemprop="url">Pauper</a></li>
<li itemprop="name" class="hidden-xs"><a href="/Commander" itemprop="url">Commander</a></li>
<li class="visible-xs">
<div class="menu-header"><b>Decks & Meta by format:</b></div>
<div class="row">
<div class="col-xs-6 text-center ">
<a href="/Standard"><span class="letterize inverted Standard" style="font-size:14px;margin:5px 10px;">Standard</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Pioneer"><span class="letterize inverted Pioneer" style="font-size:14px;margin:5px 10px;">Pioneer</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Modern"><span class="letterize inverted Modern" style="font-size:14px;margin:5px 10px;">Modern</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Pauper"><span class="letterize inverted Pauper" style="font-size:14px;margin:5px 10px;">Pauper</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Premodern"><span class="letterize inverted Premodern" style="font-size:14px;margin:5px 10px;">Premodern</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Legacy"><span class="letterize inverted Legacy" style="font-size:14px;margin:5px 10px;">Legacy</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Classic-Legacy"><span class="letterize inverted Classic-Legacy" style="font-size:14px;margin:5px 10px;">Classic-Legacy</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Vintage"><span class="letterize inverted Vintage" style="font-size:14px;margin:5px 10px;">Vintage</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Commander"><span class="letterize inverted Commander" style="font-size:14px;margin:5px 10px;">Commander</span></a>
</div>
<div class="col-xs-6 text-center ">
<a href="/Duel-Commander"><span class="letterize inverted Duel-Commander" style="font-size:14px;margin:5px 10px;">Duel-Commander</span></a>
</div>
</div>
<div class="menu-header"><b>MTG Arena decks & meta:</b></div>
<div class="row">
<div class="col-xs-6 text-center">
<a href="/Alchemy"><span class="letterize inverted Alchemy" style="font-size:14px;margin:5px 10px;">Alchemy</span></a>
</div>
<div class="col-xs-6 text-center">
<a href="/Explorer"><span class="letterize inverted Explorer" style="font-size:14px;margin:5px 10px;">Explorer</span></a>
</div>
<div class="col-xs-6 text-center">
<a href="/Historic"><span class="letterize inverted Historic" style="font-size:14px;margin:5px 10px;">Historic</span></a>
</div>
<div class="col-xs-6 text-center">
<a href="/Timeless"><span class="letterize inverted Timeless" style="font-size:14px;margin:5px 10px;">Timeless</span></a>
</div>
<div class="col-xs-6 text-center">
<a href="/Brawl"><span class="letterize inverted Brawl" style="font-size:14px;margin:5px 10px;">Brawl</span></a>
</div>
<div class="col-xs-6 text-center">
<a href="/Historic-Brawl"><span class="letterize inverted Historic-Brawl" style="font-size:14px;margin:5px 10px;">Historic-Brawl</span></a>
</div>
</div>
<div class="menu-header"><b>Read our content:</b></div>
<div class="row">
<div class="col-xs-6 text-center">
<a href="/articles" class="btn btn-default btn-xs full-width" style="font-size:14px;margin:5px 10px;"><img src="/img/logoSmall2.svg" width="20" alt="MTGdecks" data-pagespeed-url-hash="425135551" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/> Articles</a>	</div>
</div>
</li>
<li class="visible-xs omniSwitch">
<div class="menu-header"><b>Card prices preferences:</b></div>
<div class="priceSwitcher" style="padding:0px 0;margin:5px 10px;">
<div id="onlineButton" data-format="mtgo" class="btn btn-xs btn-default">
<img src="/img/icons/xmtgo_black.png.pagespeed.ic.H3srSg8h7S.png" height="12px;" loading="lazy" class="whitened" alt="" data-pagespeed-url-hash="1373742378" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	MTGO
</div>
<div id="mtgaButton" data-format="arena" class="btn btn-xs btn-default">
<img src="/img/icons/xarena.png.pagespeed.ic.LsuMbI_Aty.png" height="12px;" loading="lazy" class="whitened" alt="" data-pagespeed-url-hash="1152051722" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	ARENA
</div>
<div id="paperButton" data-format="paper" class="btn btn-xs btn-primary">
<img src="/img/icons/xmtg.png.pagespeed.ic.S2FSPZPuOM.png" height="12px;" loading="lazy" class="whitened" alt="" data-pagespeed-url-hash="62060803" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/>	PAPER
</div>
</div>
<br/>
</li>
<li class="dropdown dropdown-large hidden-xs">
<a href="#" class="dropdown-toggle" data-toggle="dropdown"><span class="glyphicon glyphicon-plus"></span> More <b class="caret"></b></a>
<ul class="dropdown-menu dropdown-menu-large row">
<li class="col-sm-3">
<ul>
<li class="dropdown-header">More Tabletop Formats</li>
<li itemprop="name">
<a href="/Duel-Commander" itemprop="url">Duel-Commander</a>	</li>
<li itemprop="name">
<a href="/Premodern" itemprop="url">Premodern</a>	</li>
<li itemprop="name">
<a href="/Vintage" itemprop="url">Vintage</a>	</li>
<li itemprop="name">
<a href="/Classic-Legacy" itemprop="url">Classic-Legacy</a>	</li>
<li itemprop="name">
<a href="/Real-Commander" itemprop="url">Real-Commander</a>	</li>
<li class="dropdown-header">More Arena Formats</li>
<li itemprop="name">
<a href="/Alchemy" itemprop="url">Alchemy</a>	</li>
<li itemprop="name">
<a href="/Explorer" itemprop="url">Explorer</a>	</li>
<li itemprop="name">
<a href="/Historic" itemprop="url">Historic</a>	</li>
<li itemprop="name">
<a href="/Timeless" itemprop="url">Timeless</a>	</li>
<li itemprop="name">
<a href="/Brawl" itemprop="url">Brawl</a>	</li>
<li itemprop="name"><a href="/mtgo" itemprop="url">MTGO Decks</a></li>
</ul>
</li>
<li class="col-sm-3">
<ul>
<li class="dropdown-header">Help & Contact</li>
<li><span><span goto="L2NvbnRhY3QvaW5kZXg" class="linker">Contact Us</span></span></li>
<li class="dropdown-header">Tools & Search</li>
<li><span><span goto="L2Zvcm1hdHMvY29zdEFuYWx5c2lz" class="linker">Cost Analysis</span></span></li>
<li><span><span goto="L2RlY2tzL3NlYXJjaA" class="linker">Advanced Search</span></span></li>
<li><a href="/players">Best deckbuilders</a></li>
<li class="dropdown-header">Articles</li>
<li><a href="/articles"><img src="/img/logoSmall2.svg" width="20" alt="MTGdecks" data-pagespeed-url-hash="425135551" data-pagespeed-onload="pagespeed.CriticalImages.checkImageForCriticality(this);" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)"/> MTG Articles</a></li>
<li class="dropdown-header">Contribute to MTG Decks</li>
<li>
<span><span goto="L2V2ZW50cy9hZGQ" class="btn btn-sm btn-info linker" style="color:white;">Submit your event <span class="glyphicon glyphicon-chevron-right transparent"></span></span></span>	<br/>
</li>
</ul>
</li>
</ul>
</li>
</ul>
</nav>
</div><!-- /.nav-collapse -->
</div>
</div>
</div>
<script data-cfasync="false" src="/cdn-cgi/scripts/5c5dd728/cloudflare-static/email-decode.min.js"></script><script defer src="/js/typeahead.bundle.min.js.pagespeed.jm.kNF28sk6nX.js" type="text/psajs" data-pagespeed-orig-index="6"></script>
<script src="/js/bootstrap.min.js+custom.js+base64.min.js.pagespeed.jc.ad0-ykMWZc.js" type="text/psajs" data-pagespeed-orig-index="7"></script><script type="text/psajs" data-pagespeed-orig-index="8">eval(mod_pagespeed_ItsjHfSVxq);</script>	<script type="text/psajs" data-pagespeed-orig-index="9">eval(mod_pagespeed_mqKjjurRp4);</script>	<script type="text/psajs" data-pagespeed-orig-index="10">eval(mod_pagespeed__NM0abgfYF);</script>
<script data-pagespeed-orig-type="text/javascript" type="text/psajs" data-pagespeed-orig-index="11">var a='https://';var c='net';var b='mtgdecks';function mtgdecks(dir){document.location=a+b+'.'+c+'/'+dir+'/';}</script>
<script type="text/psajs" data-pagespeed-orig-index="12">(function(){var link=document.createElement('link');link.href="/css/mana.min.css";link.rel='stylesheet';document.head.appendChild(link);})();</script>
<script type="text/psajs" data-pagespeed-orig-index="13">(function(){var link=document.createElement('link');link.href="/css/font-awesome.min.css";link.rel='stylesheet';document.head.appendChild(link);})();</script>
<!-- Global site tag (gtag.js) - Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XB47XKY0ZZ" type="text/psajs" data-pagespeed-orig-index="14"></script>
<script type="text/psajs" data-pagespeed-orig-index="15">window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','G-XB47XKY0ZZ');</script>
<script async src="https://www.googletagmanager.com/gtag/js?id=G-7T51WR5BCH" type="text/psajs" data-pagespeed-orig-index="16"></script>
<script type="text/psajs" data-pagespeed-orig-index="17">window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','G-7T51WR5BCH');</script>
<script pagespeed_no_defer="" data-pagespeed-no-defer="" defer src='https://static.cloudflareinsights.com/beacon.min.js' data-cf-beacon='{"token": "8120d1cd6d1a442b8d41b849e06765f9"}'></script><!-- End Cloudflare Web Analytics -->
<!-- Twitter conversion tracking base code -->
<link rel="preload" href="//cdn.jsdelivr.net/npm/keyrune@latest/css/keyrune.css" as="style" data-pagespeed-onload="this.onload=null;this.rel='stylesheet'" onload="var elem=this;if (this==window) elem=document.body;elem.setAttribute('data-pagespeed-loaded', 1)">
<noscript>
<link rel="stylesheet" href="//cdn.jsdelivr.net/npm/keyrune@latest/css/keyrune.css">
</noscript>
<script defer src="//images.dmca.com/Badges/DMCABadgeHelper.min.js" type="text/psajs" data-pagespeed-orig-index="18"></script>
<script type="text/psajs" data-pagespeed-orig-index="19">
		isBraveUA().then((result) => {
			if (result) {
				console.log("Brave detected");
			} else {
				console.log("Brave not detected");
			}
		});
	</script>
<script data-pagespeed-orig-type="text/javascript" async src="https://btloader.com/tag?o=5698917485248512&upapi=true&domain=mtgdecks.net" type="text/psajs" data-pagespeed-orig-index="20"></script>
<script type="text/psajs" data-pagespeed-orig-index="21">window.localStorage.clear()
!function(){"use strict";var e;e=document,function(){var t,n;function r(){var t=e.createElement("script");t.src="https://cafemedia-com.videoplayerhub.com/galleryplayer.js",e.head.appendChild(t)}function a(){var t=e.cookie.match("(^|[^;]+)\s*__adblocker\s*=\s*([^;]+)");return t&&t.pop()}function c(){clearInterval(n)}return{init:function(){var e;"true"===(t=a())?r():(e=0,n=setInterval((function(){100!==e&&"false"!==t||c(),"true"===t&&(r(),c()),t=a(),e++}),50))}}}().init()}();</script>
<script type="text/javascript" src="/pagespeed_static/js_defer.I4cHjq6EEP.js"></script><script>(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement('script');d.innerHTML="window.__CF$cv$params={r:'925695da9bc1ed9b',t:'MTc0MjgyMzY3OS4wMDAwMDA='};var a=document.createElement('script');a.nonce='';a.src='/cdn-cgi/challenge-platform/scripts/jsd/main.js';document.getElementsByTagName('head')[0].appendChild(a);";b.getElementsByTagName('head')[0].appendChild(d)}}if(document.body){var a=document.createElement('iframe');a.height=1;a.width=1;a.style.position='absolute';a.style.top=0;a.style.left=0;a.style.border='none';a.style.visibility='hidden';document.body.appendChild(a);if('loading'!==document.readyState)c();else if(window.addEventListener)document.addEventListener('DOMContentLoaded',c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);'loading'!==document.readyState&&(document.onreadystatechange=e,c())}}}})();</script><script defer src="https://static.cloudflareinsights.com/beacon.min.js/vcd15cbe7772f49c399c6a5babf22c1241717689176015" integrity="sha512-ZpsOmlRQV6y907TI0dKBHq9Md29nnaEIPlkf84rnaERnq6zvWvPUqr2ft8M1aS28oN72PdrCzSjY4U6VaAw1EQ==" data-cf-beacon='{"rayId":"925695da9bc1ed9b","serverTiming":{"name":{"cfExtPri":true,"cfL4":true,"cfSpeedBrain":true,"cfCacheStatus":true}},"version":"2025.1.0","token":"c7df624a15514f6ea5aa2234098afc9a"}' crossorigin="anonymous"></script>
</body>
</html>"##;
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
