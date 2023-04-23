# Bifrost

The app controlling all wolbodo lights

Built using tauri + svelte

## Setup your dev environment:

https://next--tauri.netlify.app/next/guides/getting-started/prerequisites

## Functionality

It runs the engine written in rust that plays sequences and patterns.
The frontend mutates this engine and outputs it's state.
The engine outputs the light state over (E1.31)[https://en.wikipedia.org/wiki/Architecture_for_Control_Networks#External_extensions]
