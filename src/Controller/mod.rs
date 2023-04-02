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

/// Enum which contains all the event of the application to communicate between the view and the model
/// To enable new functionality, you MUST create the event view to model and an event model to view
/// The new events created will be automatically accepted by the event manager.
#[derive(PartialEq, Clone)]
pub enum QuEventType
{
    //
    // All input possible

    /// Ask to retrieve musics from a directory
    EAskRetrieveMusicDirectory,

    /// Ask to retrieve the metadata of the music
    EAskRetrieveMusicInformation,

    /// Ask to play/pause/stop the music
    EAskReadMusic,

    /// Ask the creation/suppression/modification of a playlist with a list of musics
    EAskOperationPlaylist,

    /// Ask to retrieve a playlist or all the playlist
    EAskTryRetrievePlayList,

    //
    // All output possible
    /// result of the scan on the directory
    EMusicDirectoryRetrieved,

    /// result of the read of metadata of the music
    EMusicInformationRetrieved,

    /// result of the operation on the music
    EReadMusicState,

    /// result of the operation to apply to a playlist
    EOperationPlaylistState,

    /// result of the retrieving of the playlist
    EPlaylistRetrieved,
}

pub(crate) mod EventManager;