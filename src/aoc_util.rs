use std::{
    env::{self, args},
    fmt::Display,
    fs::{self, File, metadata, read_to_string},
    io::{self, Write},
    process::{Command, Stdio},
    sync::Mutex,
    time::{Instant, SystemTime},
};

use owo_colors::OwoColorize;

use crate::{DEBUG, ExtraItertools, bar, fetch, read_clipboard};

fn write_atomic(filename: &str, data: &str) {
    let tmp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let tmp = format!("{filename}.{}", tmp);
    File::create_new(&tmp)
        .unwrap()
        .write_all(data.as_bytes())
        .unwrap();
    fs::rename(tmp, filename).unwrap();
}

static SUBMITTED: Mutex<bool> = Mutex::new(false);

static START_TS: Mutex<Option<Instant>> = Mutex::new(None);

fn day() -> u8 {
    let exe = args().next().unwrap();
    exe.rsplit('/').next().unwrap().parse::<u8>().unwrap()
}

pub fn load_input() -> String {
    better_panic::install();

    let input = if DEBUG {
        let sample = read_to_string(format!("src/bin/{}.sample.txt", day())).unwrap();
        if sample.trim().is_empty() {
            println!("sample input file is empty");
            println!("{}", "reading sample input from clipboard!!".red().bold());
            read_clipboard().unwrap()
        } else {
            println!("{}", "using saved sample input".blue().bold());
            sample
        }
    } else {
        let url = format!("https://adventofcode.com/2024/day/{}/input", day());
        let path = format!("target/{}.input.txt", day());
        let input = match read_to_string(&path) {
            Ok(x) => x,
            Err(e) => {
                dbg!(&e);
                print!("Downloading input... ");
                io::stdout().flush().unwrap();
                match fetch(&url) {
                    Ok(input) => {
                        write_atomic(&path, &input);
                        println!("done!");
                        input
                    }
                    Err(e) => {
                        dbg!(e);
                        println!("testing session cookie...");
                        assert!(
                            fetch("https://adventofcode.com/2024")
                                .unwrap()
                                .contains("[Log Out]")
                        );
                        panic!("cookie works, input missing!")
                    }
                }
            }
        };
        let html_path = format!("target/{}.html", day());
        let submitted = match metadata(&html_path) {
            Ok(_) => true,
            Err(_) => {
                let page = fetch(&format!("https://adventofcode.com/2024/day/{}", day())).unwrap();
                write_atomic(&format!("target/{}-pre.html", day()), &page);
                if page.contains(
                    "Both parts of this puzzle are complete! They provide two gold stars: **",
                ) {
                    write_atomic(&html_path, &page);
                    true
                } else {
                    false
                }
            }
        };
        *SUBMITTED.lock().unwrap() = submitted;
        input
    };

    println!(
        "loaded input: {} chars, {} lines, {} paras",
        input.len(),
        input.lines().count(),
        input.split("\n\n").count()
    );

    let mut lines = input.lines();
    if let Some(line) = lines.next() {
        println!("{}", line.blue());
    }
    let last = lines.next_back();
    if lines.next().is_some() {
        println!("(... {} more lines)", lines.count() + 1);
    }
    if let Some(line) = last {
        println!("{}", line.blue());
    }
    bar();

    *START_TS.lock().unwrap() = Some(Instant::now());

    input
}

pub fn cp(x: impl Display) {
    let elapsed = START_TS.lock().unwrap().unwrap().elapsed();
    let elapsed = format!("{:?}", elapsed);

    let x = x.to_string();

    static COPIES: Mutex<usize> = Mutex::new(0);
    let mut copies = COPIES.lock().unwrap();
    if *copies >= 2 {
        println!("value: {}", x.red().bold());
        panic!("already copied twice");
    }
    *copies += 1;

    if DEBUG {
        let page = fs::read_to_string(format!("target/{}-pre.html", day()));
        match page {
            Ok(page) => {
                let page = page.split("<body>").last().unwrap();
                match [&format!(">{}<", x), &x].into_iter().find_map(|x| {
                    let x = page.match_indices(x).cv();
                    if !x.is_empty() { Some(x) } else { None }
                }) {
                    Some(m) => {
                        println!(
                            "value: {} ({}) took {}",
                            x.bold().green(),
                            "found".green(),
                            elapsed.yellow()
                        );
                        for (i, x) in m.into_iter().take(5) {
                            let j = i + x.len();
                            let begin = i.saturating_sub(30);
                            let end = (j + 30).min(page.len());
                            println!(
                                "... {}{}{} ...",
                                &page[begin..i].replace('\n', ""),
                                x.green().bold(),
                                &page[j..end].replace('\n', "")
                            );
                        }
                    }
                    None => {
                        println!(
                            "value: {} ({}) took {}",
                            x.yellow().bold(),
                            "not found".red(),
                            elapsed.yellow()
                        );
                    }
                }
            }
            Err(e) => {
                dbg!(&e);
                println!(
                    "value: {} (unknown result) took {}",
                    x.blue().bold(),
                    elapsed.yellow()
                );
            }
        }
    } else if *SUBMITTED.lock().unwrap() {
        let page_html = read_to_string(format!("target/{}.html", day())).unwrap();
        let mut correct_answers = vec![];
        for line in page_html.lines() {
            if let Some(line) = line.strip_prefix("<p>Your puzzle answer was <code>") {
                let (line, _) = line.split_once("</code>.</p>").unwrap();
                correct_answers.push(line.to_string());
            }
        }
        if correct_answers[*copies - 1] == x {
            println!(
                "value: {} (correct!) took {}",
                x.green().bold(),
                elapsed.yellow()
            );
        } else {
            println!(
                "value: {} (incorrect answer) took {}",
                x.red().bold(),
                elapsed.yellow()
            );
        }
    } else if env::var("AOC_COPY_CLIPBOARD").is_ok() {
        force_copy(&x);
        println!(
            "value: {} (copied to clipboard) took {}",
            x.green().bold(),
            elapsed.yellow()
        );
    } else {
        println!(
            "value: {} (set AOC_COPY_CLIPBOARD=1 to enable copy) took {}",
            x.green().bold(),
            elapsed.yellow()
        );
    }

    *START_TS.lock().unwrap() = Some(Instant::now());
}

pub fn force_copy(x: &impl Display) {
    // Copy it twice to work around a bug.
    for _ in 0..2 {
        let mut cmd = Command::new("xclip")
            .arg("-sel")
            .arg("clip")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = cmd.stdin.take().unwrap();
        stdin.write_all(x.to_string().as_bytes()).unwrap();
        stdin.flush().unwrap();
        drop(stdin);
        cmd.wait().unwrap();
    }
}

pub fn cp1(x: impl Display) {
    cp(x);
    panic!("exiting after copy")
}
