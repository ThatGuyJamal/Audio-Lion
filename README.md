# Audio Lion

![icon](./static/lion/128px.png)

Audio Lion is a simple audio player for windows (currently) and is written in [Rust](https://www.rust-lang.org/) and [Svelte](https://svelte.dev/). 

It is currently in development and is not ready for use.

## Development Information

Audio Lion at its core is using [Tauri](https://tauri.app/) for its Gui and [Rodio](https://github.com/RustAudio/rodio) to play audio. 

More information on how I am developing Audio Lion can be found will be posted when the project becomes more stable. For now message me on discord if you have any questions.

My discord is: `ThatGuyJamal#2695`


## Installing

You can install the public build of Audio Lion (recommended) by downloading the latest release from the [releases](https://github.com/ThatGuyJamal/Audio-Lion/releases) page or from [our website](https://example.com/).

You can also view all past builds of the app [here](./installers/windows/)

## Building (Windows)

To build Audio Lion you will need to have [Rust](https://www.rust-lang.org/) and [Node.js](https://nodejs.org/en/) installed.

To build Audio Lion you will need to run the following commands:

```bash
git clone https://github.com/ThatGuyJamal/Audio-Lion audio_lion_app

cd audio_lion_app

yarn install

yarn tauri build
```

After building you will find the executable in the [this](./src-tauri/target/release/bundle/msi/) folder. *This link be clickable once the project is built.*

## Building (Linux)

Coming soon...

## Building (Mac)

Not supported yet...