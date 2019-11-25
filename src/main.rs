pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    } 

pub struct Tweet {
    pub username: String,
    pub content: String,
    } 

pub trait Summary {
    fn summarize(&self) -> String;
} 

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
} 

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let my_tweet = Tweet {
        username: String::from("Muneeb"),
        content: String::from("Second Quater of IOT is Started"),
    }; 
    println!("1 new tweet: {}", my_tweet.summarize()); 

    let my_article = NewsArticle {
        headline: String::from("Australia Won the Match by inning aand 5 runs"),
        author: String::from("Micky Aurther"),
    }; 
    
    println!("\nNew article available! {}", my_article.summarize());  
}