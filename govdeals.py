#!/usr/bin/env python3
"""Scrape PSU Surplus GovDeals"""
import requests
from bs4 import BeautifulSoup

HTML1 = requests.get('https://www.govdeals.com/index.cfm?fa=Main.AdvSearchResultsNew&searchPg=Classic&inv_num=&category=00&kWord=&kWordSelect=2&sortBy=ad&agency=7123&state=&country=&locID=&timing=bySimple&locationType=state&timeType=&timingWithin=1')
SOUP = BeautifulSoup(HTML1.text, 'html.parser')

COL1 = SOUP.find_all('div', id="result_col_1")
COL4 = SOUP.find_all('div', id="result_col_4")
COL5 = SOUP.find_all('div', id="result_col_5")

for x in COL1:
    print(x.find('div', {'class': 'highslide-caption'}).contents[0].contents[0])
for x in COL4:
    print(x.find('label').contents[0].strip(), x.find_all('span')[1].contents[0])
for x in COL5:
    print(x.find('span', id="bid_price").contents[0].strip())
