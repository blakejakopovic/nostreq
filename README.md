# nostreq

Nostr relay event request generator.


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
