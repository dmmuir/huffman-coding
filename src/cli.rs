use clap::{App, Arg, ArgMatches};

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
                .help("Decode the encoded file to it's original, uncompressed format.")
                .takes_value(false),
        )
        .get_matches()
}
