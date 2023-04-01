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

pub trait QuInformationData
{
    fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>;
}

pub(crate) mod EventManager;