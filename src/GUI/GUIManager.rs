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
use std::sync::atomic::AtomicBool;
use crate::Controller::EventManager::{EventManager, QuEvent, QuEventType};

pub struct GUIManager
{
    pub(crate) m_music_information_retrieved: AtomicBool,
    pub(crate) m_current_music_information: Arc<Mutex<Vec<String>>>,
}

fn read_music_information_from_event(gui_manager: &Arc<GUIManager>, event: &QuEvent)
{
    let tuple_informations = event.m_event_arg.convert_to_key_map();
    for tuple_information in tuple_informations
    {
        gui_manager.m_current_music_information.lock().unwrap().push(tuple_information.0.clone() + ": " + tuple_information.2.as_str());
        println!("{0}", tuple_information.0.clone() + ": " + tuple_information.2.as_str());
    }
}

pub fn register_event_listeners(gui_manager: Arc<GUIManager>, event_manager: Arc<Mutex<EventManager>>)
{
    let tmp_gui_manager = gui_manager.clone();
    event_manager.lock().unwrap().register_listener(QuEventType::EMusicInformationRetrieved, move |event| {
        read_music_information_from_event(&tmp_gui_manager, event);
    });
}

pub fn create_gui_manager() -> Arc<GUIManager>
{
    let gui_manager = Arc::new(GUIManager
    {
        m_current_music_information: Arc::new(Mutex::new(Vec::new())),
        m_music_information_retrieved: AtomicBool::new(false),
    });

    return gui_manager;
}