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
use crate::Controller::EventManager::{EventManager, QuEvent};
use crate::Controller::QuEventType;

/// Structure containing all the information needed by the GUI
/// Will be used inside the GUI library to update the state
pub struct GUIManager
{
    pub(crate) m_music_information_retrieved: AtomicBool,
    pub(crate) m_current_music_information: Arc<Mutex<Vec<String>>>,
}

/// Function that read the information of an AudioInformation event
/// The information are decoded with the information presented inside the AudioInformation
/// Currently, it is a test function
///
///# Arguments
/// * gui_manager : The current gui_manager
/// * event : The event coming from an AudioInformation
fn read_music_information_from_event(gui_manager: &Arc<GUIManager>, event: &QuEvent::<QuEventType>)
{
    let tuple_informations = event.m_event_arg.convert_to_key_map();
    for tuple_information in tuple_informations
    {
        gui_manager.m_current_music_information.lock().unwrap().push(tuple_information.0.clone() + ": " + tuple_information.2.as_str());
        println!("{0}", tuple_information.0.clone() + ": " + tuple_information.2.as_str());
    }
}

/// Function that will registers all the closures that will be used to listen the events needed by the gui
///
/// # Arguments
/// * gui_manager : the current gui manager
/// * event_manager : the current event_manager
pub fn register_event_listeners(gui_manager: Arc<GUIManager>, event_manager: Arc<Mutex<EventManager::<QuEventType>>>)
{
    let tmp_gui_manager = gui_manager.clone();
    event_manager.lock().unwrap().register_listener(QuEventType::EMusicInformationRetrieved, move |event| {
        read_music_information_from_event(&tmp_gui_manager, event);
    });
}

/// Create the gui manager with all the parameters set to default values
/// MUST be called at the beginning of the application
///
/// # Return
/// Return the gui manager
pub fn create_gui_manager() -> Arc<GUIManager>
{
    let gui_manager = Arc::new(GUIManager
    {
        m_current_music_information: Arc::new(Mutex::new(Vec::new())),
        m_music_information_retrieved: AtomicBool::new(false),
    });

    return gui_manager;
}