mod elp_service;
mod parse_args;

fn main() {
    use crate::elp_service::{clone_config, get_message, say};

    parse_args::parse_args();

    let alfabet_map = elp_service::get_alfabet();

    elp_service::be_ready(1);
    // println!("{}", get_message("BLA"));
    // println!("{}", get_message(VALID_MESSAGES[check_valid_msg("ANY") as usize]));
    // println!("{}", get_message(VALID_MESSAGES[check_valid_msg("ATIS") as usize]));

    let config = clone_config();

    println!("{:?}", config);

    let mut num_msg = config.num_msg;

    while num_msg > 0 {
        let msg = get_message(config.msg_type.as_str());

        let mut skip = false;
        let mut tmp = "".to_string();

        for symbol in msg.chars() {
            if symbol == '!' {
                skip = true;
            } else if symbol == ' ' || symbol == ',' {
                tmp = tmp.trim().to_string();
                tmp.push(symbol);
                skip = false;
            } else if alfabet_map.contains_key(symbol.to_string().as_str()) && !skip {
                tmp.push_str(alfabet_map[symbol.to_string().as_str()]);
                tmp.push(' ');
            } else {
                tmp.push(symbol);
            }
        }

        println!("Message: {}", msg.replace('!', ""));
        say(tmp.replace("  ", " "), false, config.comma);

        num_msg -= 1;
    }
}
