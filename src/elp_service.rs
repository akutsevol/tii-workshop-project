use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

use std::sync::Mutex;

const CALL_SIGN: &str = "A6KIB";

#[derive(Clone, Debug)]
pub struct ConfigElp {
    pub msg_type: String,
    pub num_msg: u32,
    pub rate: u32,
    pub pause: u32,
    pub comma: bool,
    pub sayagain: bool,
}

impl ConfigElp {
    pub const fn new(
        msg_type: String,
        num_msg: u32,
        rate: u32,
        pause: u32,
        comma: bool,
        sayagain: bool,
    ) -> ConfigElp {
        ConfigElp {
            msg_type,
            num_msg,
            rate,
            pause,
            comma,
            sayagain,
        }
    }
    pub fn parse_bool(x: u32) -> bool {
        x != 0
    }
}

pub static CONFIG: Lazy<Mutex<ConfigElp>> = Lazy::new(|| {
    let m = ConfigElp::new("ANY".to_string(), 0, 180, 5, false, false);
    Mutex::new(m)
});

pub fn clone_config() -> ConfigElp {
    let config = CONFIG.lock().unwrap();
    config.clone()
}

const VOICE: &str = "ATC0"; // Default voice

const VOICES: [(&str, &str); 10] = [
    ("Pilot", "Daniel"),
    ("ATC0", "Kate"),
    ("ATC1", "Tessa"),
    ("ATC2", "Karen"),
    ("ATC3", "Samantha"),
    ("ATC4", "Karen"),
    ("ATC5", "Karen"),
    ("ATC6", "Karen"),
    ("ATC7", "Karen"),
    ("ATC8", "Serena"),
];

const ALFABET: [(&str, &str); 39] = [
    ("A", "alfa"),
    ("B", "bravo"),
    ("C", "charlie"),
    ("D", "delta"),
    ("E", "echo"),
    ("F", "foxtrot"),
    ("G", "golf"),
    ("H", "hotel"),
    ("I", "india"),
    ("J", "juliett"),
    ("K", "kilo"),
    ("L", "lima"),
    ("M", "mike"),
    ("N", "november"),
    ("O", "oscar"),
    ("P", "papa"),
    ("Q", "quebec"),
    ("R", "romeo"),
    ("S", "sierra"),
    ("T", "tango"),
    ("U", "uniform"),
    ("V", "victor"),
    ("W", "whiskey"),
    ("X", "xray"),
    ("Y", "yankee"),
    ("Z", "zulu"),
    ("1", "wun"),
    ("2", "too"),
    ("3", "tree"),
    ("4", "fower"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "ait"),
    ("9", "niner"),
    ("0", "zero"),
    ("00", "hundred"),
    ("000", "thousand"),
    (".", "decimal"),
];

const MESSAGES: [(&str, u32); 9] = [
    ("*", 1), // flight level
    ("^", 2), // heading
    ("&", 3), // squawk
    ("=", 4), // runway
    ("@", 5), // 117.975 – 137.000 MHz (VHF Aeronautical communications)
    ("!", 6), // QNH
    ("$", 7), // altitude
    ("-", 8), // wind
    ("+", 9), // any message
];

const LR: [&str; 2] = ["left", "right"];

// pub const VALID_MESSAGES: [&str; 2] = ["ANY", "ATIS"];

pub fn get_frequency() -> String {
    let fq_msg = [
        "departure",
        "berlin tower",
        "apron",
        "dubai information",
        "munich approach",
        "munich tower",
        "ground",
        "fujairah information",
    ];

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(117975..=137000);
    x = (x / 5) * 5;
    let mut s_fq = format!("{:03}.{:03}", x / 1000, x % 1000);
    s_fq = s_fq.trim_end_matches('0').trim_end_matches('.').to_string();

    format!(
        "contact {} {}",
        fq_msg[rng.gen_range(0..=(fq_msg.len() - 1))],
        s_fq
    )
    .to_string()
    .trim()
    .to_string()
}

pub fn get_rw() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "runway {:02} {}",
        rng.gen_range(10..=360) / 10,
        LR[rng.gen_range(0..=1)]
    )
    .to_string()
    .trim()
    .to_string()
}

pub fn get_wind() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "wind {:03} degree {} knots",
        (rng.gen_range(10..=360) / 10) * 10,
        rng.gen_range(1..=50)
    )
    .to_string()
    .trim()
    .to_string()
}

pub fn get_squawk() -> String {
    let forbidden = [7500, 7600, 7700];
    let mut squawk_tmp;
    let mut rng = rand::thread_rng();

    loop {
        squawk_tmp = rng.gen_range(2000..=9999);
        if !forbidden.contains(&squawk_tmp) {
            break;
        }
    }
    format!("squawk {:04}", squawk_tmp)
        .to_string()
        .trim()
        .to_string()
}

pub fn get_fl() -> String {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(60..=420);

    format!("flight level {}", (x / 5) * 5)
        .to_string()
        .trim()
        .to_string()
}

pub fn get_altitude() -> String {
    let mut rng = rand::thread_rng();

    let mut hundreds: u32 = 0;
    let mut thousands: u32 = 0;

    let alfabet_map: HashMap<&str, &str> = ALFABET.iter().cloned().collect();

    let x = (rng.gen_range(500..=42000) / 100) * 100;
    if x > 1000 {
        thousands = x / 1000;
    }
    if x > 1000 && (x - (thousands * 1000)) > 0 {
        hundreds = (x - (thousands * 1000)) / 100
    }
    let mut alt_msg = String::new();
    if thousands > 0 {
        alt_msg = format!(
            "{} {} {}",
            alt_msg,
            thousands,
            alfabet_map.get("000").unwrap()
        )
        .to_string();
    }
    if hundreds > 0 {
        alt_msg = format!(
            "{} {} {}",
            alt_msg,
            hundreds,
            alfabet_map.get("00").unwrap()
        )
        .to_string();
    }
    format!("altitude{} feet", alt_msg).to_string()
}

pub fn get_heading() -> String {
    let mut rng = rand::thread_rng();

    let x = (rng.gen_range(10..=360) / 10) * 10;

    format!("heading {:03}", x).to_string().trim().to_string()
}

pub fn get_qnh(imperial: bool) -> String {
    let mut rng = rand::thread_rng();

    let qnh: String = if !imperial || rng.gen_range(0..=1) == 0 {
        let x = rng.gen_range(931..=1066) as u32;
        let qnh = format!("qnh {} hectopascals", x);
        qnh
    } else {
        let x = rng.gen_range(2750..=3150) as f32;
        let qnh = format!("qnh {:.02} inches", x / 100.0);
        qnh
    };

    qnh.trim().to_string()
}

pub fn get_temperature() -> String {
    let mut rng = rand::thread_rng();

    let mut temperature = rng.gen_range(-30..=50);
    let mut dew_point = rng.gen_range((temperature - 15)..=temperature);
    let mut minus_temp = "".to_string();
    let mut minus_dp = "".to_string();
    if temperature < 0 {
        minus_temp = "minus".to_string();
        temperature = 0 - temperature;
    }
    if dew_point < 0 {
        minus_dp = "minus".to_string();
        dew_point = 0 - dew_point;
    }

    format!(
        "temperature {} {} dew point {} {}",
        minus_temp, temperature, minus_dp, dew_point
    )
    .to_string()
    .replace("  ", " ")
    .trim()
    .to_string()
}

pub fn get_random_msg() -> String {
    let any_msg = [
        "engine start-up approved",
        "cleared to munich via T1B departure",
        "cleared to lyon via M2F departure",
        "hold short =, taxi via LFM",
    ];

    let mut rng = rand::thread_rng();

    let mut any_tmp = "".to_string();
    let mut found = "".to_string();

    let alfabet_map: HashMap<&str, &str> = ALFABET.iter().cloned().collect();

    for mut _symbol in any_msg[rng.gen_range(0..=any_msg.len() - 1)].chars() {
        if alfabet_map.contains_key(_symbol.to_string().as_str()) {
            if _symbol != 'T' && !_symbol.is_ascii_digit() {
                _symbol = rng.gen_range('A'..='Z');
                while found.contains(_symbol) {
                    _symbol = rng.gen_range('A'..='Z');
                }
                found.push_str(_symbol.to_string().as_str());
            }
            any_tmp =
                format!("{}{} ", any_tmp, alfabet_map[_symbol.to_string().as_str()]).to_string();
        } else if _symbol == '=' {
            any_tmp = format!("{}{}", any_tmp, get_rw()).to_string();
        } else {
            any_tmp = format!("{}{}", any_tmp, _symbol).to_string();
        }
    }

    any_tmp
        .to_string()
        .to_string()
        .replace("  ", " ")
        .trim()
        .to_string()
}

pub fn get_atis_message() -> String {
    let mut rng = rand::thread_rng();

    let airports = [
        "schiphol",
        "dubai",
        "new york",
        "detroit",
        "abu dhabi",
        "warsaw",
        "berlin",
        "al bateen",
    ];

    let mut atis_msg = "".to_string();

    let info_no = rng.gen_range('A'..='Z');
    atis_msg.push_str(
        format!(
            "this is {} arrival information {}, ",
            airports[rng.gen_range(0..=airports.len() - 1)],
            info_no
        )
        .to_string()
        .as_str(),
    ); // atis_h1
    atis_msg.push_str(format!("main landing {}, ", get_rw()).to_string().as_str()); // atis_h2
    atis_msg.push_str(
        format!("transition level {}, ", (rng.gen_range(50..=100) / 10) * 10)
            .to_string()
            .as_str(),
    ); // atis_h3
    atis_msg.push_str(format!("{}, ", get_wind()).to_string().as_str()); // atis_h4
    atis_msg.push_str(
        format!(
            "visibility !{} metres, ",
            (rng.gen_range(100..=10000) / 100) * 100
        )
        .to_string()
        .as_str(),
    ); // atis_h5
    atis_msg.push_str(
        format!(
            "clouds few !{}, scattered !{}, broken !{}, ",
            (rng.gen_range(1000..=2500) / 100) * 100,
            (rng.gen_range(2500..=3500) / 100) * 100,
            (rng.gen_range(3500..=10000) / 100) * 100
        )
        .to_string()
        .as_str(),
    ); // atis_h6
    atis_msg.push_str(format!("{}, ", get_temperature()).to_string().as_str()); // atis_h7
    atis_msg.push_str(format!("{}, ", get_qnh(false)).to_string().as_str()); // atis_h8
    atis_msg.push_str("no significant change, ".to_string().as_str()); // atis_h9
    atis_msg.push_str(
        format!("end of information {}", info_no)
            .to_string()
            .as_str(),
    ); // atis_h10

    atis_msg.trim().to_string()
}

pub fn get_any_message() -> String {
    let mut messages_map = MESSAGES.to_vec();

    let mut tmp_msg = "".to_string();
    tmp_msg.push_str(format!("{}, ", CALL_SIGN).as_str());
    // shuffle messages
    messages_map.shuffle(&mut rand::thread_rng());

    let mut n = messages_map.len();
    for (symbol, _) in messages_map {
        let mut ss = "".to_string();
        match symbol {
            "=" => {
                ss.push_str(get_rw().to_string().as_str());
            }
            "*" => {
                ss.push_str(get_fl().to_string().as_str());
            }
            "!" => {
                ss.push_str(get_qnh(true).to_string().as_str());
            }
            "$" => {
                ss.push_str(get_altitude().to_string().as_str());
            }
            "@" => {
                ss.push_str(get_frequency().to_string().as_str());
            }
            "^" => {
                ss.push_str(get_heading().to_string().as_str());
            }
            "&" => {
                ss.push_str(get_squawk().to_string().as_str());
            }
            "-" => {
                ss.push_str(get_wind().to_string().as_str());
            }
            "+" => {
                ss.push_str(get_random_msg().as_str());
            }
            _ => {
                ss.push_str(symbol);
            }
        }
        tmp_msg.push_str(ss.as_str());
        n -= 1;
        if n != 0 {
            tmp_msg.push_str(", ");
        }
    }

    tmp_msg.trim().to_string()
}

pub fn get_message(type_of_msg: &str) -> String {
    match type_of_msg {
        "ANY" => get_any_message(),
        "ATIS" => get_atis_message(),
        _ => "Unknown type of the message!".to_string(),
    }
}

pub fn be_ready(mut val: usize) {
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;

    while val > 0 {
        print!(
            "\rBe ready in {} second{}..  \r",
            val,
            if val == 1 { "." } else { "s." }
        );
        io::stdout().flush().unwrap(); // Ensure the buffer is flushed
        val -= 1;
        thread::sleep(Duration::from_secs(1));
    }
    io::stdout().flush().unwrap(); // Ensure the buffer is flushed
}

pub fn say(text: String, flag: bool, comma_pause: bool) -> bool {
    use std::process::Command;
    use std::str;
    use std::thread;
    use std::time::Duration;

    let voices_map: HashMap<&str, &str> = VOICES.iter().cloned().collect();

    let config = clone_config();

    let mut voice = VOICE.to_string();

    // check voice, if defined then set voice and exit
    for (key, val) in &voices_map {
        if text.contains(key) {
            voice.push_str(key);
            println!("{} ({}:)", voice, val);
            return false;
        }
    }
    // delay if "Pause" found
    if text.contains("!Pause") {
        thread::sleep(Duration::from_secs(config.pause as u64));
        return false;
    }

    if comma_pause {
        for split_txt in text.split(',') {
            let tmp_say = split_txt.trim();
            if flag {
                println!("{tmp_say}");
            }
            Command::new("say")
                .arg(format!(
                    "--voice={}",
                    voices_map.get(voice.as_str()).unwrap()
                ))
                .arg(format!("--rate={}", config.rate))
                .arg(format!("\"{}\"", tmp_say))
                .output()
                .expect("failed to execute process");
            thread::sleep(Duration::from_micros(500));
        }
    } else {
        // print and say message
        if flag {
            println!("{}", text);
        }
        Command::new("say")
            .arg(format!(
                "--voice={}",
                voices_map.get(voice.as_str()).unwrap()
            ))
            .arg(format!("--rate={}", config.rate))
            .arg(format!("\"{}\"", text))
            .output()
            .expect("failed to execute process");
    }

    drop(config);
    true
}

pub fn get_alfabet() -> HashMap<&'static str, &'static str> {
    ALFABET.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_rw() {
        use crate::elp_service::get_rw;

        let x = get_rw();
        let mut y = x.replace("runway", "");
        y = y.replace("left", "");
        y = y.replace("right", "").trim().to_string();
        let rw: i32 = y.parse().unwrap();

        assert!(x.contains("runway"));
        assert!(x.contains("left") || x.contains("right"));
        assert!((0..=36).contains(&rw));
    }

    #[test]
    fn test_get_fl() {
        use crate::elp_service::get_fl;

        let x = get_fl();
        let mut y = x.replace("flight", "");
        y = y.replace("level", "").trim().to_string();
        let fl: i32 = y.parse().unwrap();

        assert!(x.contains("flight"));
        assert!(x.contains("level"));
        assert!((60..=420).contains(&fl));
        assert!(fl % 5 == 0);
    }

    #[test]
    fn test_get_wind() {
        use crate::elp_service::get_wind;

        let x = get_wind();
        let mut y = x.replace("wind", "");
        y = y.replace("knots", "").trim().to_string();
        let values: Vec<&str> = y.split("degree").collect();
        let degree_str = values[0].trim().to_string();
        let velocity_str = values[1].trim().to_string();

        let degree: i32 = degree_str.parse().unwrap();
        let velocity: i32 = velocity_str.parse().unwrap();

        assert!(x.contains("wind"));
        assert!(x.contains("degree"));
        assert!(x.contains("knots"));
        assert!((0..=360).contains(&degree));
        assert!(degree % 10 == 0);
        assert!((1..=50).contains(&velocity));
    }

    #[test]
    fn test_get_squawk() {
        use crate::elp_service::get_squawk;

        let forbidden = [7500, 7600, 7700];

        let x = get_squawk();
        let y = x.replace("squawk", "").trim().to_string();
        let sq: i32 = y.parse().unwrap();

        assert!(x.contains("squawk"));
        assert!((2000..=9999).contains(&sq));
        assert!(!forbidden.contains(&sq));
    }

    #[test]
    fn test_get_heading() {
        use crate::elp_service::get_heading;

        let x = get_heading();
        let y = x.replace("heading", "").trim().to_string();
        let h: i32 = y.parse().unwrap();

        assert!(x.contains("heading"));
        assert!((0..=360).contains(&h));
        assert!(h % 10 == 0);
    }

    #[test]
    fn test_get_temperature() {
        use crate::elp_service::get_temperature;

        let x = get_temperature();
        let mut y = x.replace("temperature", "").trim().to_string();
        y = y.replace("minus ", "-").trim().to_string();
        let values: Vec<&str> = y.split("dew point").collect();
        let temp_str = values[0].trim().to_string();
        let dew_point_str = values[1].trim().to_string();

        let temp: i32 = temp_str.parse().unwrap();
        let dew_point: i32 = dew_point_str.parse().unwrap();

        assert!(x.contains("temperature"));
        assert!(x.contains("dew point"));
        assert!((-30..=50).contains(&temp));
        assert!(dew_point >= (temp - 15) && dew_point <= temp);
    }

    #[test]
    fn test_get_altitude() {
        use crate::elp_service::get_altitude;

        let x = get_altitude();
        assert!(x.contains("altitude"));
        assert!(x.contains("feet"));

        let mut y = x.replace("altitude", "").trim().to_string();
        y = y.replace("feet", "").trim().to_string();

        if x.contains("thousand") && x.contains("hundred") {
            y = y.replace("hundred", "").trim().to_string();
            let values: Vec<&str> = y.split("thousand").collect();
            let thousands_str = values[0].trim().to_string();
            let hundreds_str = values[1].trim().to_string();
            let thousands: i32 = thousands_str.parse().unwrap();
            let hundreds: i32 = hundreds_str.parse().unwrap();
            assert!((1..=42).contains(&thousands));
            assert!((1..=9).contains(&hundreds));
        } else if x.contains("thousand") {
            y = y.replace("thousand", "").trim().to_string();
            let thousands: i32 = y.parse().unwrap();
            assert!((1..=42).contains(&thousands));
        } else if x.contains("hundred") {
            y = y.replace("hundred", "").trim().to_string();
            let hundreds: i32 = y.parse().unwrap();
            assert!((1..=9).contains(&hundreds));
        }
    }

    #[test]
    fn test_get_qnh() {
        use crate::elp_service::get_qnh;

        let x = get_qnh(true);
        let mut y = x.replace("qnh", "").trim().to_string();

        if x.contains("hectopascals") {
            y = y.replace("hectopascals", "").trim().to_string();
            let qnh: i32 = y.parse().unwrap();
            assert!((931..=1066).contains(&qnh));
        } else if x.contains("inches") {
            y = y.replace("inches", "").trim().to_string();
            let qnh: f32 = y.parse().unwrap();
            assert!((27.50..=31.50).contains(&qnh));
        } else {
            assert!(x.contains("somethingelse"));
        }
        assert!(x.contains("qnh"));
    }

    #[test]
    fn test_get_frequency() {
        use crate::elp_service::get_frequency;

        let x = get_frequency();
        assert!(x.contains("contact"));

        let frequency_str: String = x.chars().filter(|c| !c.is_alphabetic()).collect();
        let frequency: f32 = frequency_str.trim().parse().unwrap();

        assert!((117.975..=137.0).contains(&frequency))
    }
}
