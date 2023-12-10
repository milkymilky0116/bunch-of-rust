use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use scraper::Html;
struct PokemonProduct {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}
const BASE_URL: &str = "https://scrapeme.live/shop";
fn main() {
    let total_pages = get_pages(&BASE_URL.to_string());
    let products: Vec<PokemonProduct> = vec![];

    let products_mutex = Arc::new(Mutex::new(products));

    let (tx, rx) = mpsc::channel::<Vec<PokemonProduct>>();
    let mut handles = vec![];
    for i in 1..total_pages + 1 {
        let tx_clone = mpsc::Sender::clone(&tx);
        let handle = thread::spawn(move || {
            let url = format!("{}/page/{}/", BASE_URL, i);
            println!("Scraping: {}", url);
            let pokemon_product = scrap_page(&url);
            tx_clone.send(pokemon_product).unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let receives_products: Vec<PokemonProduct> = rx.try_iter().flatten().collect();

    let mut locked_products = products_mutex.lock().unwrap();
    locked_products.extend(receives_products);
    write_csv(&locked_products);
}

fn parse_html_document(url: &String) -> Html {
    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    document
}

fn get_pages(url: &String) -> i32 {
    let document = parse_html_document(url);
    let page_selector = scraper::Selector::parse("ul.page-numbers li:nth-last-child(2)").unwrap();
    let html_pages = document
        .select(&page_selector)
        .next()
        .map(|li| li.text().collect::<String>())
        .unwrap()
        .parse::<i32>()
        .unwrap();

    html_pages
}

fn scrap_page(url: &String) -> Vec<PokemonProduct> {
    let document = parse_html_document(url);
    let product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&product_selector);

    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    for product in html_products {
        let url = product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|a| a.value().attr("src"))
            .map(str::to_owned);

        let name = product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        let price = product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };

        pokemon_products.push(pokemon_product);
    }

    pokemon_products
}

fn write_csv(products: &Vec<PokemonProduct>) {
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["index", "url", "image", "name", "price"])
        .unwrap();

    for (i, product) in products.iter().enumerate() {
        let url = product.url.as_ref().unwrap();
        let image = product.image.as_ref().unwrap();
        let name = product.name.as_ref().unwrap();
        let price = product.price.as_ref().unwrap();

        writer
            .write_record(&[&i.to_string(), url, image, name, price])
            .unwrap();
    }
    writer.flush().unwrap();
}

//https://weworkremotely.com/remote-jobs/search?term=python
