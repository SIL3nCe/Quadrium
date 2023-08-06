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

pub mod GUIManager;
mod IcedGUIManager;

use std::fs::File;
use crate::Controller::EventManager::{QuAvailableTypeInEvent, QuInformationData};
use crate::audio_reader::AudioReader;
use iced::{Settings, window};
use iced::Application;

struct AskMusicInformation
{
    m_path_to_file: String,
}

impl QuInformationData for AskMusicInformation
{
    fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>
    {
        let mut vec: Vec<(String, QuAvailableTypeInEvent, String)> = Vec::new();
        vec.push(("path_file".to_string(), QuAvailableTypeInEvent::String, self.m_path_to_file.clone()));
        return vec;
    }
}

pub fn launch_gui()
{
    //
    // Load the icon
    let mut decoder = png::Decoder::new(File::open("Resource/Logo/quadrium_dark_logo.png").unwrap());
    let mut reader = decoder.read_info().unwrap();

    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut img_data).unwrap();

    let result = iced::window::icon::from_rgba(img_data, info.width, info.height);

    //
    // Launch the GUI powered by Iced
    // Will create the event manager and GUI manager as required
    IcedGUIManager::IcedGUIManager::run(
        Settings {
            window : window::Settings
            {
                icon : result.unwrap().into(),
                ..window::Settings::default()
            },
            ..Settings::default()
        }
    );
}