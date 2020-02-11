use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut show = Show {
        id: 650,
        page: 0
    };

    let mut data = true;
    let mut all_magnets: Vec<String> = Vec::new();

    let selector = Selector::parse(".link-1080p > .hs-magnet-link > a").unwrap();

    while data {
        let res = reqwest::get(&show.url()).await?;
        let body = res.text().await?;
        let parsed = Html::parse_document(&body);
        let selected = parsed.select(&selector);

        let mut magnets = selected
            .map(|it| it.value().attr("href").unwrap().to_string())
            .collect::<Vec<_>>();

        data = !magnets.is_empty();
        all_magnets.append(&mut magnets);

        show.next_page()
    }

    for magnet in all_magnets {
        println!("{}", magnet);
    }
    Ok(())
}

struct Show {
    id: i32,
    page: i8
}

impl Show {
    fn next_page(&mut self) {
        self.page += 1;
    }

    fn url(&self) -> String {
        return format!("https://horriblesubs.info/api.php?method=getshows&type=show&showid={id}&nextid={page}",
                       id = self.id, page = self.page);
    }
}