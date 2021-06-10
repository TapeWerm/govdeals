use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    let doc = Html::parse_document(&body);
    let select1 = Selector::parse("#result_col_1").unwrap();
    let col1 = doc.select(&select1);
    let select2 = Selector::parse("div.highslide-caption a").unwrap();
    for x in col1 {
        println!("{:#?}", x.select(&select2).next().unwrap().inner_html());
    }
    Ok(())
}
