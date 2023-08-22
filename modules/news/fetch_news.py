import bs4
import requests

urls = [
    "https://information.yunlin.gov.tw/News_Content.aspx?n=478&s=307734",
    "https://www.ocac.gov.tw/OCAC/SubSites/Pages/Detail.aspx?site=3117a715-f7a7-4821-ba75-ab93badf8d20&nodeid=1109&pid=52857954",
    "https://news.pchome.com.tw/living/cna/20230716/index-16894841933149418009.html",
    "https://www.allnews.tw/news/44260",
]

titles = []
for url in urls:
    res = requests.get(url)
    soup = bs4.BeautifulSoup(res.text, "html.parser")
    title = title.text.strip() if (title := soup.select_one("title")) else ""
    titles.append(title)

headlines = zip(titles, urls)
with open(".news_headlines", "w") as f:
    for title, url in headlines:
        f.write(f"{title} --- {url}\n")
