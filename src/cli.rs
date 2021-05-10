use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("mkey")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommands(vec![
            SubCommand::with_name("encode")
                .about("Encode a message for input public_key(other) and secret_key(you)")
                .args(&[
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .takes_value(true)
                        .help("encrypt message"),
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .default_value("")
                        .help("password"),
                ]),
            SubCommand::with_name("generate")
                .about("Generate a random account")
                .args(&[
                    Arg::with_name("words")
                        .short("w")
                        .long("words")
                        .default_value("12")
                        .help("The number of words in the phrase to generate. One of 12 (default), 15, 18, 21 and 24"),
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .default_value("json")
                        .help("output format"),
                    Arg::with_name("network")
                        .short("n")
                        .long("network")
                        .default_value("substrate")
                        .help("Specify a network. One of
                                 polkadot/reserved1/kusama/reserved3/plasm/bifrost/edgeware/karura/reynolds/acala/laminar/polymath/kulupu/darwinia/robonomics/centrifuge/substrate/reserved43/substratee/reserved46/reserved47"),
                    Arg::with_name("amount")
                        .short("a")
                        .long("amount")
                        .default_value("1")
                        .help("amount"),
                ]),
            SubCommand::with_name("decode")
                .about("Decode a message for input public_key(other) and secret_key(you)")
                .args(&[
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .takes_value(true)
                        .help("encrypt message"),
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .default_value("")
                        .help("password"),
                ]),
        ])
}
