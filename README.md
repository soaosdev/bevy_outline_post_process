# bevy_outline_post_process

[![Crates](https://img.shields.io/crates/v/bevy_outline_post_process)](https://crates.io/crates/bevy_outline_post_process)
![License](https://img.shields.io/badge/license-0BSD%2FMIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_outline_post_process)
[![Docs](https://img.shields.io/docsrs/bevy_outline_post_process)](https://exvacuum.github.io/bevy_outline_post_process)

A plugin for the [Bevy](https://bevyengine.org) engine which adds an outline post-processing effect. Optionally supports adaptive outlining, so darker areas are outlined in white rather than black, based on luminance.

## Screenshots
![](https://git.exvacuum.dev/bevy_outline_post_process/plain/doc/screenshot.png)
![](https://git.exvacuum.dev/bevy_outline_post_process/plain/doc/screenshot_smooth.png)
Configuration Used:
```rs
bevy_outline_post_process::components::OutlinePostProcessSettings::new(2.0, LinearRgba::BLACK, 0.01, 0.01, 1.0);
```
## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.4-0.5       | 0.15         |
| 0.3           | 0.14         |
| 0.1-0.2       | 0.13         |

## Installation

### crates.io
```toml
[dependencies]
bevy_outline_post_process = "0.5"
```

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_outline_post_process]
git = "https://git.soaos.dev/bevy_outline_post_process"
```

## Usage

In `main.rs`:
```rs
use bevy::prelude::*;
use bevy_outline_post_process;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_outline_post_process::OutlinePostProcessPlugin,
        ))
        .run();
}
```

When spawning a camera:
```rs
commands.spawn((
    // Camera3d...
    bevy_outline_post_process::components::OutlinePostProcessSettings::default();
));
```

This effect will only run for cameras which contain this component.

## License

This crate is licensed under your choice of 0BSD, Apache-2.0, or MIT license.

