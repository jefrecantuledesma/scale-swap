use expanduser::expanduser;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = expanduser("~/.config/sway/config").unwrap();
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Temporary path to write updated configuration
    let temp_path = Path::new("/home/fribbit/.config/sway/config_temp");
    let temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_path)?;

    let mut writer = BufWriter::new(temp_file);

    for line in reader.lines() {
        let line = line?;
        let modified_line = if line
            .contains(r##""BOE 0x0BCA Unknown" resolution 2256x1504@59.999HZ scale 1.5 "##)
        {
            line.replace("scale 1.5", "scale 1")
        } else if line.contains(r##""BOE 0x0BCA Unknown" resolution 2256x1504@59.999HZ scale 1 "##)
        {
            line.replace("scale 1", "scale 1.5")
        } else {
            line // No change if neither condition is met
        };
        writeln!(writer, "{}", modified_line)?;
    }

    // Ensure all data is flushed to disk and close files
    writer.flush()?;

    // Rename the temporary file to replace the old one
    std::fs::rename(temp_path, path)?;

    std::process::Command::new("swaymsg")
        .arg("reload")
        .spawn()
        .expect("Could not reload Swag.");

    Ok(())
}
