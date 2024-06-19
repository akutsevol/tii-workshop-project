mod elp_service;
mod parse_args;

fn main() {
    use crate::elp_service::{clone_config, get_message, prepare_message_for_say, say};

    parse_args::parse_args();

    let config = clone_config();
    println!("{:?}", config);

    let mut num_msg = config.num_msg;

    while num_msg > 0 {
        elp_service::be_ready(config.pause);

        let msg = get_message(config.msg_type.as_str());

        let msg_say = prepare_message_for_say(&msg);

        // say("Pilot".to_string(), false, config.comma).unwrap();
        println!("{}", msg.replace('!', ""));
        say(msg_say.replace("  ", " "), false, config.comma).unwrap();

        num_msg -= 1;
    }
}
