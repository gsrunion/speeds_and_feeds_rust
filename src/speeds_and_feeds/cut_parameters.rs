use std::fmt::Display;
use std::fmt::Formatter;


use crate::speeds_and_feeds::cut_strategy::CutStrategy;
use crate::speeds_and_feeds::cut_type::CutType;
use crate::speeds_and_feeds::material::Material;
use crate::speeds_and_feeds::tool::Tool;
use crate::speeds_and_feeds::aggressiveness::Aggressiveness;

pub fn stepover(strategy: CutStrategy, cut_type: CutType, tool: Tool, aggressiveness: Aggressiveness) -> f64 {
    let range = match (cut_type, strategy) {
        // TODO cross check these ranges later
        // TODO consider making CutType and CutStrategy multipliers such that they can handle their conversion
        (CutType::Rough, CutStrategy::WideAndShallow) => (0.4f64, 1f64) ,
        (CutType::Rough, CutStrategy::NarrowAndDeep) => (0.1f64, 0.25f64),
        (CutType::Finish, CutStrategy::WideAndShallow) => (0.05f64, 0.1f64),
        (CutType::Finish, CutStrategy::NarrowAndDeep) => (0.05f64, 0.1f64),
    };

    let aggresiveness_multiplier: f64 = aggressiveness.select(range);
    let standard_chipload: f64 = 0.5f64;
    aggresiveness_multiplier * tool.value() * standard_chipload
}

pub fn chipload(tool: Tool, material: Material, aggressiveness: Aggressiveness) -> f64 {
    let range = match (tool, material) {
        // TODO check range values and perhaps find a way not to match
        (Tool::Quarter, Material::Soft) => (0.0f64, 0.0f64),
        (Tool::Quarter, Material::Medium) => (0.0f64, 0.0f64),
        (Tool::Quarter, Material::Hard) => (0.0f64, 0.0f64),

        (Tool::Eighth, Material::Soft) => (0.0f64, 0.0f64),
        (Tool::Eighth, Material::Medium) => (0.0f64, 0.0f64),
        (Tool::Eighth, Material::Hard) => (0.0f64, 0.0f64),

        (Tool::Sixteenth, Material::Soft) => (0.0f64, 0.0f64),
        (Tool::Sixteenth, Material::Medium) => (0.0f64, 0.0f64),
        (Tool::Sixteenth, Material::Hard) => (0.0f64, 0.0f64),
    };

    aggressiveness.select(range)
}

pub fn adjusted_chipload(chipload: f64, stepover: f64, tool: Tool) -> f64 {
    // TODO include formula in comments, add test, make source look more like formula
    let numerator: f64 = chipload * tool.value() / 2.0f64;
    let a: f64 = stepover * tool.value();
    let b: f64 = stepover * stepover;
    let c: f64 = a - b;
    let denominator: f64 = c.sqrt();
    numerator / denominator
}

pub fn depth_of_cut(strategy: CutStrategy, material: Material, tool: Tool, aggressiveness: Aggressiveness) -> f64 {
    let range = match (strategy, material) {
        // TODO check ranges
        (CutStrategy::WideAndShallow, Material::Soft) => (0.0f64, 0.0f64),
        (CutStrategy::WideAndShallow, Material::Medium) => (0.0f64, 0.0f64),
        (CutStrategy::WideAndShallow, Material::Hard) => (0.0f64, 0.0f64),

        (CutStrategy::NarrowAndDeep, Material::Soft) => (0.0f64, 0.0f64),
        (CutStrategy::NarrowAndDeep, Material::Medium) => (0.0f64, 0.0f64),
        (CutStrategy::NarrowAndDeep, Material::Hard) => (0.0f64, 0.0f64),
    };

    aggressiveness.select(range) * tool.value()
}