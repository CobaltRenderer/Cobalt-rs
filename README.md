<img width="300" src="https://github.com/CobaltRenderer/Cobalt/blob/main/Assets/logos/ColorTransparentText.png?raw=true">

# Cobalt-rs

The Cobalt Renderer is a generic, cross platform graphics library. Cobalt aims to make using modern 3D graphics hardware simple. It does this by giving you a clean API to express what you want to draw (or compute), without getting bogged down in how to do that. Cobalt is designed to be an accessible, lightweight alternative to directly using low-level graphics APIs such as Vulkan, Direct3D and OpenGL.

This is a Rust wrapper for the [Cobalt Renderer](https://github.com/CobaltRenderer/Cobalt). The `cobalt_renderer` crate provides the complete Cobalt Renderer API for Rust. There are only a few tweaks to the C++ API to make it safer and easier.

**While the Cobalt Renderer is used in production, this wrapping crate should be considered in beta. We may introduce significant API changes in the future and there may be bugs. We welcome any bug reports or suggestions.**

## Example

Here is a simple example of printing the name of all available graphics devices.

```rust
// Setup library
let library = cobalt_renderer::init().expect("Could not init cobalt_renderer library");

// Find renderer plugin to use
let path = std::path::PathBuf::from(cobalt_renderer::LOCAL_RUNTIME_BIN_DIR);
let mut plugin_enumerator = library.renderer_plugin_enumerator();
plugin_enumerator
    .enumerate_plugins_in_directory(path)
    .unwrap();
let mut plugin = plugin_enumerator
    .preferred_plugin()
    .expect("Could not find a renderer plugin");

// Find all devices and print out name
let enumerator = plugin
    .create_device_enumerator(cobalt_renderer::renderer::DeviceEnumerationFlags::None)
    .expect("Could not enumerate devices");
for device in enumerator.all_devices() {
    println!("{}", device.device_name());
}
```

To see general usage (and a spinning triangle), please see `examples/hello_triangle.rs` in the project repository. 
To run the example with a downloaded SDK, use `cargo run --example hello_triangle --features download_sdk`

## Getting Started 

Default builds require a Cobalt SDK and setting the environment variable `COBALT_SDK_DIR` to its path. We recommend this for established projects. You can download an [official SDK release](https://github.com/CobaltRenderer/Cobalt/releases), or clone and build one yourself.

This crate also support downloading releases automatically with the optional `download_sdk` feature. 

**NOTE: The `download_sdk` and `build_sdk` features are intended as a convenience feature, but you should be cautious of build scripts downloading payloads. The build script checks the release SHA-256 hash matches and only clones specific commits, not tags, but you ultimately take responsibility for using these features.**

```toml
[dependencies]
cobalt_renderer = { version = "0.1.1", features = ["download_sdk"] }
```

You should also set the `COBALT_SDK_CACHE_DIR` environment variable in your projects `.cargo/config.toml` to save the SDK outside the `target` directory and preserve it when `cargo clean` is run.

```toml
[env]
COBALT_SDK_CACHE_DIR = { value = "CobaltSDK", relative = true }
```

If an SDK is not available for your platform or otherwise has issues, you can also use the `build_sdk` feature, which will clone and build the SDK from source. 

More information on using an SDK, the available options and environment variables can be found in this crate's documentation.

## Cobalt SDK

`cobalt_renderer` requires the Cobalt Renderer SDK as an external dependency. The SDK can be provided by the user, automatically downloaded, or cloned and built from source. See documentation for more information.

### Versioning

The major and minor SDK version must match the version of `cobalt_renderer_sys` used. Patch numbers may be different. When using the `download_sdk` and `build_sdk` features, the corresponding version will be used.

## Documentation

See [SDK Documentation](https://cobaltrenderer.github.io/Cobalt) for detailed documentation for using the Cobalt Renderer and fundamental concepts. Although this documentation is for C++, it maps well to the `cobalt_renderer` crate. Specific Rust crate documentation can be found in the `cobalt_renderer` crate docs.

## Contributing

This project welcomes contributions and suggestions. Feel free to join our public [Discord server](https://discord.gg/St5bkhrnu8) to get involved. If you're interested in contributing code changes, please see the Cobalt Renderer [README](https://github.com/CobaltRenderer/Cobalt/tree/main#contributing) for guidance on how to make the process smoother.

## License

While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](https://github.com/CobaltRenderer/Cobalt-rs/blob/main/LICENSE.txt) file in this repository for full information.
