use clap::{App, Arg, ArgMatches, SubCommand};

pub fn app() -> ArgMatches<'static> {
    App::new("huff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("(De)Compress files with huffman trees.")
        .arg(
            Arg::with_name("filepath")
                .short("f")
                .long("filepath")
                .value_name("FILE")
                .help("Path of file to compress")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .help("Decode the encoded source to it's original, uncompressed format.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("stats")
                .short("stats")
                .long("stats")
                .help("Encodes source to produce compression statitics.")
                .takes_value(false),
        )
        .subcommand(
            SubCommand::with_name("stats")
                .help("Retrieves statistics on the compressed file.")
                .arg(
                    Arg::with_name("file")
                    .help("The file to get stats from")
                    .takes_value(true),
                )
        )
        .get_matches()
}
