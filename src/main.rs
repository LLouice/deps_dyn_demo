use deps_dyn_demo::deps::{reqwest, serde, serde_json, tokio};

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(crate = "deps_dyn_demo::deps::serde")]
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
        serde_json::from_reader(std::fs::File::open("/Users/llouice/tmp/resp.json").unwrap())
            .expect("Failed to parse JSON from stdin");
    println!("Parsed JSON: {:?}", value);

    let person: Person =
        serde_json::from_reader(std::fs::File::open("/Users/llouice/tmp/Person.json").unwrap())
            .expect("Failed to parse Person JSON");
    println!("Parsed Person: {:?}", person);
}
