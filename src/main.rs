mod read_yml;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let time_table = read_yml::read_yml()?;

    // 読み込んだ内容を表示する
    for time_slot in time_table.time {
        println!(
            "{}: Start: {}, End: {}",
            time_slot.class, time_slot.start, time_slot.end
        );
    }

    Ok(())
}
