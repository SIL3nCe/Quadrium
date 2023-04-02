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

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use crate::Controller::QuInformationData;

/// Enum which contains all the event of the application to communicate between the view and the model
/// To enable new functionality, you MUST create the event view to model and an event model to view
/// The new events created will be automatically accepted by the event manager.
#[derive(PartialEq, Clone)]
pub enum QuEventType
{
    //
    // All input possible
    EAskRetrieveMusicDirectory, // Ask to retrieve musics from a directory
    EAskRetrieveMusicInformation,   // Ask to retrieve the metadata of the music
    EAskReadMusic,  // Ask to play/pause/stop the music
    EAskOperationPlaylist, // Ask the creation/suppression/modification of a playlist with a list of musics
    EAskTryRetrievePlayList,    // Ask to retrieve a playlist or all the playlist

    //
    // All output possible
    EMusicDirectoryRetrieved,   // result of the scan on the directory
    EMusicInformationRetrieved, // result of the read of metadata of the music
    EReadMusicState,    // result of the operation on the music
    EOperationPlaylistState,    // result of the operation to apply to a playlist
    EPlaylistRetrieved, // result of the retrieving of the playlist
}

///
/// Declaration of an event inside the event manager of Quadrium
/// All the event MUST contains a type and a list of QuInformationData.
/// Each event must contains a standard definition to allow interaction between the view and the model
///
/// # Attributes
/// * m_event_type: the type of the event
/// * m_event_arg: the information sent through the event
#[derive(Clone)]
pub struct QuEvent
{
    pub m_event_type: QuEventType,  // The type of the event
    pub m_event_arg : Arc<dyn QuInformationData + Send + Sync>, // All the information sent inside the event
}

///
/// The event manager which will be used to help interaction between the view and the model
/// This is the projection in the MVP (Model View Projector), a sub class of  the MVC (Model View Controller).
/// It is modular and accept a large choice of event. The processing of the events is realized inside its own thread.
/// Even if it is not a Singleton to avoid unsafe block, it is recommended to create only one event manager.
///
/// # How to use it
/// You must create the event manager in the beginning of the application with the function :
/// ```
/// let event_manager: Arc<Mutex<EventManager>> = EventManager::create_event_manager();
/// ```
///
/// Then you can register listeners for an event type with a closures :
/// ```
/// event_manager.lock().unwrap().register_listener(QuEventType::EMusicInformationRetrieved, |event|
/// {
///     // Your closures code
/// });
/// ```
/// It is important that the closure is compatible with multi-threading.
///
/// Then, the event manager must be launched manually currently :
/// ```
/// event_manager.launch();
/// ```
///
/// To send event inside the event manager, you need to call the push_event function like this :
/// ```
/// let request_music_information = AskMusicInformation {
///     m_path_to_file: "the/path/to/file",
/// };
/// event_manager.lock().unwrap().push_event(QuEvent
/// {
///     m_event_type: QuEventType::EAskRetrieveMusicInformation,
///     m_event_arg: Arc::new(request_music_information),
/// });
/// ```
///
/// If you send an event inside a listener, it is mandatory to push the event inside the temporary queue.
/// Example :
/// ```
/// event_manager.lock().unwrap().register_listener(QuEventType::EAskRetrieveMusicInformation, move |event| {
///     let argument = event.m_event_arg.convert_to_key_map();
///     if argument.len() != 1
///     {
///         return;
///     }
///
///     let flac_reader = audio_reader::flac_reader::FlacReader
///     {
///
///     };
///
///     let audio_information = flac_reader.read_information(argument[0].2.clone());
///
///     let event_to_send = QuEvent
///     {
///         m_event_type: QuEventType::EMusicInformationRetrieved,
///         m_event_arg: Arc::new(audio_information),
///     };
///
///     push_event_in_tmp_queue(event_to_send);
/// });
/// ```
pub struct EventManager
{
    m_event_list : Mutex<Vec<QuEvent>>,
    m_register_event_listeners : Mutex<Vec<(QuEventType,Vec<Arc<Mutex<dyn FnMut(&QuEvent) + Send + Sync + 'static>>>)>>,
    m_need_update : Arc<(Mutex<bool>, Condvar)>,
    m_stop_update : bool,
}

/// A structure which contains all the event sent
/// Currently only used inside the singleton EVENT_QUEUE as a temporary event list during the update
///
/// # Attribute
/// * m_event_list: a thread-safe list of event
pub struct EventQueue
{
    m_event_list : Mutex<Vec<QuEvent>>,
}

///
/// Singleton used to save all the event send during the update process
/// Currently unsafe block to use it because rust do not accept static mut as safe code.
/// However the code is nearly safe.
static mut EVENT_QUEUE: EventQueue = EventQueue
{
    m_event_list: Mutex::new(Vec::new()),
};

///
/// Function to push event inside the temporary queue
///
/// # Parameter
/// event: The event to push inside the temporary queue.
pub fn push_event_in_tmp_queue(event: QuEvent)
{
    unsafe
        {
            EVENT_QUEUE.m_event_list.lock().unwrap().push(event);
        }
}

///
/// Get the events inside the temporary queue
pub fn get_events_in_tmp_queue() -> Vec<QuEvent>
{
    unsafe
        {
            return EVENT_QUEUE.m_event_list.lock().unwrap().clone();
        }
}

///
/// Clears the temporary queue
pub fn clear_event_in_tmp_queue()
{
    unsafe
        {
            EVENT_QUEUE.m_event_list.lock().unwrap().clear();
        }
}

impl EventManager
{
    ///
    /// Push an event inside the event manager
    /// MUST be used only if the event is send not inside a lock of the event manager
    ///
    /// # Parameters
    /// * self: a mutable references of the event manager
    /// * event: the event to push
    pub fn push_event(&mut self, event: QuEvent)
    {
        self.m_event_list.lock().unwrap().push(event);
        let (lock, cvar) = &*self.m_need_update;
        let mut started = lock.lock().unwrap();
        *started = true;

        cvar.notify_one();
    }

    ///
    /// Register a listener to a specific event type.
    /// The callback must be a closure compatible with thread.
    ///
    /// # Parameters
    /// * self: a mutable references of the event manager
    /// * callback: a function/closures which take as parameter a QuEvent and returns nothing
    pub fn register_listener<Function>(&mut self, event_type: QuEventType, callback: Function) where
        Function: FnMut(&QuEvent) + Send + Sync + 'static
    {
        let mut is_added : bool = false;
        let mut register_event_listeners = self.m_register_event_listeners.lock().unwrap();
        let callback_boxed = Arc::new(Mutex::new(callback));
        for tuple_event_listeners in register_event_listeners.iter_mut()
        {
            if tuple_event_listeners.0 == event_type
            {
                tuple_event_listeners.1.push(callback_boxed.clone());
                is_added = true;
                break;
            }
        }

        if !is_added
        {
            let mut initial_vec: Vec<Arc<Mutex<dyn FnMut(&QuEvent) + Send + Sync>>> = Vec::new();
            initial_vec.push(callback_boxed.clone());
            register_event_listeners.push((event_type, initial_vec));
        }

        let (lock, cvar) = &*self.m_need_update;
        let mut started = lock.lock().unwrap();
        *started = true;

        cvar.notify_one();
    }

    ///
    /// Process all the events inside the event manager.
    /// Will call all callback registered with the events.
    ///
    /// # Parameter
    /// * self : a mutable references of the event_manager
    fn update(&mut self)
    {
        //
        // O(n³), very slow algorithm
        // Must run inside its own thread
        let event_list = self.m_event_list.lock().unwrap().clone();
        let mut register_event_listeners = self.m_register_event_listeners.lock().unwrap().clone();
        for event in &event_list
        {
            for tuple_event_listeners in register_event_listeners.iter_mut()
            {
                if event.m_event_type == tuple_event_listeners.0
                {
                    for callback in tuple_event_listeners.1.iter_mut()
                    {
                        //
                        // Lock the callback
                        // Will be unlock after the iteration
                        callback.lock().unwrap()(event);
                    }
                }
            }
        }
        self.m_event_list.lock().unwrap().clear();

        //
        // Nearly safe
        unsafe
            {
                self.m_event_list = Mutex::from(get_events_in_tmp_queue());
                clear_event_in_tmp_queue();
            }
    }

    ///
    /// Launch the process event thread
    ///
    /// # Parameters
    /// this: an thread compatible event manager
    pub fn launch(this: Arc<Mutex<Self>>)
    {
        this.lock().unwrap().m_stop_update = false;
        let need_update = Arc::clone(&this.lock().unwrap().m_need_update);
        thread::spawn(move || {
           while !this.lock().unwrap().m_stop_update
           {
               //
               // https://doc.rust-lang.org/std/sync/struct.Condvar.html
               let (lock, cvar) = &*need_update;
               let mut need_update_bool = *lock.lock().unwrap();
               while !need_update_bool
               {
                   cvar.wait(lock.lock().unwrap());
                   need_update_bool = *lock.lock().unwrap();
               }

               this.lock().unwrap().update();

               *lock.lock().unwrap() = !this.lock().unwrap().m_event_list.lock().unwrap().is_empty();
               cvar.notify_one();
           }
        });
    }

    ///
    /// Stop the process events threads
    ///
    /// #Parameters
    /// self : a mutable event manager
    pub fn stop(mut self)
    {
        self.m_stop_update = true;
    }
}

///
/// Create the event managers with all the attributes initialize to default values/
///
/// #Return
/// Return a thread compatible event manager
pub fn create_event_manager() -> Arc<Mutex<EventManager>>
{
    let event_manager = Arc::new(Mutex::new(
        EventManager
        {
            m_event_list: Mutex::new(Vec::new()),
            m_register_event_listeners: Mutex::new(Vec::new()),
            m_stop_update: false,
            m_need_update: Arc::new((Mutex::new(false), Condvar::new())),
        }
    ));
    return event_manager;
}