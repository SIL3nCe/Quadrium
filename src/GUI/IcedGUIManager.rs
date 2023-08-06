/*
 *     Quadrium
 *     Copyright (C) 2023  SIL3nCe beta-ray70
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
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
use iced::{executor, Length, Subscription};
use iced::{Application, Command, Element, Settings, Theme};
use iced::Length::Fill;
use iced::theme::Text;
use iced::widget::{button, column, text, container, Column, row};
use crate::Controller::EventManager::{create_event_manager, EventManager, QuEvent};
use crate::Controller::QuEventType;
use crate::{audio_reader, GUI};
use crate::GUI::{AskMusicInformation};
use crate::GUI::GUIManager::*;

pub struct IcedGUIManager
{
    event_manager: Arc<Mutex<EventManager<QuEventType>>>,
    gui_manager: Arc<GUIManager>
}

#[derive(Debug, Clone, Copy)]
pub enum EQuMessage
{
    e_load_current_track_info,
}

impl iced::application::Application for IcedGUIManager
{
    type Executor = executor::Default;
    type Flags = ();
    type Message = EQuMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (IcedGUIManager, Command<Self::Message>)
    {
        let event_manager = create_event_manager::<QuEventType>();
        let use_gui_manager = create_gui_manager();

        register_event_listeners(use_gui_manager.clone(), event_manager.clone());
        audio_reader::register_event_listeners(event_manager.clone());
        EventManager::launch(event_manager.clone());

        let icedGuiManager = IcedGUIManager
        {
            event_manager: event_manager.clone(),
            gui_manager: use_gui_manager.clone()
        };

        (icedGuiManager, Command::none())
    }

    fn title(&self) -> String
    {
        String::from("Quadrium Music Player")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>
    {
        //
        // Treat event of iced
        match message
        {
            EQuMessage::e_load_current_track_info =>
                {
                    let args: Vec<String> = std::env::args().collect();
                    let request_music_information = AskMusicInformation {
                        m_path_to_file: args[1].clone(),
                    };
                    self.event_manager.lock().unwrap().push_event(QuEvent::<QuEventType>
                    {
                        m_event_type: QuEventType::EAskRetrieveMusicInformation,
                        m_event_arg: Arc::new(request_music_information),
                    });
                }
        }
        Command::none()
    }

    fn view(&self) -> Element<EQuMessage>
    {
        let current_music_information = self.gui_manager.m_current_music_information.lock().unwrap();
        let current_music_information = if !current_music_information.is_empty()
        {
            column![
                text("Toto")
            ]
        }
        else
        {
            column![]
        };

        let content = column![
            button("Retrieve Music information").on_press(EQuMessage::e_load_current_track_info),
            current_music_information,
        ];

        return container(content).width(Length::Fill).height(Length::Fill).into();
    }
}