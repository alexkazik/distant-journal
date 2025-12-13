# Unofficial Sleeping Gods: Distant Skies Journal

Use the tool [here](https://alexkazik.github.io/distant-journal/).

## Translation

If you want to help with the translation, update wither `msg.lrc` and/or `game.lrc`.

And then either create a pull request, an issue or contact me
(via [email](mailto:distant-journal+6437@tx0.eu) or [BGG](https://boardgamegeek.com/geekmail/compose?touser=txnull)).

## Running it yourself

### Requirements

- https://rustup.rs/
- `rustup target add wasm32-unknown-unknown`
- https://trunkrs.dev/#install

### Running

Run this application with the trunk development server:

```bash
trunk serve --features=debug --open
```

### Building

```bash
trunk build
```

If the application will not be in the domain root (e.g. `https://example.com/distant-journal`):

```bash
trunk build --no-default-features --public-url /distant-journal
```
