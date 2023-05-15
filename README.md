# nip95-tool

*THIS IS WORK IN PROGRESS*


## Description
A tool to decode images from kind 1064 events ([NIP-95](https://github.com/frbitten/nostr-nips/blob/NIP-95/95.md)).

## Instructions
Specify an input file that contains an array of JSON events of kind 1064 and an output folder like so:

```bash
nip95-tool -i exampleEvents.json -o files
```

To create this file, you can use the [nostr-tool](https://github.com/0xtrr/nostr-tool) CLI application like so:

```bash
nostr-tool -r wss://nostr.oxtr.dev list-events -k 1064 -o events.json
```

## Features
- [x] Basic kind 1064 events
- [ ] Encrypted files
- [ ] Fetch events directly from relay instead of json file with events