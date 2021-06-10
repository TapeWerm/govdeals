CS 410 Rust Course Project: PSU Surplus GovDeals Scraper

By: Vincent Vermilya
# Description
My friend rattboi made [a cool bot](https://github.com/rattboi/surplus-bot) that scraped the old [PSU Surplus](https://www.pdx.edu/surplus/) website and posted new items to IRC. The 2 problems with it is it is a slow Python program and PSU Surplus dumped its old site for GovDeals and will use a new site after the pandemic. I decided to make a mediocre new web scraper for the temporary site that just so happens to count towards completing a class. Hurrah.

`cargo run` to print current auctions.

`cargo test` to run my inadequate test to meet assignment requirements.
# [LICENSE](LICENSE)
# Setup
[https://docs.rs/openssl/0.10.34/openssl/#automatic](https://docs.rs/openssl/0.10.34/openssl/#automatic)
# Development
My Minecraft server scripts broke recently because Mojang's CDN Akamai decided wget was abusive.
My scripts polled Mojang's webpage hourly to see if the current server zipfile name matched the one already downloaded.
The web scraping is pretty basic, it just greps for a regex match to the zipfile download link.
Supplying any string as a user agent is a valid workaround.
`Hello world`, `Mozilla Firefox`, `IE6`, Rick roll links, anything goes.
But if you don't supply a user agent then you must be a hacker, I don't make the rules.
I also made a proper web scraper in the CAT for checking backup report emails to make sure they aren't on fire.
It is written in Python 3 and uses Beautiful Soup.

Rust and relevant crates don't give a lot of documentation to help you get running.
I recognized CSS selectors but I was thrown off by the amount of debug info printed.
For a few minutes I tried to read it, thinking it was catching too much with the CSS selector.
Then I got smart and settled for printing "yolo" each loop iteration.
8 yolos matched the expected 8 rows I was trying to scrape.
I continued reading documentation and Googling until I found the `inner_html` function which gave me the text I craved.
From there I tried to brush up on my bad Rust string skills so my code didn't completely suck.
It's not great code, I'd tweak it to handle empty fields at the minimum, maybe even rewrite it.
Adding bid count, a database to identify when items are added, and Discord webhook support are all needed for this to be more than a toy program.
It doesn't really matter to me anyways, GovDeals is supposed to be temporary until PSU Surplus reopens later this year.
