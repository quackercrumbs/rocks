# rocks

Silly project to fetch near earth object data and render them in Bevy.
This isn't a really useful app, but it was definitely worth the learning experience.

## Goals

1. Practice Rust
2. Learn about ECS
3. Learn about futures

## Setup

Use the nightly tool chain
```
rustup default nightly-x86_64-pc-windows-msvc
```

### Secrets
```
echo "[topsecrets]" >> config/private.ini
echo "NASA_API_KEY = 123456" >> config/private.ini
```

## Quick Start
```
// running the data viewer
cargo run -p app
```

## Project Structure
| Name | Description |
| -- | -- |
| .cargo/ | contains configuration for cargo (build tool). Needed for bevy compilation on windows |
| app/ | the bevy app |
| rocks/ | library crate containing client to interface with nasa api and sqlite |\
| config/ | config files containing API keys |
| journal/ | some lessons learned that i documented while working on the project |
| importer/ | not used, originally to importer nasa data to local sqlite db |
| migrations/ | not used, diesel migration scripts. |

## References
- Bevy Getting Started: https://bevyengine.org/learn/book/getting-started/setup/
- Bevy Examples: https://github.com/bevyengine/bevy/tree/main/examples
- Bevy Cookbooy: https://github.com/bevy-cheatbook/bevy-cheatbook/tree/main/src/code/examples
- NASA API: https://api.nasa.gov/#asteroids-neows
- Bevy Egui: https://github.com/mvlabat/bevy_egui

## Credits
"Earth Day Map" (https://www.solarsystemscope.com/textures/) by solarsystemscope is licensed under Creative Commons Attribution (http://creativecommons.org/licenses/by/4.0/).
