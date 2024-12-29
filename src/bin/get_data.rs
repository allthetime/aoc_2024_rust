use std::{fs, env};
use std::sync::Arc;
use reqwest::{cookie::Jar, Url};
use std::path::PathBuf;

const SESSION: &str = "53616c7465645f5f3665783a507693c03e5b0fb8ec9c115697b67168fb7b1cda906c4f4e44d9abe772d59d30d1584f17ff7c0cb7bd4ceca2f7e859d436ae28de";
const RU: &str = "53616c7465645f5f74063ec40c57acb1788aacdb6d3d3a5725ad2acbdb264a2e";
const YEAR: i32 = 2024;

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {

    let cookie = format!("session={SESSION}; ru={RU}; Domain=adventofcode.com");
    let url = "https://adventofcode.com".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()?;

    let resp = client.get(url).send().await?
        .text().await?;

    let day_fs_string = format!("00{}", day);
    let str_len = day_fs_string.len();
    let dfs = &day_fs_string.as_str()[str_len-3..];

    let data = resp.trim();

    dbg!(dfs);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src");
    d.push(format!("day{}",dfs));
    d.push("data");
    
    dbg!(&d);

    println!("https://adventofcode.com/2024/day/{}", dfs.parse::<usize>().unwrap());

    fs::write(d, data).expect("Unable to write file");

    Ok(())
}
