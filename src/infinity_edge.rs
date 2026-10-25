use mod_api_stable::*;

#[derive(Clone, Debug)]
pub struct InfinityEdge;

impl StableItem for InfinityEdge {
    fn clone_box(&self) -> Box<dyn StableItem> {
        Box::new(self.clone())
    }

    fn key(&self) -> String {
        // Key used in game
        "infinity_edge".to_string()
    }

    fn icon(&self) -> String {
        // Icon term used for this item. This can be set from item_icons_18x18#data.sprite_sheet file
        "infinity_edge".to_string()
    }

    fn price(&self) -> usize {
        // Price of upgrading to this item in gold
        1000
    }

    fn tier(&self) -> usize {
        // The tier of the item. 0 -> tier1 items, 1 -> tier2 items, 2 -> tier3 items, 3 -> tier4 items, and 4 -> tier5 items
        3
    }

    fn previous_tier(&self) -> Vec<String> {
        // Items the player can build from to get this item. This is used to show the "builds from" section of the tooltip
        vec!["ruinous_blade".to_string()]
    }

    fn next_tier(&self) -> Vec<String> {
        // Items the player can build into from this item. This is used to show the "builds into" section of the tooltip
        vec![]
    }

    fn stat(&self) -> BuffV1 {
        // Base stats for the item that will show up at the top of the tooltip
        BuffV1 {
            attack: 50,
            crit_chance: 20,
            ..Default::default()
        }
    }

    fn on_attack(
        &mut self,
        _sim: &mut StableSim<'_>,
        _caster: usize,
        _target: usize,
        damage: &mut usize,
        _damage_type: mod_api_stable::DamageTypeV1,
        attack_type: AttackTypeV1,
        is_crit: bool,
    ) {
        // If the attack is a base attack and it crits, apply the bonus crit damage
        if attack_type == AttackTypeV1::BaseAttack && is_crit {
            *damage += (*damage as f64 * (30.0 / 100.0)).round() as usize
        }
    }

    fn tags(&self) -> Vec<ItemTagV1> {
        // Tagged AD because the stats grant AD and crit chance (crit chance has no tag)
        vec![ItemTagV1::Ad]
    }

    fn category(&self) -> ItemCategoryV1 {
        // Tagged AD because it is an item that is typically used by AD champions
        ItemCategoryV1::Ad
    }
}
