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

#[derive(PartialEq)]
pub enum QuAvailableTypeInEvent
{
    String,
    Float,
    Uint8,
    Uint32,
    Uint64,
    Int8,
    Int32,
    Int64,
}


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

pub trait QuInformationData
{
    fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>;
}

pub(crate) mod EventManager;