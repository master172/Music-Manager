use std::path::Path;
use std::process::Command;
use std::thread;

#[derive(Debug)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub fn search_youtube(query: &str, limit: usize) -> Vec<SearchResult> {
    let yt_dlp_path = Path::new("libs/yt-dlp.exe");

    let search_arg = format!("ytsearch{}:{}", limit, query);

    let output = Command::new(yt_dlp_path)
        .args([
            &search_arg,
            "--print",
            "%(title)s|%(webpage_url)s",
            "--no-playlist",
        ])
        .output()
        .expect("failed to run yt-dlp search");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '|');
            Some(SearchResult {
                title: parts.next()?.to_string(),
                url: parts.next()?.to_string(),
            })
        })
        .collect()
}

pub fn search_youtube_async(
    query: String,
    limit: usize,
    callback: impl Fn(Vec<SearchResult>) + Send + 'static,
) {
    thread::spawn(move || {
        let results = search_youtube(&query, limit);
        callback(results);
    });
}

//pub fn search(query: &str, limit: usize) {
//    let results = search_youtube(query, limit);
//    println!("Search completed!");
//    for r in results {
//        println!("{} -> {}", r.title, r.url);
//    }
//}

pub fn search_async(query: &str, limit: usize) {
    search_youtube_async(query.into(), limit, |results| {
        println!("Search completed!");
        for r in results {
            println!("{} -> {}", r.title, r.url);
        }
    });
}
