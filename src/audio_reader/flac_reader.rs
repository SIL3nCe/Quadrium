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

//
// Based on https://xiph.org/flac/format.html

use std::fs::File;
use std::mem::size_of;
use crate::utils;
use crate::audio_reader::{AudioInformation, AudioReader};
use crate::utils::file_reader::{read_u16_from_file, read_u32_from_file, read_u64_from_file, read_u8_from_file};

/******************************************************
 * Declaration of the different structures needed
 * to extract informations of the flac files
 ******************************************************/
struct MetaDataHeader
{
    m_is_last: bool,
    m_block_type: u8,
    m_length: u32,
}

struct StreamBlockInfo
{
    m_min_block_size: u16,
    m_max_block_size: u16,
    m_min_frame_size: u32,
    m_max_frame_size: u32,
    m_rate: u32,
    m_channel_count: u8,
    m_bits_per_sample: u8,
    m_total_sample: u64
}

struct ApplicationBlock
{
    m_application_id: u32,
    m_application_data: Vec<u8>
}

struct VorbisCommentBlock
{
    m_vendor_string: String,
    m_user_comment_list: Vec<String>,
}

struct SeekPoint
{
    m_sample_number_first_sample: u64,
    m_sample_offset: u64,
    m_number_samples: u16
}

struct SeekTableBlock
{
    m_seekPoints : Vec<SeekPoint>
}

struct CuesheetTrackIndexBlock
{
    m_offset_samples: u64,
    m_index_point: u8
}

struct CuesheetTrackBlock
{
    m_track_offset_samples: u64,
    m_track_number: u8,
    m_track_isrc: [u8; 12],
    m_is_audio: bool,
    m_pre_emphasis: bool,
    m_track_index_points_number: u8,
    m_cuesheet_track_indices: Vec<CuesheetTrackIndex>
}

struct CuesheetBlock
{
    m_media_catalog_number: [u8; 128],
    m_lead_in_number: u64,
    m_is_cd: bool,
    m_track_number: u8,
    m_cuesheet_tracks: Vec<CuesheetTrackBlock>
}

struct FrameHeader
{

}

struct FrameFooter
{

}

pub struct FlacFile
{
    m_streaminfo: StreamBlockInfo,
    m_application_infos: Vec<ApplicationBlock>,
    m_vorbis_comment: Vec<VorbisCommentBlock>,
    m_seektables: Vec<SeekTableBlock>,
    m_cuesheets: Vec<CuesheetBlock>
}

pub struct FlacReader
{
}

/*****************************************************
 * Functions not exposed needed to decodes flac files
 *****************************************************/

fn read_metadata_header(file: &File) -> MetaDataHeader
{
    //
    // Read Metadata block
    let mut metadata_header = utils::file_reader::read_u32_from_file(&file).swap_bytes();

    let block_size = metadata_header & 0xFFFFF;
    metadata_header >>= 24;

   let block_type = metadata_header & 0x7F; // Get block type
    metadata_header >>= 7;

    let b_last_block = metadata_header & 1;

    return MetaDataHeader{
        m_is_last: b_last_block == 1,
        m_block_type: block_type as u8,
        m_length: block_size,
    };
}

fn read_streaminfo_block(file: &File) -> StreamBlockInfo
{
    //
    // Get the min block size of the file
    let min_block_size: u16 = utils::file_reader::read_u16_from_file(file).swap_bytes();

    //
    // Get all information on 128 bits
    let mut multi_info: u128 = utils::file_reader::read_u128_from_file(file).swap_bytes();

    //
    // Get the total samples inside the files (36bits)
    let total_samples: u64 = (multi_info & 0xFFFFFFFFF) as u64;
    multi_info >>= 36;

    //
    // Get the number of bits per sample (5bits)
    let bits_per_sample: u8 = (((multi_info & 0x1F) as u8)) + 1;
    multi_info >>= 5;

    //
    // Get channel count on the next 3bits
    let channel_count: u8 = (((multi_info & 0x7) as u8))  + 1;
    multi_info >>= 3;

    // Get the sample rate on the next 20 bits
    let rate: u32 = (multi_info & 0xFFFFF) as u32;
    multi_info >>= 20;

    //
    // Get the max frame size on the 24 bits
    let max_frame_size: u32 = (multi_info & 0xFFFFFF) as u32;
    multi_info >>= 24;

    //
    // Get the min frame size on the 24 bits
    let min_frame_size: u32 = (multi_info & 0xFFFFFF) as u32;
    multi_info >>= 24;

    //
    // Get the max block size on the first 16 bits
    let max_block_size: u16 = (multi_info & 0xFFFF) as u16;
    multi_info >>= 16;

    assert!(multi_info == 0);

    //
    // Get the md5
    let _md5: u128 = utils::file_reader::read_u128_from_file(file).swap_bytes();

    //
    // Return the information
    let stream_block = StreamBlockInfo{
        m_bits_per_sample: bits_per_sample,
        m_rate : rate,
        m_min_block_size: min_block_size,
        m_max_block_size: max_block_size,
        m_total_sample: total_samples,
        m_min_frame_size: min_frame_size,
        m_max_frame_size: max_frame_size,
        m_channel_count: channel_count
    };

    return stream_block;
}

fn read_block_padding(file: &File, number_padding: u32)
{
    for _i in 0..number_padding
    {
        read_u8_from_file(file);
    }
}

fn read_seek_table(file: &File, size_block: u32) -> SeekTableBlock
{
    //
    // Get the number of seek point
    // Divide by 18 because it is the size in Bytes of a Seekpoint
    let seek_point_count: u32 = size_block / 18;
    let mut seekpoints = Vec::with_capacity(seek_point_count as usize);
    for _i in 0..seek_point_count
    {
        let sample_number = read_u64_from_file(file).swap_bytes();
        let sample_offset = read_u64_from_file(file).swap_bytes();
        let number_samples = read_u16_from_file(file).swap_bytes();
        seekpoints.push(SeekPoint
        {
            m_number_samples: number_samples,
            m_sample_offset: sample_offset,
            m_sample_number_first_sample: sample_number
        });
    }
    return SeekTableBlock
    {
        m_seekPoints: seekpoints
    };
}

fn read_block_application(file: &File, size_block: u32) -> ApplicationBlock
{
    //
    // Get the application id
    let application_id = read_u32_from_file(file);

    //
    // Get the data of the block coming from the application
    // Depending the application id, informations must be decoded in different ways
    let data_count = (size_block - 32)/8;
    let mut data: Vec<u8> = Vec::new();
    for _i in 0..data_count -1
    {
        data.push(read_u8_from_file(file));
    }

    return ApplicationBlock{
        m_application_id: application_id,
        m_application_data: data,
    };
}

fn read_vorbis_comment_block(file: &File, size_block: u32) -> VorbisCommentBlock
{
    //
    // Get the vendor which realize the files
    let vendor_length = read_u32_from_file(file);
    let mut vendor_string: String = String::new();
    for _i in 0..vendor_length
    {
        vendor_string.push(read_u8_from_file(file) as char);
    }

    //
    // Get the user comment list
    // It contains tag of the album, the artist...
    // Based on https://www.xiph.org/vorbis/doc/v-comment.html
    let user_comment_list_length: u32 = read_u32_from_file(file);
    let mut list_comment: Vec<String> = Vec::new();
    for _i in 0..user_comment_list_length
    {
        let comment_size = read_u32_from_file(file);
        let mut comment: String = String::new();
        for _j in 0..comment_size
        {
            comment.push(read_u8_from_file(file) as char);
        }

        list_comment.push(comment);
    }
    return VorbisCommentBlock
    {
        m_vendor_string: vendor_string,
        m_user_comment_list: list_comment,
    };
}

fn read_cuesheet_track_index_block(file: &File) -> CuesheetTrackIndexBlock
{
    let offset_samples = read_u64_from_file(file).swap_bytes();
    let index_point_number = read_u8_from_file(file).swap_bytes();

    //
    // Read empty data for padding
    for _i in 0..3
    {
        read_u8_from_file(file);
    }

    return CuesheetTrackIndexBlock
    {
        m_offset_samples: offset_samples,
        m_index_point: index_point_number,
    };
}

fn read_cuesheet_track_block(file: &File) -> CuesheetTrackBlock
{
    let track_offset_in_samples = read_u64_from_file(file).swap_bytes();
    let track_number = read_u8_from_file(file).swap_bytes();

    //
    // Read track ISRC
    let mut read_track_isrc_array: [u8; 12] = [0; 12];
    for iTrackISRCIndex in 0..12
    {
        read_track_isrc_array[iTrackISRCIndex] = read_u8_from_file(file);
    }

    //
    // Read the different flags
    let tmp_flags_track_type_pre_emphasis = read_u8_from_file(file).swap_bytes();
    let is_audio: bool = tmp_flags_track_type_pre_emphasis & 0x1 == 1;
    let is_pre_emphasis: bool = tmp_flags_track_type_pre_emphasis & 0x2 == 1;

    //
    // Seek to the next fields as required by specification
    for _iEmptyDataIndex in 0..13
    {
        read_u8_from_file(file);
    }

    let track_index_point_number = read_u8_from_file(file).swap_bytes();
    let mut cuesheet_track_indexes: Vec<CuesheetTrackIndexBlock> = Vec::new();
    for _i in 0..track_index_point_number
    {
        cuesheet_track_indexes.push(read_cuesheet_track_index_block(file));
    }

    return CuesheetTrackBlock
    {
        m_track_offset_samples: track_offset_in_samples,
        m_track_number: track_number,
        m_track_isrc: read_track_isrc_array,
        m_is_audio: is_audio,
        m_pre_emphasis: is_pre_emphasis,
        m_track_index_points_number: track_index_point_number,
        m_cuesheet_track_indices: cuesheet_track_indexes,
    };
}

fn read_cuesheet_block(file: &File, size_block: u32) -> CuesheetBlock
{
    //
    // Read media catalog number
    let mut media_catalog_number: [u8; 128] = [0; 128];
    for iMediaCatalogNumber in 0..128
    {
        let current_number = read_u8_from_file(file);
        media_catalog_number[iMediaCatalogNumber] = current_number;
    }

    //
    // Read the number of lead in
    let number_lead_in = read_u64_from_file(file).swap_bytes();

    //
    // Read the flag if it is a CD
    let is_cd_in_u8 = read_u8_from_file(file).swap_bytes();
    let is_cd = is_cd_in_u8 & 0x1 == 1;

    //
    // Move seek to the number of tracks
    for _iEmptyDataIndex in 0..258
    {
        read_u8_from_file(file);
    }

    //
    // Read the number of track
    let number_tracks = read_u8_from_file(file).swap_bytes();

    //
    // Read the cuesheet tracks
    let mut cuesheet_tracks: Vec<CuesheetTrackBlock> = Vec::new();
    for _i in 0..number_tracks
    {
        cuesheet_tracks.push(read_cuesheet_track_block(file));
    }

    return CuesheetBlock
    {
        m_media_catalog_number: media_catalog_number,
        m_lead_in_number: number_lead_in,
        m_is_cd: is_cd,
        m_track_number: number_tracks,
        m_cuesheet_tracks: cuesheet_tracks,
    }
}

fn read_frame_header(file: &File, stream_info: StreamBlockInfo)
{
    //
    // Get the first 8-bit of the frame header to get the block strategy and sync code
    let mut first_value = read_u32_from_file(file).swap_bytes();
    let block_strategy = first_value & 0x1;
    let reserved_bit = first_value & 0x2;
    first_value >>= 2;

    let sync_code = first_value & 0x3F;
    if sync_code != 0x2F
    {
        panic!("Error when reading the flac files");
    }

    //
    // Get the sample rate and the channel assignment of the frame
    let rate_and_channel = read_u8_from_file(file).swap_bytes();
    let channel_assignment = rate_and_channel & 0xF;
    let sample_rate = rate_and_channel & 0xF0;


}

/**************************************************
 * Public functions for flac files
 **************************************************/

pub fn is_flac_file(file: & File) -> bool
{
    let value = utils::file_reader::read_u32_from_file(file);
    let magic_number = 0x66 as u32 | (0x4C as u32) << 8 | (0x61 as u32) << 16 | (0x43 as u32) << 24;
    magic_number == value
}

impl AudioReader for FlacReader
{
    fn read_information(&self, str_path_to_music : String) -> AudioInformation
    {
        //
        // Init the audio reader
        let mut audio_reader: AudioInformation = AudioInformation {
            m_str_music_type : std::string::String::from(""),
            m_str_music_name : std::string::String::from(""),
            m_str_duration : std::string::String::from(""),
            m_str_artist_name : std::string::String::from(""),
            m_str_tracknumber : std::string::String::from(""),
            m_str_album : std::string::String::from(""),
            m_str_date: std::string::String::from(""),
            m_rate : 0,
            m_channel_count: 0,
            m_bits_per_sample: 0
        };

        //
        // Open the file
        let file = match std::fs::File::open(str_path_to_music)
        {
            Err(why) => panic!("Could not open the file !"),
            Ok(file) => file,
        };

        //
        // Test if the file is a flac
        let b_is_flac_file = is_flac_file(&file);
        if b_is_flac_file
        {
            //
            // Read the StreamInfoBlock
            let mut header_stream_info = read_metadata_header(&file);
            if header_stream_info.m_block_type == 0
            {
                let stream_block = read_streaminfo_block(&file);

                audio_reader.m_rate = stream_block.m_rate;
                audio_reader.m_bits_per_sample = stream_block.m_bits_per_sample;
                audio_reader.m_channel_count = stream_block.m_channel_count;
            }

            //
            // Read the others metadatablock
            while !header_stream_info.m_is_last
            {
                header_stream_info = read_metadata_header(&file);
                if header_stream_info.m_block_type == 1
                {
                    println!("Padding of {0}", header_stream_info.m_length / 8);
                    read_block_padding(&file, header_stream_info.m_length / 8);
                }
                else if header_stream_info.m_block_type == 2
                {
                    println!("ApplicationData");
                    let application_data = read_block_application(&file, header_stream_info.m_length);
                }
                else if header_stream_info.m_block_type == 3
                {
                    println!("Seektable");
                    let _seek_table_data = read_seek_table(&file, header_stream_info.m_length);
                }
                else if header_stream_info.m_block_type == 4
                {
                    let vorbis_comment = read_vorbis_comment_block(&file, header_stream_info.m_length);
                    for comment in vorbis_comment.m_user_comment_list
                    {
                        //
                        // TODO: Fix this bad implementation
                        if comment.contains("ARTIST")
                        {
                            let artist_index = "ARTIST=".len();
                            audio_reader.m_str_artist_name = comment[artist_index..].to_string();
                        }
                        else if comment.contains("TITLE")
                        {
                            let title_index = "TITLE=".len();
                            audio_reader.m_str_music_name = comment[title_index..].to_string();
                        }
                        else if comment.contains("TRACKNUMBER")
                        {
                            let tracknumber_index = "TRACKNUMBER=".len();
                            audio_reader.m_str_tracknumber = comment[tracknumber_index..].to_string();
                        }
                        else if comment.contains("DATE")
                        {
                            let date_index = "DATE=".len();
                            audio_reader.m_str_date = comment[date_index..].to_string();
                        }
                        else if comment.contains("YEAR")
                        {
                            //
                            // Not standard but can be used in flac files so...
                            let date_index = "YEAR=".len();
                            audio_reader.m_str_date = comment[date_index..].to_string();
                        }
                        else if comment.contains("ALBUM")
                        {
                            let album_index = "ALBUM=".len();
                            audio_reader.m_str_album = comment[album_index..].to_string();
                        }
                        else if comment.contains("GENRE")
                        {
                            let genre_index = "GENRE=".len();
                            audio_reader.m_str_music_type = comment[genre_index..].to_string();
                        }
                    }
                }
                else if header_stream_info.m_block_type == 5
                {
                    println!("Cuesheet");
                    let cuesheet_block = read_cuesheet_block(&file, header_stream_info.m_length);
                }
                else if header_stream_info.m_block_type == 6
                {
                    println!("Picture");
                }
            }
        }

        return audio_reader;
    }
}