/*
 *     Quadrium - Music Player in Rust
 *     Copyright (C) 2023  SIL3nCe beta-ray70
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod audio_reader;
mod utils;
mod GUI;
mod Controller;

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

    if args.get(1).unwrap().is_empty()
    {
        panic!("No file given");
    }
    let file_path = &args[1].clone();
    println!("file_path: {0}", file_path);

    let file = std::fs::File::open(file_path.clone())?;
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
    let file_path = file_path;
    let audio_information = reader.read_information(file_path.clone());
    println!("\nAudio Information\nrate: {0}\nbits per sample: {1}\nchannel count: {2}", audio_information.m_rate, audio_information.m_bits_per_sample, audio_information.m_channel_count);
    println!("\nTrack information:\nTrackname: {0}\nArtist: {1}\nAlbum: {2}\nDate: {3}\nGenre: {4}", audio_information.m_str_music_name, audio_information.m_str_artist_name, audio_information.m_str_album, audio_information.m_str_date, audio_information.m_str_music_type);

    GUI::launch_gui();

    Ok(())
}
