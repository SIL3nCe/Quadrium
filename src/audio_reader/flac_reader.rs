use std::fs::File;
use crate::utils;
use std::io::{Seek, SeekFrom};
use crate::audio_reader::{AudioInformation, AudioReader};

struct StreamBlockInfo
{
    m_minBlockSize: u16,
    m_maxBlockSize: u16,
    m_minFrameSize: u32,
    m_maxFrameSize: u32,
    m_rate: u32,
    m_channelCount: u8,
    m_bitsPerSample: u8,
    m_totalSample: u64
}

pub struct FlacReader
{
}

pub fn is_flac_file(file: & File) -> bool
{
    let value = utils::file_reader::read_u32_from_file(file);
    let magic_number = 0x66 as u32 | (0x4C as u32) << 8 | (0x61 as u32) << 16 | (0x43 as u32) << 24;
    magic_number == value
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
    let total_samples: u64 = ((multi_info & 0xFFFFFFFFF) as u64);
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
    let rate: u32 = ((multi_info & 0xFFFFF) as u32);
    multi_info >>= 20;

    //
    // Get the max frame size on the 24 bits
    let max_frame_size: u32 = ((multi_info & 0xFFFFFF) as u32);
    multi_info >>= 24;

    //
    // Get the min frame size on the 24 bits
    let min_frame_size: u32 = ((multi_info & 0xFFFFFF) as u32);
    multi_info >>= 24;

    //
    // Get the max block size on the first 16 bits
    let max_block_size: u16 = ((multi_info & 0xFFFF) as u16);
    multi_info >>= 16;

    assert!(multi_info == 0);

    //
    // Get the md5
    let md5: u128 = utils::file_reader::read_u128_from_file(file).swap_bytes();

    //
    // Return the information
    let streamBlock = StreamBlockInfo{
        m_bitsPerSample : bits_per_sample,
        m_rate : rate,
        m_minBlockSize : min_block_size,
        m_maxBlockSize : max_block_size,
        m_totalSample : total_samples,
        m_minFrameSize : min_frame_size,
        m_maxFrameSize : max_frame_size,
        m_channelCount : channel_count
    };

    return streamBlock;
}

impl AudioReader for FlacReader
{
    fn read_information(&self, str_path_to_music : String) -> AudioInformation
    {
        //
        // Init the audio reader
        let mut audioReader : AudioInformation = AudioInformation {
            m_str_music_type : std::string::String::from(""),
            m_str_music_name : std::string::String::from(""),
            m_str_duration : std::string::String::from(""),
            m_str_artist_name : std::string::String::from(""),
            m_rate : 0,
            m_channelCount : 0,
            m_bitsPerSample : 0
        };

        //
        // Open the file
        let mut file = match std::fs::File::open(str_path_to_music)
        {
            Err(why) => panic!("Could not open the file !"),
            Ok(file) => file,
        };

        //
        // Test if the file is a flac
        let b_is_flac_file = is_flac_file(&file);
        if b_is_flac_file
        {

            let seek_position = file.seek(SeekFrom::Current(0)).unwrap();

            //
            // Read Metadata block
            let metadata_header = utils::file_reader::read_u32_from_file(&file);
            let b_last_block = metadata_header & 1;
            let block_type = (metadata_header & 0xe).swap_bytes(); // Get block type
            let block_size = metadata_header.swap_bytes() & 0xFFFFF;

            let current_1_seek_position = file.seek(SeekFrom::Current(0)).unwrap();

            if block_type == 0
            {
                let streamBlock = read_streaminfo_block(&file);

                audioReader.m_rate = streamBlock.m_rate;
                audioReader.m_bitsPerSample = streamBlock.m_bitsPerSample;
                audioReader.m_channelCount = streamBlock.m_channelCount;
            }
        }

        return audioReader;
    }
}