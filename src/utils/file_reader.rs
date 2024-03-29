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

use std::io::Read;

pub fn read_u8_from_file(mut file: &std::fs::File) -> u8
{
    let u8_value : u8;
    let mut au8_buffer_magic = [0; 1];
    let _result = file.read_exact(&mut au8_buffer_magic);
    u8_value = au8_buffer_magic[0];
    if cfg!(target_endian = "big")
    {
        return u8_value;
    }
    else
    {
        return u8_value.swap_bytes();
    }
}

pub fn read_u16_from_file(mut file: &std::fs::File) -> u16
{
    let u16_value : u16;
    let mut au8_buffer = [0; 2];
    let _result = file.read_exact(&mut au8_buffer);
    u16_value = au8_buffer[0] as u16 + (au8_buffer[1] as u16) << 8;
    if cfg!(target_endian = "big")
    {
        return u16_value;
    }
    else
    {
        return u16_value.swap_bytes();
    }
}

pub fn read_u32_from_file(mut file: &std::fs::File) -> u32
{
    let mut u32_value: u32 = 0;
    let mut au8_buffer = [0; 4];

    //
    // We read exactly 32 bits of data in the file.
    // Depending of the architecture of the processor, the data can be in little endian or in big endian.
    // So if the data in the file are in little endian, you must swap the data before to make bitwise operation (TO VERIFY).
    let _result = file.read_exact(&mut au8_buffer);

    let mut u32_valuetmp : u32;

    if cfg!(target_endian = "big")
    {
        //
        // We make the bitwise operator in big endian
        for i in 0..4
        {
            u32_valuetmp = au8_buffer[i] as u32;
            u32_valuetmp >>= 8 * i;
            u32_value += u32_valuetmp;
        }

        return u32_value;
    }
    else
    {
        //
        // We make the bitwise operator in little endian
        for i in 0..4
        {
            u32_valuetmp = au8_buffer[i] as u32;
            u32_valuetmp <<= 8 * i;
            u32_value += u32_valuetmp;
        }

        return u32_value;
    }
}

pub fn read_u64_from_file(mut file: &std::fs::File) -> u64
{
    let mut u64_value: u64 = 0;
    let mut au8_buffer = [0; 8];

    //
    // We read exactly 64 bits of data in the file.
    // Depending of the architecture of the processor, the data can be in little endian or in big endian.
    // So if the data in the file are in little endian, you must swap the data before to make bitwise operation (TO VERIFY).
    let _result = file.read_exact(&mut au8_buffer);

    let mut u64_valuetmp : u64;

    if cfg!(target_endian = "big")
    {
        //
        // We make the bitwise operator in big endian
        for i in 0..8
        {
            u64_valuetmp = au8_buffer[i] as u64;
            u64_valuetmp >>= 8 * i;
            u64_value += u64_valuetmp;
        }

        return u64_value;
    }
    else
    {
        //
        // We make the bitwise operator in little endian
        for i in 0..8
        {
            u64_valuetmp = au8_buffer[i] as u64;
            u64_valuetmp <<= 8 * i;
            u64_value += u64_valuetmp;
        }

        return u64_value;
    }
}

pub fn read_u128_from_file(mut file: &std::fs::File) -> u128
{
    let mut u128_value: u128 = 0;
    let mut au128_buffer = [0; 16];

    //
    // We read exactly 128 bits of data in the file.
    // Depending of the architecture of the processor, the data can be in little endian or in big endian.
    // So if the data in the file are in little endian, you must swap the data before to make bitwise operation (TO VERIFY).
    let _result = file.read_exact(&mut au128_buffer);

    let mut u128_valuetmp : u128;

    if cfg!(target_endian = "big")
    {
        //
        // We make the bitwise operator in big endian
        for i in 0..16
        {
            u128_valuetmp = au128_buffer[i] as u128;
            u128_valuetmp >>= 8 * i;
            u128_value += u128_valuetmp;
        }

        return u128_value;
    }
    else
    {
        //
        // We make the bitwise operator in little endian
        for i in 0..16
        {
            u128_valuetmp = au128_buffer[i] as u128;
            u128_valuetmp <<= 8 * i;
            u128_value += u128_valuetmp;
        }

        return u128_value;
    }
}