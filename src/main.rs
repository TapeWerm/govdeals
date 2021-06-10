use scraper::Html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    println!("{:#?}", body);
    Ok(())
}
