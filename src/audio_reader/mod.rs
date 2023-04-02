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

use std::sync::{Arc, Mutex};
use crate::{audio_reader, Controller};
use crate::Controller::EventManager::{EventManager, push_event_in_tmp_queue, QuEvent};
use crate::Controller::{QuEventType, QuInformationData};

/// AudioInformation
/// Structure that define all the information needed to define a title
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
    ///
    /// Implementation of the function which transform all the information to a vector of tuples
    fn convert_to_key_map(&self) -> Vec<(String, crate::Controller::QuAvailableTypeInEvent, String)>
    {
        let mut key_map: Vec<(String, crate::Controller::QuAvailableTypeInEvent, String)> = Vec::new();
        key_map.push(("music_name".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_music_name.clone()));
        key_map.push(("music_type".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_music_type.clone()));
        key_map.push(("artist_name".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_artist_name.clone()));
        key_map.push(("track_number".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_tracknumber.clone()));
        key_map.push(("album".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_album.clone()));
        key_map.push(("date".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_date.clone()));
        key_map.push(("duration".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_str_duration.clone()));
        key_map.push(("track_rate".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_rate.to_string()));
        key_map.push(("channel_count".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_channel_count.to_string()));
        key_map.push(("bits_per_sample".to_string(), Controller::QuAvailableTypeInEvent::String, self.m_bits_per_sample.to_string()));

        return key_map;
    }
}

///
/// Register all the event listeners dedicated to the audio/music
///
/// #Params
/// event_manager: the event manager of the application
pub fn register_event_listeners(event_manager: Arc<Mutex<EventManager::<QuEventType>>>)
{
    let tmp_event_queue = event_manager.lock().unwrap().get_temporary_queue().clone();
    event_manager.lock().unwrap().register_listener(QuEventType::EAskRetrieveMusicInformation, move |event| {
        let argument = event.m_event_arg.convert_to_key_map();
        if argument.len() != 1
        {
            return;
        }

        let flac_reader = audio_reader::flac_reader::FlacReader
        {

        };

        let audio_information = flac_reader.read_information(argument[0].2.clone());

        let event_to_send = QuEvent::<QuEventType>
        {
            m_event_type: QuEventType::EMusicInformationRetrieved,
            m_event_arg: Arc::new(audio_information),
        };

        push_event_in_tmp_queue(event_to_send, tmp_event_queue.clone());
    });
}

//
// Declare the module flac_reader to let use the flac reader
// @deprecated
pub mod flac_reader;

/// \interface AudioReader
/// \brief Interface to create reader of music file
/// Quadrium can read different audio files such as WAV, Flac... This interface defines the way to create the reader of these files.
/// This is a private interface. The user will only access to the MusicReaderManager.
pub trait AudioReader
{
    /// \brief Read information about the audio files
    fn read_information(&self, str_path_to_music : String) -> AudioInformation;
}