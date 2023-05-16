use nostreq::*;
use std::env;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Support for --or as an OR filter input separator
    let or_list: Vec<Vec<String>> = args[1..].into_iter().fold(Vec::new(), |mut acc, x| {
        if x == "--or" || acc.is_empty() {
            // clap expects the program name in index 0, so we must add it to each OR arg set
            acc.push(vec![String::from(args[0].to_string())]);
        }
        if x != "--or" {
            acc.last_mut().unwrap().push((&x).to_string());
        }
        acc
    });

    // Process each OR argument set
    let mut filter: Vec<String> = vec![];
    for or in &or_list {
        let cli_matches = cli().get_matches_from(or);

        let mut request = request_from_cli(cli_matches);

        let request_json = request.to_json();

        // Only append if the json request is not empty
        if request_json != "{}" {
            filter.push(request_json)
        }
    }

    // Set default value for filter if it's empty
    if filter.is_empty() {
        filter = vec!["{}".to_string()];
    }

    // Generate subscription id, if not provided
    let subscription_id = match or_list.first() {
        Some(args) => {
            let cli_matches = cli().get_matches_from(args);
            match cli_matches.get_one::<String>("subscription-id") {
                None => Uuid::new_v4().to_string(),
                Some(m) => m.to_string(),
            }
        }
        None => Uuid::new_v4().to_string(),
    };

    println!(r#"["REQ","{}",{}]"#, subscription_id, filter.join(","));
}
