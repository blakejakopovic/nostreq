# nostreq

Nostr relay event request generator


## Examples

Note: Comma or space separated multi-values are supported.

Generate a relay request for events with kind 0 or 2
```shell
$ nostreq --kinds 0,2
```

Generate a relay request for events with authors
```shell
$ nostreq --authors pubkey1,pubkey2,pubkey3
```

Generate a relay request a custom subscription id
```shell
$ nostreq --subscription-id myspecialsub
```

Generate a blank relay request with UUID generated subscription id
```shell
$ nostreq
```

Show command arguments
```shell
$ nostreq --help
```

## Using with [nostcat](https://github.com/blakejakopovic/nostcat) to request events from relays

```shell
$ nostreq --limit 10 --kinds 1 | nostcat wss://relay.damus.io
```

## Full text

```
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
