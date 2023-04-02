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

use dioxus::prelude::*;
use crate::Controller::EventManager::{create_event_manager, EventManager, QuEvent, QuAvailableTypeInEvent, QuInformationData};
use crate::Controller::{QuEventType};
use std::sync::Arc;
use crate::audio_reader;
use crate::audio_reader::AudioReader;

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
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element
{
    let event_manager = create_event_manager::<QuEventType>();
    let use_gui_manager = GUIManager::create_gui_manager();

    GUIManager::register_event_listeners(use_gui_manager, event_manager.clone());
    audio_reader::register_event_listeners(event_manager.clone());
    EventManager::launch(event_manager.clone());

    cx.render(rsx!
    {
        div
        {
            "Quadrium Music Player"
        }

        button
        {
            onclick: move |event|
            {
                let args: Vec<String> = std::env::args().collect();
                let request_music_information = AskMusicInformation {
                    m_path_to_file: args[1].clone(),
                };
                event_manager.lock().unwrap().push_event(QuEvent::<QuEventType>
                {
                    m_event_type: QuEventType::EAskRetrieveMusicInformation,
                    m_event_arg: Arc::new(request_music_information),
                });
            },

            "Get information data !"
        }
    })
}