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

use std::iter::Once;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use dioxus::prelude::dioxus_hot_reload::init;
use crate::Controller::QuInformationData;

#[derive(PartialEq, Clone)]
pub enum QuEventType
{
    //
    // All input possible
    EAskRetrieveMusicDirectory,
    EAskRetrieveMusicInformation,
    EAskReadMusic,
    EAskCreatePlaylist,
    EAskRemovePlaylist,
    EAskModifyPlaylist,
    EAskTryRetrievePlayList,

    //
    // All output possible
    EMusicDirectoryRetrieved,
    EMusicInformationRetrieved,
    EReadMusicState,
    ECreatePlaylistState,
    ERemovePlaylistState,
    EModifyPlaylistState,
    EPlaylistRetrieved,
}

#[derive(Clone)]
pub struct QuEvent
{
    pub m_event_type: QuEventType,
    pub m_event_arg : Arc<dyn QuInformationData + Send + Sync>,
}

pub struct EventManager
{
    m_event_list : Mutex<Vec<QuEvent>>,
    m_register_event_listeners : Mutex<Vec<(QuEventType,Vec<Arc<Mutex<dyn FnMut(&QuEvent) + Send + Sync + 'static>>>)>>,
    m_need_update : Arc<(Mutex<bool>, Condvar)>,
    m_stop_update : bool,
}

pub struct EventQueue
{
    m_event_list : Mutex<Vec<QuEvent>>,
}

static mut event_queue: EventQueue = EventQueue
{
    m_event_list: Mutex::new(Vec::new()),
};

pub unsafe fn push_event_in_tmp_queue(event: QuEvent)
{
    event_queue.m_event_list.lock().unwrap().push(event);
}

pub unsafe fn get_events_in_tmp_queue() -> Vec<QuEvent>
{
    return event_queue.m_event_list.lock().unwrap().clone();
}

pub unsafe fn clear_event_in_tmp_queue()
{
    event_queue.m_event_list.lock().unwrap().clear();
}

impl EventManager
{
    pub fn push_event(&mut self, event: QuEvent)
    {
        self.m_event_list.lock().unwrap().push(event);
        let (lock, cvar) = &*self.m_need_update;
        let mut started = lock.lock().unwrap();
        *started = true;

        cvar.notify_one();
    }

    pub fn register_listener<Function>(&mut self, event_type: QuEventType, callback: Function) where
        Function: FnMut(&QuEvent) + Send + Sync + 'static
    {
        let mut is_added : bool = false;
        let mut register_event_listeners = self.m_register_event_listeners.lock().unwrap();
        let callback_boxed = Arc::new(Mutex::new(callback));
        for tuple_event_listeners in register_event_listeners.iter_mut()
        {
            if (tuple_event_listeners.0 == event_type)
            {
                tuple_event_listeners.1.push(callback_boxed.clone());
                is_added = true;
                break;
            }
        }

        if (!is_added)
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

    fn update(&mut self)
    {
        //
        // O(n³), very slow algorithm
        // Must run inside its own thread
        let mut event_list = self.m_event_list.lock().unwrap().clone();
        let mut register_event_listeners = self.m_register_event_listeners.lock().unwrap().clone();
        for event in &event_list
        {
            for tuple_event_listeners in register_event_listeners.iter_mut()
            {
                if (event.m_event_type == tuple_event_listeners.0)
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

    pub fn launch(this: Arc<Mutex<Self>>)
    {
        this.lock().unwrap().m_stop_update = false;
        let mut need_update = Arc::clone(&this.lock().unwrap().m_need_update);
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

    pub fn stop(mut self)
    {
        self.m_stop_update = true;
    }
}

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