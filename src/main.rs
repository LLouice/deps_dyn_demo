use dyn_test::deps::{reqwest, serde, serde_json, tokio};

#[derive(Debug, serde::Deserialize)]
#[serde(crate = "dyn_test::deps::serde")]
struct Person {
    name: String,
    age: u32,
    phones: Vec<String>,
}

#[tokio::main]
async fn main() {
    let res = reqwest::get("https://www.rust-lang.org")
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to read response text");
    println!("Response: {}", res);

    let value: serde_json::Value =
        serde_json::from_reader(std::fs::File::open("../resp.json").unwrap())
            .expect("Failed to parse JSON from stdin");
    println!("Parsed JSON: {:?}", value);

    let person: Person = serde_json::from_reader(std::fs::File::open("../Person.json").unwrap())
        .expect("Failed to parse Person JSON");
    println!("Parsed Person: {:?}", person);
}
