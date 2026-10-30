# Maintenance Guide

### When Cobalt updates

Cobalt-rs should update shortly after a new version of the Cobalt Renderer is made.
- For the system bindings
  - Update `cobalt_renderer_sys` to the new version of the Cobalt Renderer, it should match
  - Run `generate_bindings.sh` (requires Linux) to generate a new set of bindings, any updates should be reviewed and committed
    - In theory, only major and minor releases should cause changes that aren't version number definitions
  - in `build.rs`, update the release commit hash and download SDK links and hashes to match the new release
- For the main `cobalt_renderer` crate
  - Update `cobalt_renderer` to use the new system version and increment it's version as needed
  - If new functionality is available or other changes are required, they should be made
    - The release changelog for Cobalt Renderer should be reviewed for any impactful changes
  - Update the main `README.md` to use the new version
- All changes should be tested locally and on the CI system before publishing
