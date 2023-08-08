use anyhow::Result;
use std::{
    fs::{self, create_dir_all},
    io::{BufWriter, Write},
    path::PathBuf,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.csv")?;
    let output_dir = PathBuf::from("output");
    create_dir_all(&output_dir)?;

    for line in input
        .lines()
        .map(|l| l.trim())
        .filter(|line| !line.is_empty())
    {
        let (prefix, range) = line.split_once(',').expect("invalid input file");

        let output = output_dir.join(prefix.to_owned() + ".txt");
        let output_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(output)?;

        let mut writer = BufWriter::new(output_file);

        let (start, end) = range
            .split_once('-')
            .expect("range should be splitted by \'-\'");
        let start = u32::from_str_radix(start, 16)?;
        let end = u32::from_str_radix(end, 16)?;

        for character in
            (start..=end).map(|code_point| unsafe { char::from_u32_unchecked(code_point) })
        {
            writer.write_fmt(format_args!("{character}"))?;
        }
    }
    Ok(())
}
