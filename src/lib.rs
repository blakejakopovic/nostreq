use clap::{Arg, Command, ArgAction, ArgMatches};
use serde::{Serialize, Deserialize};

pub fn cli() -> Command {
    Command::new("nostreq")
        .about("Nostr relay event request generator")
        .version("0.1.0")
        .author("Blake Jakopovic")
        .arg(
            Arg::new("subscription-id")
                .help("custom request subscription id")
                .long("subscription-id")
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("ids")
                .help("a list of event ids or prefixes")
                .long("ids")
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("authors")
                .help("a list of pubkeys or prefixes, the pubkey of an event must be one of these")
                .long("authors")
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("kinds")
                .help("a list of a kind numbers")
                .long("kinds")
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("etags")
                .help(r#"a list of event ids that are referenced in an "e" tag"#)
                .long("etags")
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("ptags")
                .help(r#"a list of pubkeys that are referenced in a "p" tag"#)
                .long("ptags")
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("since")
                .help("a timestamp, events must be newer than this to pass")
                .long("since")
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("until")
                .help("a timestamp, events must be older than this to pass")
                .long("until")
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("limit")
                .help("maximum number of events to be returned in the initial query")
                .long("limit")
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(u32))
        )

}

// Reference from NIP-01 Documentation
// {
//   "ids": <a list of event ids or prefixes>,
//   "authors": <a list of pubkeys or prefixes, the pubkey of an event must be one of these>,
//   "kinds": <a list of a kind numbers>,
//   "#e": <a list of event ids that are referenced in an "e" tag>,
//   "#p": <a list of pubkeys that are referenced in a "p" tag>,
//   "since": <a timestamp, events must be newer than this to pass>,
//   "until": <a timestamp, events must be older than this to pass>,
//   "limit": <maximum number of events to be returned in the initial query>
// }

#[derive(Serialize, Deserialize)]
pub struct RequestBuilder {
  #[serde(skip_serializing_if = "Option::is_none")]
  ids: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  authors: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  kinds: Option<Vec<u32>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "#e"))]
  e: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "#p"))]
  p: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  since: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  until: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit: Option<u32>,
}

impl RequestBuilder {

  pub fn new() -> RequestBuilder {
        RequestBuilder {
            ids: None,
            authors: None,
            kinds: None,
            e: None,
            p: None,
            since: None,
            until: None,
            limit: None,
        }
  }

  pub fn id(&mut self, id: String) -> &mut Self {
      self.ids([id].to_vec())
  }

  pub fn ids(&mut self, mut ids: Vec<String>) -> &mut Self {
      self.ids.get_or_insert(vec![]).append(&mut ids);
      self
  }

  pub fn author(&mut self, author: String) -> &mut Self {
      self.authors([author].to_vec())
  }

  pub fn authors(&mut self, mut authors: Vec<String>) -> &mut Self {
      self.authors.get_or_insert(vec![]).append(&mut authors);
      self
  }

  pub fn kind(&mut self, kind: u32) -> &mut Self {
      self.kinds([kind].to_vec())
  }

  pub fn kinds(&mut self, mut kinds: Vec<u32>) -> &mut Self {
      self.kinds.get_or_insert(vec![]).append(&mut kinds);
      self
  }

  pub fn etag(&mut self, etag: String) -> &mut Self {
      self.etags([etag].to_vec())
  }

  pub fn etags(&mut self, mut etags: Vec<String>) -> &mut Self {
      self.e.get_or_insert(vec![]).append(&mut etags);
      self
  }

  pub fn ptag(&mut self, ptag: String) -> &mut Self {
      self.ptags([ptag].to_vec())
  }

  pub fn ptags(&mut self, mut ptags: Vec<String>) -> &mut Self {
      self.p.get_or_insert(vec![]).append(&mut ptags);
      self
  }

  pub fn since(&mut self, since: u32) -> &mut Self {
      self.since = Some(since);
      self
  }

  pub fn until(&mut self, until: u32) -> &mut Self {
      self.until = Some(until);
      self
  }

  pub fn limit(&mut self, limit: u32) -> &mut Self {
      self.limit = Some(limit);
      self
  }

  pub fn to_json(&mut self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

pub fn request_from_cli(cli_matches: ArgMatches) -> RequestBuilder {

    let mut request = RequestBuilder::new();

    for id in cli_matches
        .get_many::<String>("ids")
        .unwrap_or_default() {
            request.id(id.to_string());
        }

    for author in cli_matches
        .get_many::<String>("authors")
        .unwrap_or_default() {
             request.author(author.to_string());
        }

    for kind in cli_matches
        .get_many::<u32>("kinds")
        .unwrap_or_default() {
             request.kind(*kind);
        };

    for etag in cli_matches
        .get_many::<String>("etags")
        .unwrap_or_default() {
             request.etag(etag.to_string());
        }

    for ptag in cli_matches
        .get_many::<String>("ptags")
        .unwrap_or_default() {
             request.ptag(ptag.to_string());
        }

    match cli_matches.get_one::<u32>("since") {
      None => {},
      Some(since) => { request.since(*since); }
    }

    match cli_matches.get_one::<u32>("until") {
      None => {},
      Some(until) => { request.until(*until); }
    }

    match cli_matches.get_one::<u32>("limit") {
      None => {},
      Some(limit) => { request.limit(*limit); }
    }

    request
}
