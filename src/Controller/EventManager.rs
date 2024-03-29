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

//! Mod which implements event manager to communicate through events.
//!
//! Primarily designed to interact between GUI and Models in a MVP (Model View Projector) design pattern

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

///
/// Enum defining all the parameters' type accepted for QuInformationData
pub enum QuAvailableTypeInEvent
{
    /// String parameters
    String,

    /// float parameters
    Float,

    /// uint8 parameters
    Uint8,

    /// uint32 parameters
    Uint32,

    /// uint64 parameters
    Uint64,

    /// int8 parameters
    Int8,

    /// int32 parameters
    Int32,

    /// int64 parameters
    Int64,
}

/// Trait allowing to send data through event.
/// Contains only one function which allows to convert attributes of the implemented structure
/// inside a tuple of fields defined by the string of the fields, the types of the parameters and
/// the value of the parameters encoded inside a string
pub trait QuInformationData
{
    ///
    /// Convert all the attributes inside the implemented structure to an array of tuple.
    /// The tuple contains in order :
    /// 1. The field in String
    /// 2. The type of the parameters
    /// 3. The parameters encoded inside a string
    ///
    /// # Params
    /// self: Reference to the QuInformationData itself
    ///
    /// # Return
    /// A vector of tuples containing all the attributes encoded
    fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>;
}

///
/// Declaration of an event inside the event manager of Quadrium
/// All the event MUST contains a type and a list of QuInformationData.
/// The type of an event must be specified by the application and it is recommended to use enum structure.
/// Each event must contains a standard definition to allow interaction between the view and the model
///
/// # Attributes
/// * m_event_type: the type of the event
/// * m_event_arg: the information sent through the event
#[derive(Clone)]
pub struct QuEvent<EventType>
{
    pub m_event_type: EventType,  // The type of the event
    pub m_event_arg : Arc<dyn QuInformationData + Send + Sync>, // All the information sent inside the event
}

///
/// The event manager is designed to help interaction between the view and the model.
/// This is the projection in the MVP (Model View Projector), a sub class of  the MVC (Model View Controller).
/// It is modular and accept a large choice of event. The processing of the events is realized inside its own thread.
/// Even if it is not a Singleton to avoid unsafe block, it is recommended to create only one event manager.
///
/// # How to use it
/// You must create the event manager in the beginning of the application with the function :
/// ```
/// let event_manager: Arc<Mutex<EventManager>> = EventManager::create_event_manager::<EventType>();
/// ```
///
/// Then you can register listeners for an event type with a closures :
/// ```
/// event_manager.lock().unwrap().register_listener(EventType::EEventAsked, |event|
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
/// struct AskMusicInformation
/// {
///     m_path_to_file: String,
/// }
///
/// impl QuInformationData for AskMusicInformation
/// {
///     fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>
///     {
///         let mut vec: Vec<(String, QuAvailableTypeInEvent, String)> = Vec::new();
///         vec.push(("path_file".to_string(), QuAvailableTypeInEvent::String,
///             self.m_path_to_file.clone()));
///         return vec;
///     }
/// }
///
/// let request_music_information = AskMusicInformation {
///     m_path_to_file: "the/path/to/file",
/// };
/// event_manager.lock().unwrap().push_event(QuEvent::<EventType>
/// {
///     m_event_type: EventType::EEventToSend,
///     m_event_arg: Arc::new(request_music_information),
/// });
/// ```
///
/// If you send an event inside a listener, it is mandatory to push the event inside the temporary queue.
/// Example :
/// ```
/// let tmp_event_queue = event_manager.lock().unwrap().get_temporary_queue().clone();
/// event_manager.lock().unwrap().register_listener(QuEventType::EAskRetrieveMusicInformation,
///     move |event|
/// {
///     let argument = event.m_event_arg.convert_to_key_map();
///
///     if argument.len() != 1
///     {
///         return;
///     }
///
///     let flac_reader = audio_reader::flac_reader::FlacReader
///     {
///     };
///
///     let audio_information = flac_reader.read_information(argument[0].2.clone());
///     let event_to_send = QuEvent::<QuEventType>
///     {
///         m_event_type: QuEventType::EMusicInformationRetrieved,
///         m_event_arg: Arc::new(audio_information),
///     };
///     push_event_in_tmp_queue(event_to_send, tmp_event_queue.clone());
/// });
/// ```
pub struct EventManager<EventType>
where EventType: PartialEq + Clone
{
    /// The list of all event sent
    m_event_list : Mutex<Vec<QuEvent<EventType>>>,

    /// The list of all event listeners defining by a callback
    m_register_event_listeners : Mutex<Vec<(EventType,Vec<Arc<Mutex<dyn FnMut(&QuEvent<EventType>) + Send + Sync + 'static>>>)>>,

    /// Tuples containing a boolean to know if the event manager need to process event and a condition variable
    /// to allow passive wait.
    m_need_update : Arc<(Mutex<bool>, Condvar)>,

    /// boolean to stop the process of the event
    m_stop_update : bool,

    /// temporary queue used to save event push during the process of the event
    m_tmp_event_queue : Arc<Mutex<EventQueue<EventType>>>,
}

/// A structure which contains all the event sent
///
/// # Attribute
/// * m_event_list: a thread-safe list of event
pub struct EventQueue<EventType>
{
    m_event_list : Mutex<Vec<QuEvent<EventType>>>,
}

/// Function to push event inside the temporary queue
///
/// # Parameter
/// event: The event to push inside the temporary queue.
pub fn push_event_in_tmp_queue<EventType>(event: QuEvent<EventType>, tmp_event_queue: Arc<Mutex<EventQueue<EventType>>>)
{
    tmp_event_queue.lock().unwrap().m_event_list.lock().unwrap().push(event);
}

impl<EventType> EventManager<EventType>
    where EventType: PartialEq + Clone + Send + Sync + 'static
{
    /// Push an event inside the event manager
    /// MUST be used only if the event is send not inside a lock of the event manager
    ///
    /// # Parameters
    /// * self: a mutable references of the event manager
    /// * event: the event to push
    pub fn push_event(&mut self, event: QuEvent<EventType>)
    {
        self.m_event_list.lock().unwrap().push(event);
        let (lock, cvar) = &*self.m_need_update;
        let mut started = lock.lock().unwrap();
        *started = true;

        cvar.notify_one();
    }

    /// Register a listener to a specific event type.
    /// The callback must be a closure compatible with thread.
    ///
    /// # Parameters
    /// * self: a mutable references of the event manager
    /// * callback: a function/closures which take as parameter a QuEvent and returns nothing
    pub fn register_listener<Function>(&mut self, event_type: EventType, callback: Function) where
        Function: FnMut(&QuEvent<EventType>) + Send + Sync + 'static
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
            let mut initial_vec: Vec<Arc<Mutex<dyn FnMut(&QuEvent<EventType>) + Send + Sync>>> = Vec::new();
            initial_vec.push(callback_boxed.clone());
            register_event_listeners.push((event_type, initial_vec));
        }

        let (lock, cvar) = &*self.m_need_update;
        let mut started = lock.lock().unwrap();
        *started = true;

        cvar.notify_one();
    }

    pub fn get_temporary_queue(&self) -> Arc<Mutex<EventQueue<EventType>>>
    {
        return self.m_tmp_event_queue.clone();
    }

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

        let mut event_list = self.m_event_list.lock().unwrap();
        let mut tmp_queue = self.m_tmp_event_queue.lock().unwrap();
        for event in tmp_queue.m_event_list.lock().unwrap().iter()
        {
            event_list.push(event.clone());
        }
        tmp_queue.m_event_list.lock().unwrap().clear();
    }

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

    /// Stop the process events threads
    ///
    /// # Parameters
    /// self : a mutable event manager
    pub fn stop(mut self)
    {
        self.m_stop_update = true;
    }
}

/// Create the event managers with all the attributes initialize to default values/
///
/// # Return
/// Return a thread compatible event manager
pub fn create_event_manager<EventType>() -> Arc<Mutex<EventManager<EventType>>>
    where EventType: PartialEq + Clone + Send + Sync + 'static
{
    let event_manager = Arc::new(Mutex::new(
        EventManager
        {
            m_event_list: Mutex::new(Vec::new()),
            m_register_event_listeners: Mutex::new(Vec::new()),
            m_stop_update: false,
            m_need_update: Arc::new((Mutex::new(false), Condvar::new())),
            m_tmp_event_queue: Arc::new(Mutex::new(EventQueue::<EventType>{
                m_event_list: Mutex::new(Vec::new()),
            })),
        }
    ));
    return event_manager;
}