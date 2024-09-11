use traits::{self, Summary, Tweet};

fn main() {
	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course, as you probably already know, people"),
		reply: true,
		retweet: false,
	};

	println!("1 new tweet: {}", tweet.summarize());
}
