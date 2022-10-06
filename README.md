# Gltf Viewer

Quick and dirty viewer of Gltf models without animation support.

This is mainly useful for models vs. entire scenes in order to verify that models are loaded
correctly and can be attached to other enities in order to move/rotate them inside the game.

![sample](./assets/gltf-viewer.gif)

Shoutout to the awesome [bevy_editor_pls](https://github.com/jakobhellermann/bevy_editor_pls)
editor tools which provide a versatile camera + a way to quickly tweak some values.

## Usage

After you clone the repo just do:

```sh
cargo run
```

to see the famous flight helmet.

To load another Gltf scene provide the full path to it instead, i.e.:

```sh
cargo run /Volumes/path/to/my/awesome-assets/scene.gltf#Scene0
```

## Alternatives

For a full-fledged scene viewer try the [bevy scene
viewer](https://github.com/bevyengine/bevy/blob/main/examples/tools/scene_viewer.rs) instead.
