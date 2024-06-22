pub struct Article {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize (&self) -> String {
        return format!("{} by {}", self.headline, self.author)
    }

    fn summarize_auth (&self) -> String {
        return format!("{}", self.author)
    }
}

pub struct Tweet {
    pub author: String,
    pub reply: bool,
    pub retweet: bool,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_auth (&self) -> String {
        return format!("{}", self.author)
    }
}

pub trait Summary {
    fn summarize_auth (&self) -> String;

    fn summarize (&self) -> String {
        return format!("(Read more by : {})", self.summarize_auth())
    }
}