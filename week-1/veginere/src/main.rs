use clap::{Arg, ArgAction, Command};

fn encrypt(k :u8, t :u8) {
    let c = ((t + k - 2* ('a' as u8)) % 27) + ('a' as u8);
    print!("{}", c as char); 
}

fn decrypt(k :u8, c :u8) {
    if c >= k {
        let t = c - k + ('a' as u8);
        print!("{}", t as char);
        return
    }

    let t = 27 + ('a' as u8) + c - k ;
    print!("{}", t as char); 
}

fn vigenere(key: &String, text: &String, enc: bool) {
    let stext = text.to_ascii_lowercase();
    let skey  = key.to_ascii_lowercase();

    for ti in  stext.char_indices() {
        if ti.1.is_ascii_alphabetic() {
            let k = skey.chars().nth(ti.0 % skey.len()).unwrap();
            if enc {
                encrypt(k as u8, ti.1 as u8);
            } else {
                decrypt(k as u8, ti.1 as u8);
            }

        }
    }
    println!();
}


fn main() {

    let matches = Command::new("veginer")
                          .version("0.1")
                          .about("Encypt text with vigenere cipher")
						  .arg(Arg::new("key")
						  	  .short('k')
                              .action(ArgAction::Set)
                              .value_name("key")
                              .required(true)
                              .help("Key"))
						  .arg(Arg::new("text")
						  	  .short('t')
                              .action(ArgAction::Set)
                              .value_name("text")
                              .required(false)
                              .conflicts_with("cypher")
                              .help("Text"))
                          .arg(Arg::new("cypher")
                              .short('c')
                              .action(ArgAction::Set)
                              .value_name("cypher")
                              .required(false)
                              .conflicts_with("text")
                              .help("Cypher text"))
                          .get_matches();

    match matches.get_one::<String>("text") {
        Some(t) => vigenere(matches.get_one::<String>("key").unwrap(), t, true),
        None => {},
    }

    match matches.get_one::<String>("cypher") {
        Some(c) => vigenere(matches.get_one::<String>("key").unwrap(), c, false),
        None => {},
    }
}
