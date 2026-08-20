<img width="300" src="https://github.com/CobaltRenderer/Cobalt/blob/main/Assets/logos/ColorTransparentText.png?raw=true">

# cobalt_renderer_sys

This crate contains FFI bindings for the Cobalt Renderer CBindings library. Use the `cobalt_renderer` crate for a safer Rust API.

## Bindings Generation

Bindings are pregenerated with `bindgen` and stored at `bindings.rs`. The bindings are generated on Linux x86_64. They are intended to be cross-platform, but 32-bit targets may not work. When the `generate_bindings` feature is enabled, new bindings are generated and used but not saved. Crate maintainers can use the `generate_bindings.sh` script in the repository to save the currently generated bindings.

## Contributing

This project welcomes contributions and suggestions. Feel free to join our public [Discord server](https://discord.gg/St5bkhrnu8) to get involved. If you're interested in contributing code changes, please see the Cobalt Renderer [README](https://github.com/CobaltRenderer/Cobalt/tree/main#contributing) for guidance on how to make the process smoother.

## License

While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](https://github.com/CobaltRenderer/Cobalt-rs/blob/main/LICENSE.txt) file in this repository for full information.
