mod elp_service;

fn parse_args() {
    use clap::{Arg, Command};
    use elp_service::ConfigElp;

    let valid_messages = vec!["ANY", "ATIS"]; // Define valid messages as needed

    let matches = Command::new("elp")
        .about("ELP training tool for pilots")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .default_value("ANY")
                .help("Message type")
                .value_parser(valid_messages),
        )
        .arg(
            Arg::new("count")
                .short('m')
                .long("count")
                .default_value("1")
                .help("Message count"),
        )
        .arg(
            Arg::new("rate")
                .short('r')
                .long("rate")
                .default_value("180")
                .help("Words per minute"),
        )
        .arg(
            Arg::new("pause")
                .short('p')
                .long("pause")
                .default_value("5")
                .help("Pause between messages"),
        )
        .arg(
            Arg::new("comma")
                .short('c')
                .long("comma")
                .default_value("0")
                .help("Pause between blocks"),
        )
        .arg(
            Arg::new("sayagain")
                .short('s')
                .long("sayagain")
                .default_value("0")
                .help("Repeat each message 2 times"),
        )
        .get_matches();

    let my_config = ConfigElp::new(
        matches.get_one::<String>("type").unwrap().to_string(),
        matches.get_one::<String>("count").unwrap().parse().unwrap(),
        matches.get_one::<String>("rate").unwrap().parse().unwrap(),
        matches.get_one::<String>("pause").unwrap().parse().unwrap(),
        ConfigElp::parse_bool(matches.get_one::<String>("comma").unwrap().parse().unwrap()),
        ConfigElp::parse_bool(
            matches
                .get_one::<String>("sayagain")
                .unwrap()
                .parse()
                .unwrap(),
        ),
    );

    // println!("{:?}", my_config);

    let mut config = crate::elp_service::CONFIG.lock().unwrap();
    config.msg_type = my_config.msg_type;
    config.num_msg = my_config.num_msg;
    config.rate = my_config.rate;
    config.pause = my_config.pause;
    config.comma = my_config.comma;
    config.sayagain = my_config.sayagain;

    drop(config);
}

fn main() {
    use crate::elp_service::{clone_config, get_message, say};

    parse_args();

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

        println!("{}", msg.replace('!', ""));
        say(tmp.replace("  ", " "), false, config.comma);

        num_msg -= 1;
    }
}
