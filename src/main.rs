use clap::{arg, command, value_parser};
use rand::{rng, seq::IndexedMutRandom};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -c --characters <STRING> "the list of characters to use, non separated, space is not used"
            )
            .default_value("qwerasdfzxcvtgbyuiohjklnmp1234567890*=+/?><QWERTASDFGZXCVBYUIOPHJKLPNM")
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -l --length <INT> "length of the password to generate"
            )
            .default_value("20")
            .value_parser(value_parser!(String)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    let characters = matches
        .get_one::<String>("characters")
        .expect("failed to get the list of characters to use for the password");

    let mut characters: Vec<char> = characters.chars().collect();

    let length: usize = matches
        .get_one::<String>("length")
        .expect("failed to get the length of the password to generate")
        .trim()
        .parse()
        .expect("failed to parse the length of the password to generate");

    let mut password = vec![' '; length];

    let mut rng = rng();

    for x in password.iter_mut() {
        *x = *characters
            .choose_mut(&mut rng)
            .expect("woups, failed to pick the random character");
    }

    let password: String = password.into_iter().collect();

    println!("{password}");
}
