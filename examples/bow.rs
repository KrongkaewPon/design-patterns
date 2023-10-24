trait CrawlerParameter {
    fn get_api_token(&self) -> String;
    fn get_url(&self) -> String;
}

struct BowCrawlerParameter {}
impl CrawlerParameter for BowCrawlerParameter {
    fn get_api_token(&self) -> String {
        "token".to_string()
    }
    fn get_url(&self) -> String {
        "backend".to_string()
    }
}

struct SGCrawlerParameter {}
impl CrawlerParameter for SGCrawlerParameter {
    fn get_api_token(&self) -> String {
        "sg_token".to_string()
    }
    fn get_url(&self) -> String {
        "sg_backend".to_string()
    }
}

struct Crawler {
    api_token: String,
    url: String,
    crawler_parameter: Box<dyn CrawlerParameter>,
}

impl Crawler {
    fn new(crawler_parameter: Box<dyn CrawlerParameter>) -> Self {
        Crawler {
            api_token: "api_token".into(),
            url: "url".into(),
            crawler_parameter,
        }
    }
}

trait Crawl {
    fn prepare(&mut self);
    fn fetch(&mut self) {
        self.prepare();
        println!("Fetch!!");
    }
}
impl Crawl for Crawler {
    fn prepare(&mut self) {
        self.api_token = self.crawler_parameter.get_api_token();
        self.url = self.crawler_parameter.get_url()
    }
    fn fetch(&mut self) {
        self.prepare();
        println!("api_token!! {}", self.api_token);
    }
}
fn main() {
    let mut crawl = Crawler::new(Box::new(BowCrawlerParameter {}));
    crawl.fetch();
    // crawl
}
