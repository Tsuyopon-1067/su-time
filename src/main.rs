use clap::{App, Arg};

fn main() {
    let matches = App::new("sutime")
        .version("0.1.0")
        .author("Tsuyopon_1067")
        .about("show a timetable")
        .arg(
            Arg::with_name("all")
                .short("a")
                .help("print all timetable")
                .takes_value(false),
        )
        .get_matches();

    let all = matches.is_present("all");

    println!("{}", all);
}
