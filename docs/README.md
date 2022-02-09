# Notes

Random collection of notes throughout development process.
----

02/09
- I was thinking that before i start making calls to the NASA api, i would want some way to control when to send the request and what parameters to send with it
- To do that, i should probably have some UI components
- I tried following the bevy ui example, but couldn't figure out a way to implement an editable text field (or didn't feel like doing the work)
  - The example that i was following: https://github.com/bevyengine/bevy/blob/main/examples/ui/ui.rs
- Found this bevy_egui library via the bevy assets store (https://bevyengine.org/assets/#ui)
  - Seemed promissing so i'll stick to using that.
  - There was a cool working running example (assessible in the browser too!)
    - https://mvlabat.github.io/bevy_egui_web_showcase/index.html


02/07
- was having trouble trying to connect diesel sqlite as a resource.
  - Was getting compilation errors saying that some of the fields in SqliteConnection were RefCell and didn't implement the Sync Trait
  - As shown in this doc, SqliteConnection doesn't implement it (https://docs.diesel.rs/master/diesel/prelude/struct.SqliteConnection.html#impl-Sync)
- then tried to turn to using hyper, but ran into another problem
  - This issue basically describes it (i'm using the dynamic feature, but the linker doesn't work when using bevy/dynamic and hyper)
  - https://github.com/bevy-cheatbook/bevy-cheatbook/issues/114
  - https://github.com/bevyengine/bevy/issues/2547
  - Just removed dynamic from the cargo feature for bevy

02/06
- was having trouble loading assets pulled from sketch fab. When trying to use the auto generated GLTF for some of the models, the textures would never load. and sometimes the mesh would take a couple of minutes to load.
- Instead, just looked for a free earth texture and applied it to a sphere.
- Created the model via Microsoft's 3d builder
- This is also a good resource:
  - https://bevy-cheatbook.github.io/features/gltf.html
  - https://github.com/bevyengine/bevy/blob/latest/examples/3d/load_gltf.rs