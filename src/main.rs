/*
 * Speeds and feeds calculations as described at
 * https://shapeokoenthusiasts.gitbook.io/shapeoko-cnc-a-to-z/feeds-and-speeds-basics#wrapping-up-suggested-process
 */


mod speeds_and_feeds;

#[macro_use]
extern crate enum_display_derive;





use crate::speeds_and_feeds::cut_parameters::{stepover, chipload, adjusted_chipload, depth_of_cut};
use std::io;
use std::io::Write;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

fn prompt_for_str(prompt: &str) -> String {
    io::stdout().write(prompt.as_bytes()).unwrap();
    io::stdout().write("\n>".as_bytes()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.as_str().trim());
}

fn prompt_for_number<T: FromStr>(prompt: &str) -> T {
    let input = prompt_for_str(prompt);
    match input.parse::<T>() {
        Ok(x) => x,
        Err(_) => panic!("Expected numeric input. Encountered '{}'", input)
    }
}

fn prompt_and_map<T>(prompt: &str, mapper: fn(u8) -> T) -> T {
    mapper(prompt_for_number(prompt))
}

fn prompt_for_enum<T: TryFrom<u8>>(prompt: &str) -> T {
    match prompt_and_map(prompt, |x| x.try_into()) {
        Ok(val) => val,
        Err(_) => panic!("")
    }
}


fn main() {
    println!("Speeds and feeds calculator for Shapeoko 3 CNC routers");

    // better would be to have the enums define how to print themselves
    let material = prompt_for_enum("What kind of material are you cutting soft(1), medium(2), hard(3)?");
    let tool = prompt_for_enum("What kind of bit are you using sixteenth(1), eighth(2), quarter(3)?");
    let cut_type = prompt_for_enum("What kind of cut are you using roughing(1), finishing(2)?");
    let cut_strategy = prompt_for_enum("What kind of cut strategy are you using wide and shallow(1), narrow and deep(2)?");
    let aggressiveness = prompt_for_enum("How aggressive do you want to be low(1), medium(2), high(3)");
    let rpm: u32  = prompt_for_number("What rpm");
    let flutes: u8 = prompt_for_number("How many flutes");

    let stepover = stepover(cut_strategy, cut_type, tool, aggressiveness);
    let target_chipload = chipload(tool, material, aggressiveness);
    let adjusted_chipload = adjusted_chipload(target_chipload, stepover, tool);
    let feed_rate = adjusted_chipload * flutes as f64 * rpm as f64;
    let depth_of_cut = depth_of_cut(cut_strategy, material, tool, aggressiveness);

    println!("Parameters ->. material:{}, tool:{}, cut_type:{}, strategy:{}, aggressiveness:{}",
         material,
         tool,
         cut_type,
         cut_strategy,
         aggressiveness
    );

    println!("Outputs -> feed rate:{}, depth of cut:{}", feed_rate, depth_of_cut);
}
