use std::error::Error;

use scraper::{Html, Selector};
use sublime_fuzzy::best_match;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn search_lyrics(title: &str, author: &str) -> Result<String> {
    let url_base = format!("https://www.musixmatch.com/search/{}/lyrics", title);
    let mut page = 1;

    let mut results: Vec<(isize, String)> = vec![];

    let empty_selector = Selector::parse("div.empty").unwrap();
    let tracks_selector = Selector::parse("ul.tracks").unwrap();
    let link_selector = Selector::parse("h2 > a").unwrap();
    let title_selector = Selector::parse("h2 > a > span").unwrap();
    let author_selector = Selector::parse("h3 > span > span > a").unwrap();
    loop {
        let url = format!("{}/{}", url_base, page);
        let response = reqwest::blocking::get(url)?;

        let html = response.text()?;
        let document = Html::parse_document(&html);

        let has_tracks = document.select(&empty_selector).count() == 0;
        if !has_tracks {
            break;
        }

        let tracks = document.select(&tracks_selector).next().unwrap();

        for track in tracks.select(&Selector::parse("li").unwrap()) {
            let track_title = track.select(&title_selector).next().unwrap().inner_html();
            let track_author = track.select(&author_selector).next().unwrap().inner_html();
            let title_match = match best_match(title, &track_title) {
                Some(m) => m.score(),
                None => 0,
            };
            let author_match = match best_match(author, &track_author) {
                Some(m) => m.score(),
                None => 0,
            };

            let total_score = title_match + author_match;
            let mut link = track
                .select(&link_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_owned();
            link.remove(0); // Removing / char

            results.push((total_score, link));
        }

        page += 1;
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(results[0].1.clone())
}

fn main() -> Result<()> {
    Ok(())
}
