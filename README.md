# Home Page
KevKevDev Main Home page

Containing a lot of portfolio style projects; and being a useful app in its own right.

## Features:
  - Nav bar
  - Blog
  - ToDo mini-app
  - graphics demos, web games, more and more as I create, and when I bring in past projects to show off.



## Build errors:

> rust wasm file schema version: 0.2.88
>      this binary schema version: 0.2.87
>
> Currently the bindgen format is unstable enough that these two schema versions
> must exactly match. You can accomplish this by either updating this binary or 
> the wasm-bindgen dependency in the Rust project.
> 
> You should be able to update the wasm-bindgen dependency with:
> 
>    `cargo update -p wasm-bindgen --precise 0.2.87`
>
> don't forget to recompile your wasm file! Alternatively, you can update the 
> binary with:
>
>    `cargo install -f wasm-bindgen-cli --version 0.2.88`

from google: 

[2019] Using wasm-pack build should have you good to go here as it will download the correct binary.

[2023] need to specify in my own cargo.toml, 

`wasm-bindgen = "=0.2.87"` or whatever
