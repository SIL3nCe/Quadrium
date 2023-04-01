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

pub mod file_reader;

#[cfg(test)]
mod test_file_reader
{
    use crate::utils::file_reader::read_u128_from_file;
    use super::*;

    #[test]
    fn read_128_bits_in_file()
    {
        let file = match std::fs::File::open("TestFiles/test_file.txt")
        {
            Err(why) => panic!("Could not open the file !"),
            Ok(file) => file,
        };
        let mut result = read_u128_from_file(&file);
        let string_result = "AZERTYUIOPQSDFGF";
        let stringInVecU8 = string_result.as_bytes();
        let mut is_ok : bool = true;
        for i in 0..(stringInVecU8.len() - 1)
        {
            if (result & (0xFF) != stringInVecU8[i].into())
            {
                assert!(false, "i = {0}, result & 0xFF = {1} and stringInVecU8[i] = {2}", i, result & 0xFF, stringInVecU8[i]);
                break;
            }

            if (i + 1 < stringInVecU8.len())
            {
                result >>= 8;
            }
        }
    }
}