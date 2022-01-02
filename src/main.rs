use htmlescape::decode_html;
use scraper::{Html, Selector};

struct Deal {
    item: String,
    // Auction closes on date at time
    date: String,
    time: String,
    price: String,
    bids: u32,
    picture: String,
}

/// PSU Surplus GovDeals parser
fn parse(doc: scraper::Html) -> Vec<Deal> {
    let srows = Selector::parse("#boxx_row").unwrap();
    let rows = doc.select(&srows);
    let mut deals: Vec<Deal> = Vec::new();
    for row in rows {
        let scol1 = Selector::parse("#result_col_1").unwrap();
        let spicture = Selector::parse("a.highslide").unwrap();
        let col1 = row.select(&scol1).next().unwrap();
        let picture = col1
            .select(&spicture)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();

        let scol2 = Selector::parse("#result_col_2").unwrap();
        let sitem = Selector::parse("a").unwrap();
        let col2 = row.select(&scol2).next().unwrap();
        let item_raw = col2.select(&sitem).next().unwrap().inner_html();
        let item = decode_html(&item_raw).unwrap();

        let scol4 = Selector::parse("#result_col_4").unwrap();
        let sdate = Selector::parse("label").unwrap();
        let stime = Selector::parse("label span").unwrap();
        let col4 = row.select(&scol4).next().unwrap();
        let date = col4
            .select(&sdate)
            .next()
            .unwrap()
            .inner_html()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        let time = col4.select(&stime).next().unwrap().inner_html();

        let scol5 = Selector::parse("#result_col_5").unwrap();
        let sprice = Selector::parse("#bid_price").unwrap();
        let col5 = row.select(&scol5).next().unwrap();
        let inprice = col5.select(&sprice).next().unwrap().inner_html();
        let price = inprice.split_whitespace().next().unwrap().to_string();
        let bids = inprice
            .split_whitespace()
            .nth(2)
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap();

        deals.push(Deal {
            item,
            date,
            time,
            price,
            bids,
            picture,
        });
    }

    // Recurse to next page
    let spagebar = Selector::parse("#pagination_1").unwrap();
    let spages = Selector::parse("ul li a").unwrap();
    // There's a duplicate pagebar at the bottom
    let nextbar = doc.select(&spagebar).next();
    if let Some(topbar) = nextbar {
        let pages = topbar.select(&spages);
        for page in pages {
            // >> next page button
            if page.inner_html() == "&gt;&gt;" {
                let mut nextpage: String = "https://www.govdeals.com/".to_owned();
                nextpage.push_str(page.value().attr("href").unwrap());
                let body = reqwest::blocking::get(nextpage).unwrap().text().unwrap();
                let recurse = Html::parse_document(&body);
                let r = parse(recurse);
                for x in r {
                    deals.push(x);
                }
            }
        }
    }

    deals
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1")?
        .text()?;
    let doc = Html::parse_document(&body);

    let r = parse(doc);
    for x in r {
        println!("Item: {}", x.item);
        println!("Auction Close: {} {}", x.date, x.time);
        println!("Current Bid: {} {} Bids", x.price, x.bids);
        println!("Picture: https://www.govdeals.com{}", x.picture);
        println!();
    }
    Ok(())
}

#[test]
fn test_parse() {
    let body = r##"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <div id="pagination_1">
        <ul>
            <li><a href="#">1</a></li>
        </ul>
    </div>
    <div id="boxx_row">
        <div id="result_col_1">
            <a href="/photos/7123/7123_746_7.jpg" class="highslide">img</a>
        </div>
        <div id="result_col_2">
            <a href="index.cfm?fa=Main.Item&amp;itemid=746&amp;acctid=7123">Pallet of Dell Computers, HP Printers, etc.</a>
        </div>
        <div id="result_col_4">
            <label>7/9/2021 &nbsp;&nbsp;
                <span>7:30 PM ET</span>
            </label>
        </div>
        <div id="result_col_5">
            <span id="bid_price">$1,090.00
                <br>&nbsp;&nbsp;Bids:  27
            </span>
        </div>
    </div>
    <div id="boxx_row">
        <div id="result_col_1">
            <a href="/photos/7123/7123_746_7.jpg" class="highslide">img</a>
        </div>
        <div id="result_col_2">
            <a href="index.cfm?fa=Main.Item&amp;itemid=746&amp;acctid=7123">Pallet of Dell Computers, HP Printers, & etc.</a>
        </div>
        <div id="result_col_4">
            <label>7/9/2021 &nbsp;&nbsp;
                <span>7:30 PM ET</span>
            </label>
        </div>
        <div id="result_col_5">
            <span id="bid_price">$1,090.00</span>
        </div>
    </div>
    "##;
    let doc = Html::parse_document(&body);
    let r = parse(doc);
    assert_eq!(r[0].item, "Pallet of Dell Computers, HP Printers, etc.");
    assert_eq!(r[0].date, "7/9/2021");
    assert_eq!(r[0].time, "7:30 PM ET");
    assert_eq!(r[0].price, "$1,090.00");
    assert_eq!(r[0].bids, 27);
    assert_eq!(r[0].picture, "/photos/7123/7123_746_7.jpg");
    assert_eq!(r[1].item, "Pallet of Dell Computers, HP Printers, & etc.");
    assert_eq!(r[1].date, "7/9/2021");
    assert_eq!(r[1].time, "7:30 PM ET");
    assert_eq!(r[1].price, "$1,090.00");
    assert_eq!(r[1].bids, 0);
    assert_eq!(r[1].picture, "/photos/7123/7123_746_7.jpg");
}
