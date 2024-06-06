mod read_yml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
