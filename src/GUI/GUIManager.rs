use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use dioxus::prelude::*;
use crate::Controller::EventManager::{create_event_manager, EventManager, QuEvent, QuEventType};
use crate::audio_reader::AudioInformation;

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

pub fn register_functions(gui_manager: Arc<GUIManager>, event_manager: Arc<Mutex<EventManager>>)
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