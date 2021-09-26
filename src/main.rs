mod speeds_and_feeds;

#[macro_use]
extern crate enum_display_derive;

use crate::speeds_and_feeds::material::Material;
use crate::speeds_and_feeds::tool::Tool;
use crate::speeds_and_feeds::cut_type::CutType;
use crate::speeds_and_feeds::cut_strategy::CutStrategy;
use crate::speeds_and_feeds::aggressiveness::Aggressiveness;
use crate::speeds_and_feeds::cut_parameters::{stepover, chipload, adjusted_chipload, depth_of_cut};

fn main() {
    let material = Material::Soft;
    let tool = Tool::Quarter;
    let cut_type = CutType::Rough;
    let cut_strategy = CutStrategy::WideAndShallow;
    let aggressivness = Aggressiveness::High;
    let rpm = 18000f64;
    let flutes = 3f64;

    let stepover = stepover(cut_strategy, cut_type, tool, aggressivness);
    let target_chipload = chipload(tool, material, aggressivness);
    let adjusted_chipload = adjusted_chipload(target_chipload, stepover, tool);
    let feed_rate = adjusted_chipload * flutes * rpm;
    let depth_of_cut = depth_of_cut(cut_strategy, material, tool, aggressivness);

    println!("Parameters ->. material:{}, tool:{}, cut_type:{}, strategy:{}, aggressiveness:{}",
         material,
         tool,
         cut_type,
         cut_strategy,
         aggressivness
    );

    println!("Outputs -> feed rate:{}, depth of cut:{}", feed_rate, depth_of_cut);
}
