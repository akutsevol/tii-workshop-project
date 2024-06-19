mod elp_service;
mod parse_args;

fn main() {
    use crate::elp_service::{clone_config, get_message, prepare_message_for_say, say};

    parse_args::parse_args();

    let config = clone_config();
    println!("{:?}", config);

    // let alfabet_map = elp_service::get_alfabet();

    let mut num_msg = config.num_msg;

    while num_msg > 0 {
        elp_service::be_ready(config.pause);

        let msg = get_message(config.msg_type.as_str());

        let msg_say = prepare_message_for_say(&msg);

        // let mut skip = false;
        // let mut tmp = "".to_string();

        // for symbol in msg.chars() {
        //     if symbol == '!' {
        //         skip = true;
        //     } else if symbol == ' ' || symbol == ',' {
        //         tmp = tmp.trim().to_string();
        //         tmp.push(symbol);
        //         skip = false;
        //     } else if alfabet_map.contains_key(symbol.to_string().as_str()) && !skip {
        //         tmp.push_str(alfabet_map[symbol.to_string().as_str()]);
        //         tmp.push(' ');
        //     } else {
        //         tmp.push(symbol);
        //     }
        // }

        // say("Pilot".to_string(), false, config.comma).unwrap();
        println!("{}", msg.replace('!', ""));
        say(msg_say.replace("  ", " "), false, config.comma).unwrap();

        num_msg -= 1;
    }
}
