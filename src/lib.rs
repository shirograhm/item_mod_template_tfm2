use mod_api_stable::*;

mod infinity_edge;

fn init(_host: &StableHost) -> StableMod {
    let mut reg = StableMod::new("item_mod_template_tfm2");

    // Add item to game
    reg.add_item(infinity_edge::InfinityEdge);
    reg
}

declare_stable_mod!(init);
