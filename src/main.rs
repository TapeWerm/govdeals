use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    let doc = Html::parse_document(&body);

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

    for x in 0..names.len() {
        println!("{}", names[x]);
        println!("Auction close: {} {}", dates[x], times[x]);
        println!("Current bid: {}", prices[x]);
        println!();
    }
    Ok(())
}
