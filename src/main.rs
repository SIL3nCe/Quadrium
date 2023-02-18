mod audio_reader;
mod utils;

use std::fs::read;
use std::io::{Seek, SeekFrom};
use crate::audio_reader::flac_reader::FlacReader;
use crate::audio_reader::AudioReader;
use std::env;

fn main() -> std::io::Result<()>
{
    println!("Quadrium : Music Player");

    let args: Vec<String> = env::args().collect();
    if args.len() == 0
    {
        panic!("Not enough arguments. Usage: \nQuadrium path/file/to/flac.flac");
    }

    if (args.get(1).unwrap().is_empty())
    {
        panic!("No file given");
    }
    let mut filePath = &args[1].clone();
    println!("filePath: {0}", filePath);

    let mut file = std::fs::File::open(filePath.clone())?;
    let b_is_flac_file = audio_reader::flac_reader::is_flac_file(&file);
    if b_is_flac_file
    {
        println!("It's a flac file");
    }
    else
    {
        println!("It's not a flac file");
    }

    let reader: FlacReader = FlacReader {};
    let file_path = filePath;
    let audio_information = reader.read_information(file_path.clone());
    println!("Audio Information\nrate: {0}\nbits per sample: {1}\nchannel count: {2}", audio_information.m_rate, audio_information.m_bitsPerSample, audio_information.m_channelCount);

    Ok(())
}
