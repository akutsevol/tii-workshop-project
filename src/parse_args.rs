pub fn parse_args() {
    use crate::elp_service::{get_voices, ConfigElp};
    use clap::{Arg, Command};

    let valid_messages = vec!["ANY", "ATIS"]; // Define valid messages as needed
    let mut valid_voices: Vec<&str> = get_voices().unwrap().keys().cloned().collect();
    valid_voices.sort();

    let matches = Command::new("elp")
        .about("ELP training tool for pilots")
        .arg(
            Arg::new("callsign")
                .short('a')
                .long("callsign")
                .default_value("A6KIA")
                .help("Call sign"),
        )
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
        .arg(
            Arg::new("voice")
                .short('v')
                .long("voice")
                .default_value("ATC0")
                .help("Voice name")
                .value_parser(valid_voices.clone()),
        )
        .get_matches();

    let mut config = crate::elp_service::CONFIG.lock().unwrap();
    config.call_sign = matches.get_one::<String>("callsign").unwrap().to_string();
    config.msg_type = matches.get_one::<String>("type").unwrap().to_string();
    config.num_msg = matches.get_one::<String>("count").unwrap().parse().unwrap();
    config.rate = matches.get_one::<String>("rate").unwrap().parse().unwrap();
    config.pause = matches.get_one::<String>("pause").unwrap().parse().unwrap();
    config.comma =
        ConfigElp::parse_bool(matches.get_one::<String>("comma").unwrap().parse().unwrap());
    config.sayagain = ConfigElp::parse_bool(
        matches
            .get_one::<String>("sayagain")
            .unwrap()
            .parse()
            .unwrap(),
    );
    config.voice = matches.get_one::<String>("voice").unwrap().to_string();

    drop(config);
}
