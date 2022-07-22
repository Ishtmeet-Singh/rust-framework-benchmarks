use lazy_static::lazy_static;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::{
    fs,
    process::{Child, Command},
    time::Duration,
};

static BENCHMARK_SETTINGS: [Settings; 4] = [
    Settings {
        concurrency: 10,
        threads: 1,
        duration: 30,
    },
    Settings {
        concurrency: 50,
        threads: 1,
        duration: 30,
    },
    Settings {
        concurrency: 100,
        threads: 1,
        duration: 30,
    },
    Settings {
        concurrency: 500,
        threads: 1,
        duration: 30,
    },
];

static FRAMEWORK_SETTINGS: &str = include_str!("../config.json");
struct Settings {
    concurrency: u32,
    threads: u8,
    duration: u32, // seconds
}

#[derive(Debug, Clone)]
struct Stats {
    requests_per_second: f64,
    name: String,
    average_latency: String,
    max_latency: String,
    total_requests: u64,
    concurrency: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Framework {
    name: &'static str,
    port: u32,
    binary: &'static str,
    url: &'static str,
}

impl Framework {
    fn print_log(&self, settings: &Settings) {
        println!(
            "\n[{}] - Concurrency ({}) | Threads ({}) | Duration ({})",
            self.name, settings.concurrency, settings.threads, settings.duration
        )
    }

    #[must_use = "Require handle to kill it once the benchmark finishes"]
    fn start_server(&self) -> Child {
        match Command::new(format!("target/release/{}", self.binary))
            .arg(format!("{}", self.port))
            .spawn()
        {
            Ok(handle) => handle,
            Err(err_message) => panic!("{}", err_message.to_string()),
        }
    }

    async fn run_benchmark(&self) {
        // start the framework's server
        let mut server_handle = self.start_server();

        // kill server if there's an error while creating a directory
        if let Err(err_message) = fs::create_dir_all(format!("perf/{}", self.binary)) {
            server_handle.kill().unwrap();
            println!(
                "\n[ERROR] Couldn't create directory (perf/{}) : {}",
                self.name, err_message
            );
            std::process::exit(-1);
        };

        BENCHMARK_SETTINGS.iter().for_each(|setting| {
            self.print_log(setting);

            // wait 1 sec till the server starts running (some servers take more time to start - for example tide)
            std::thread::sleep(Duration::from_secs(1));

            let wrk_handle = Command::new("wrk")
                .arg(format!("-d{}s", setting.duration))
                .arg(format!("-t{}", setting.threads))
                .arg(format!("-c{}", setting.concurrency))
                .arg(format!("http://localhost:{}", self.port))
                .output();
            let wrk_output = wrk_handle.unwrap();

            // kill server if there's an error while writing `wrk` output to the file
            if let Err(err_message) = fs::write(
                format!("perf/{}/{}.txt", self.binary, setting.concurrency),
                wrk_output.stdout,
            ) {
                server_handle.kill().unwrap();
                println!("\n[ERROR] Couldn't write to file: {}", err_message);
                std::process::exit(-1);
            }
            // wait a bit to free system resources
            std::thread::sleep(Duration::from_secs(1));
        });

        if let Err(err_message) = server_handle.kill() {
            println!("\nFailed to kill {} server.\n{}", self.name, err_message);
            std::process::exit(-1);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut frameworks = parse_frameworks();
    for (index, current_framework) in frameworks.iter().enumerate() {
        println!("Progress: {}/{}", index + 1, frameworks.len());
        current_framework.run_benchmark().await;
    }

    let sorted_frameworks = sort_framework(&mut frameworks);
    write_markdown(&sorted_frameworks);
    write_readme(&frameworks);
}

fn write_readme(frameworks: &Vec<Framework>) {
    let split_string: Vec<&str> = READ_ME_STRING.split("==SPLIT==").collect();
    let mut markdown_content = String::new();

    writeln!(&mut markdown_content, "## Frameworks included").unwrap();
    for framework in frameworks {
        writeln!(
            &mut markdown_content,
            "**[{}]({})**",
            framework.name, framework.url
        )
        .unwrap();
    }
    writeln!(&mut markdown_content, "# Results").unwrap();
    BENCHMARK_SETTINGS.iter().for_each(|curr| {
        let current_result =
            fs::read_to_string(format!("results-{}.md", curr.concurrency)).unwrap();

        // |   Concurrency: 10   |   Duration: 20 secs   |   Threads: 1   |
        // |:-------------------:|:---------------------:|:--------------:|
        writeln!(
            &mut markdown_content,
            "|   Concurrency: {}   |   Duration: {} secs   |   Threads: {}   |",
            curr.concurrency, curr.duration, curr.threads
        )
        .unwrap();
        writeln!(
            &mut markdown_content,
            "|:-------------------:|:---------------------:|:--------------:|\n",
        )
        .unwrap();
        writeln!(&mut markdown_content, "{}\n", current_result).unwrap();
    });

    let new_md = format!(
        "{}\n{}\n{}",
        split_string[0], markdown_content, split_string[1]
    );
    fs::write("./readme.md", new_md).unwrap();
}

fn write_markdown(sorted_frameworks: &[Vec<Stats>]) {
    for batches in sorted_frameworks.iter() {
        let concurrency = batches[0].concurrency.to_string();
        let mut markdown_string = String::new();
        writeln!(&mut markdown_string, "{}", MARKDOWN_HEADER).unwrap();

        for framework_stat in batches {
            writeln!(
                &mut markdown_string,
                "|**{}**|{}|{}|{}|{}|",
                framework_stat.name,
                (framework_stat.requests_per_second as u64).to_formatted_string(&Locale::en),
                framework_stat.average_latency,
                framework_stat.max_latency,
                (framework_stat.total_requests as u64).to_formatted_string(&Locale::en)
            )
            .unwrap();
        }
        fs::write(format!("./results-{}.md", concurrency), markdown_string).unwrap();
    }
}

fn parse_frameworks() -> Vec<Framework> {
    serde_json::from_str(FRAMEWORK_SETTINGS).unwrap()
}

fn calculate_results(frameworks: &[Framework]) -> Vec<Stats> {
    let mut statistics: Vec<Stats> =
        Vec::with_capacity(BENCHMARK_SETTINGS.len() * frameworks.len());
    // for every setting type, fetch all frameworks stats
    for setting in &BENCHMARK_SETTINGS {
        for framework in frameworks {
            let benchmark_result = fs::read_to_string(format!(
                "perf/{}/{}.txt",
                framework.binary, setting.concurrency
            ))
            .unwrap();
            lazy_static! {
                static ref LATENCY_RGX: Regex =
                    Regex::new(r"Latency((\s)*[0-9]*.[0-9]*[a-z]*){3}").unwrap();
                static ref TOTAL_REQUESTS_RGX: Regex = Regex::new(r"[0-9]+ requests in").unwrap();
                static ref REQUESTS_PER_SECOND_RGX: Regex =
                    Regex::new(r"Requests/sec: [0-9]*.[0-9]*").unwrap();
            }
            let latency_string: String = LATENCY_RGX
                .find_iter(&benchmark_result)
                .map(|mat| mat.as_str())
                .collect();
            let mut latencies = latency_string.split_whitespace().skip(1);
            let avg_latency = latencies.next().unwrap();
            let max_latency = latencies.nth(1).unwrap();

            let total_requests_string: String = TOTAL_REQUESTS_RGX
                .find_iter(&benchmark_result)
                .map(|mat| mat.as_str())
                .collect();
            let requests_per_sec_string: String = REQUESTS_PER_SECOND_RGX
                .find_iter(&benchmark_result)
                .map(|mat| mat.as_str())
                .collect();

            let total_requests = total_requests_string.split_whitespace().next().unwrap();
            let req_per_sec = requests_per_sec_string.split_whitespace().nth(1).unwrap();

            statistics.push(Stats {
                requests_per_second: req_per_sec.parse::<f64>().unwrap(),
                name: framework.name.to_string(),
                average_latency: avg_latency.to_string(),
                max_latency: max_latency.to_string(),
                total_requests: total_requests.parse().unwrap(),
                concurrency: setting.concurrency,
            })
        }
    }
    statistics
}

fn sort_framework(frameworks: &mut [Framework]) -> Vec<Vec<Stats>> {
    // calculate results and render into markdown table
    let mut vec = calculate_results(frameworks);

    let mut sorted_frameworks = Vec::new();

    // let sorted_frameworks: Vec<Vec<Option<Stats>>> = vec![vec![None, 4]; BENCHMARK_SETTINGS.len()];
    let chunks = vec.chunks_mut(frameworks.len());
    chunks.for_each(|curr| {
        curr.sort_by(|curr, next| {
            next.requests_per_second
                .partial_cmp(&curr.requests_per_second)
                .unwrap()
        });
        sorted_frameworks.push(curr.to_vec());
    });

    sorted_frameworks
    // println!("Chunks after {:?}", chunks);
}

static MARKDOWN_HEADER: &str =
    "|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |\n|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|";

static READ_ME_STRING: &str = r##"
# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.

==SPLIT==

## Benchmarking tool
The benchmarks have been performed using [wrk](https://github.com/wg/wrk), locally. 

Check the raw output from wrk [here](https://github.com/Ishtmeet-Singh/rust-framework-benchmarks/tree/master/perf).
## Try it yourself
To run the code please follow the steps - 

1. Download the repository as a zip, or clone/fork it.
2. `cd rust-framework-benchmarks`
3. `cargo build --release`
4. Open multiple terminals and start each server (if you want to run all simultaneously). 
Eg, `./target/release/actix` on one, `./target/release/hyper` on another and so on.
5. Run batch tests - `sh ./start-test.sh`

All the output will be stored in `perf/*`

## Machine used
M1 Max MacBook Pro 2021 - 64GB ram, 10 CPU cores and 32 GPU cores

## Suggestions and changes
All the suggestions, code changes or additions of another web framework is appreciated. I'd like to keep the code as close as a real world scenario, instead of optimising it to the metal.


"##;
