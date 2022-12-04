# nostreq

Nostr relay event request generator

## Getting started
Using Cargo to install (requires ~/.cargo/bin to be in PATH)
```shell
$ cargo install nostreq
```

Building from source (may be unstable)
```shell
$ git clone https://github.com/blakejakopovic/nostreq
$ cargo build --release
$ ./target/release/nostreq
```

## Examples

Note: Comma or space separated multi-values are supported.

Generate a relay request for events with kind 0 or 2
```shell
$ nostreq --kinds 0,2
["REQ", "ec3e6238-5ef3-4162-899c-a58e882e586a", {"kinds":[0,2]}]
```

Generate a relay request for events with authors
```shell
$ nostreq --authors pubkey1,pubkey2,pubkey3
["REQ", "2e3479cd-e04d-4e4d-a370-a7c77cf7998e", {"authors":["pubkey1","pubkey2","pubkey3"]}]
```

Generate a relay request with multiple filters (OR)
```shell
$ nostreq --authors pubkey1 --limit 1 --or --etags pubkey1
["REQ", "dede0573-ffb4-45e7-a2e7-346178426762", {"authors":["pubkey1"],"limit":1},{"#e":["pubkey1"]}]
```

Generate a relay request a custom subscription id
```shell
$ nostreq --subscription-id myspecialsub
["REQ", "myspecialsub", {}]
```

Generate a blank relay request with UUID generated subscription id
```shell
$ nostreq
["REQ", "83d1ca68-c629-452c-b045-16197fde2b42", {}]
```

## Using with [nostcat](https://github.com/blakejakopovic/nostcat) to request events from relays

```shell
$ nostreq --limit 1 --kinds 1 | nostcat wss://relay.damus.io
["EVENT","515a1980-fdf4-4a42-936b-b64eb7bd1574",{"id":"c7ac087a2ab2e293dd5d75ca344be649983dee263166e1da3a3d806fed4b5240","pubkey":"887645fef0ce0c3c1218d2f5d8e6132a19304cdc57cd20281d082f38cfea0072","created_at":1670150101,"kind":1,"tags":[],"content":"Show HN: Using Vim as an input method editor (IME) for X11 apps https://github.com/algon-320/vime","sig":"d02e6fe1b9c34e33c5182871cfc251680b760860275896965fae08c8a3f007648d9708c0277143f44d41f2e2a1a9fe24a865526be971a26d4347baf01175dfc7"}]
```


## Full help text
```shell
$ nostreq --help

Usage: nostreq [OPTIONS]

Options:
      --subscription-id <subscription-id>
          custom request subscription id
      --ids [<ids>...]
          a list of event ids or prefixes
      --authors [<authors>...]
          a list of pubkeys or prefixes, the pubkey of an event must be one of these
      --kinds [<kinds>...]
          a list of a kind numbers
      --etags [<etags>...]
          a list of event ids that are referenced in an "e" tag
      --ptags [<ptags>...]
          a list of pubkeys that are referenced in a "p" tag
      --since <since>
          a timestamp, events must be newer than this to pass
      --until <until>
          a timestamp, events must be older than this to pass
      --limit <limit>
          maximum number of events to be returned in the initial query
  -h, --help
          Print help information
  -V, --version
          Print version information
```
