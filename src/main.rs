use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    let doc = Html::parse_document(&body);
    let scol1 = Selector::parse("#result_col_1").unwrap();
    let col1 = doc.select(&scol1);
    let sname = Selector::parse("div.highslide-caption a").unwrap();
    let mut names: Vec<String> = Vec::new();
    for x in col1 {
        let name = x.select(&sname).next().unwrap().inner_html();
        names.push(name);
    }
    for x in 0..names.len() {
        println!("{}", names[x]);
    }
    Ok(())
}
