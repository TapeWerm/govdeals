use scraper::{Html, Selector};

/// Table of auction items
struct Deals {
    names: Vec<String>,
    /// Expiry date
    dates: Vec<String>,
    /// Expiry time
    times: Vec<String>,
    prices: Vec<String>,
}

/// PSU Surplus GovDeals parser
fn parse(doc: scraper::Html) -> Deals {
    let scol1 = Selector::parse("#result_col_1").unwrap();
    let scol4 = Selector::parse("#result_col_4").unwrap();
    let scol5 = Selector::parse("#result_col_5").unwrap();
    let col1 = doc.select(&scol1);
    let col4 = doc.select(&scol4);
    let col5 = doc.select(&scol5);

    let sname = Selector::parse("div.highslide-caption a").unwrap();
    let mut names: Vec<String> = Vec::new();
    for x in col1 {
        let name = x.select(&sname).next().unwrap().inner_html();
        names.push(name);
    }

    let sdate = Selector::parse("label").unwrap();
    let stime = Selector::parse("label span").unwrap();
    let mut dates: Vec<String> = Vec::new();
    let mut times: Vec<String> = Vec::new();
    for x in col4 {
        let inner = x.select(&sdate).next().unwrap().inner_html();
        let date = inner.split_whitespace().next().unwrap().to_string();
        dates.push(date);
        let time = x.select(&stime).next().unwrap().inner_html();
        times.push(time);
    }

    let sprice = Selector::parse("#bid_price").unwrap();
    let mut prices: Vec<String> = Vec::new();
    for x in col5 {
        let inner = x.select(&sprice).next().unwrap().inner_html();
        let price = inner.split_whitespace().next().unwrap().to_string();
        prices.push(price);
    }

    Deals {
        names,
        dates,
        times,
        prices,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    let doc = Html::parse_document(&body);

    let r = parse(doc);
    for x in 0..r.names.len() {
        println!("{}", r.names[x]);
        println!("Auction close: {} {}", r.dates[x], r.times[x]);
        println!("Current bid: {}", r.prices[x]);
        println!();
    }
    Ok(())
}

#[test]
fn test_parse() {
    let body = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <div id="result_col_1"  class="col-4 col-sm-4 col-md-4 col-lg-2 col-xl-1 d-flex justify-content-start">
        <a href="/photos/7123/7123_721_7.jpg" class="highslide" onClick="return hs.expand(this,{captionId: 'caption1'})" title="Temptronic Titan SA148440 Thermochuck Chiller for EG4/200 AS-IS">
        <IMG src="/photos/7123/Thumbnails/7123_721_7.jpg" style="border-color:#999; margin-bottom:15px; margin-top:5px;" hspace="3" alt="Temptronic Titan SA148440 Thermochuck Chiller for EG4/200 AS-IS"></a>
        <div class="highslide-caption" id="caption1"><a href="index.cfm?fa=Main.Item&itemid=721&acctid=7123">Temptronic Titan SA148440 Thermochuck Chiller for EG4/200 AS-IS</a></div>
    </div>
    <div id="result_col_4" name="result_col_4_1"  class="col-10 col-sm-10 col-md-10 col-lg-2 col-xl-2 px-1">
        <span id="auct_lbl_srch" style="font-weight:bold;padding-left:10px;">Auction Close:</span>
        <label for="shortcut1" style="padding-left:10px;">6/14/2021 &nbsp;&nbsp;
        <span style="white-space:nowrap">7:30 PM ET</label></span>
    </div>
    <div align="center" id="result_col_5" name="result_col_5_1"  class="col-10 col-sm-10 col-md-10 col-lg-1 col-xl-2 px-1 py-1">
        <span id="bid_lbl_srch" style="font-weight:bold;padding-left:10px;">Current Bid:&nbsp;</span>
        <span id="bid_price">
            $800.00
        </span>
    </div>
    "#;
    let doc = Html::parse_document(&body);
    let r = parse(doc);
    assert_eq!(
        r.names[0],
        "Temptronic Titan SA148440 Thermochuck Chiller for EG4/200 AS-IS"
    );
    assert_eq!(r.dates[0], "6/14/2021");
    assert_eq!(r.times[0], "7:30 PM ET");
    assert_eq!(r.prices[0], "$800.00");
}
