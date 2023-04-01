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

use crate::Controller;
use crate::Controller::QuInformationData;

/// \struct MusicInformation
/// Structure that define all the information that define this music
pub struct AudioInformation
{
    pub m_str_music_name : String,
    pub m_str_music_type : String,
    pub m_str_artist_name : String,
    pub m_str_tracknumber: String,
    pub m_str_album : String,
    pub m_str_date: String,
    pub m_str_duration: String,

    pub m_rate: u32,
    pub m_channel_count: u8,
    pub m_bits_per_sample: u8,
}

impl QuInformationData for AudioInformation
{
    fn convert_to_key_map(&self) -> Vec<(String, crate::Controller::QuAvailableTypeInEvent, String)>
    {
        let mut key_map: Vec<(String, crate::Controller::QuAvailableTypeInEvent, String)> = Vec::new();
        key_map.push(("music_name".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_music_name.clone()));
        key_map.push(("music_type".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_music_type.clone()));
        key_map.push(("artist_name".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_artist_name.clone()));
        key_map.push(("track_number".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_tracknumber.clone()));
        key_map.push(("album".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_album.clone()));
        key_map.push(("date".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_date.clone()));
        key_map.push(("duration".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_str_duration.clone()));
        key_map.push(("track_rate".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_rate.to_string()));
        key_map.push(("channel_count".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_channel_count.to_string()));
        key_map.push(("bits_per_sample".to_string(), Controller::QuAvailableTypeInEvent::string, self.m_bits_per_sample.to_string()));

        return key_map;
    }
}

//
// Declare the module flac_reader to let use the flac reader
// @deprecated
pub mod flac_reader;

/// \interface MusicReader
/// \brief Interface to create reader of music file
/// Quadrium can read different audio files such as WAV, Flac... This interface defines the way to create the reader of these files.
/// This is a private interface. The user will only access to the MusicReaderManager.
pub trait AudioReader
{
    /// \brief Read information about the audio files
    fn read_information(&self, str_path_to_music : String) -> AudioInformation;
}