use nostreq::*;

fn main() {
    let cli_matches = cli().get_matches();

    let mut request = request_from_cli(cli_matches);

    let request_json = request.to_json();

    println!(r#"["REQ", "{}", {}]"#, request.subscription_id, request_json);
}
