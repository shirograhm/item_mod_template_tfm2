# item_mod_template_tfm2

A starter template for Teamfight Manager 2 item mods, built on the stable mod ABI. It adds one example item, **Infinity Edge**, which you can replace to make your own.

## Layout

| Path | What it is |
| --- | --- |
| `src/lib.rs` | Mod entry point. Registers each item with `reg.add_item(...)`. |
| `src/infinity_edge.rs` | Example item: key, icon, price, tier, build path, stats, tags and an `on_attack` hook. |
| `text/item.i18n` | Item names and tooltip text per language, keyed by item key. Merged into the base game's text. |
| `aseprite_resources/ingame/` | Item icon sheet (`#sheet.png`) and its frame map (`#data.sprite_sheet`). Overrides the base game's sheet. |
| `mod.mod_info` | Mod name, author, id, version and base game dependency. |
| `mod.override_info` | Points the base game's item text and icon assets at this mod's copies. |
| `mod-api-stable/` | Copy of the SDK from the game's `mod-sdk-stable` folder. This may require an update if the game is updated. |

## Make it yours

1. **Pick a mod id** and use it everywhere: the folder name, `mod_id` in `mod.mod_info`, `StableMod::new(...)` in `src/lib.rs`, `name` in `Cargo.toml`, and the `asset/<mod_id>/...` paths in `mod.override_info`. Then fill in `name`, `author` and `description` in `mod.mod_info`.
2. **Add an item.** Copy `src/infinity_edge.rs`, change the struct name and each method's value, then add `mod your_item;` and `reg.add_item(your_item::YourItem);` to `src/lib.rs`. The comments in the example explain each field.
3. **Add its text.** Under every language in `text/item.i18n`, add an entry with the item's key containing a `name` and an `option` (the tooltip effect text).
4. **Add its icon.** Draw an 18×18 icon onto `#sheet.png`, then add a frame to `#data.sprite_sheet` named after what `icon()` returns. Frame `x`, `y`, `w` and `h` are fractions of the sheet's width and height, not pixels.

## Build and install

You need Rust ([rustup.rs](https://rustup.rs)).

```sh
cargo build --release
```

Create `<game folder>/mods/<mod_id>/` and copy these into it:

- `target/release/<mod_id>.dll`
- `mod.mod_info` and `mod.override_info`
- `text/` and `aseprite_resources/`
- `thumbnail.png`

The DLL must be named after the mod id, which is also the folder name.
