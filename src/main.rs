mod audio_reader;
mod utils;
mod GUI;
mod Controller;

use std::fs::read;
use std::io::{Seek, SeekFrom};
use crate::audio_reader::flac_reader::FlacReader;
use crate::audio_reader::AudioReader;
use std::env;
use crate::Controller::EventManager::EventManager;
use crate::GUI::GUIManager::GUIManager;

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
    println!("\nAudio Information\nrate: {0}\nbits per sample: {1}\nchannel count: {2}", audio_information.m_rate, audio_information.m_bits_per_sample, audio_information.m_channel_count);
    println!("\nTrack information:\nTrackname: {0}\nArtist: {1}\nAlbum: {2}\nDate: {3}", audio_information.m_str_music_name, audio_information.m_str_artist_name, audio_information.m_str_album, audio_information.m_str_date);

    GUI::launch_gui();

    Ok(())
}
