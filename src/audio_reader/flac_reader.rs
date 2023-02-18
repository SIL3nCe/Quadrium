use std::fs::File;
use crate::utils;
use std::io::{Seek, SeekFrom};
use crate::audio_reader::{AudioInformation, AudioReader};
use crate::utils::file_reader::{read_u32_from_file, read_u8_from_file};

struct MetaDataHeader
{
    m_isLast : bool,
    m_blockType : u8,
    m_length: u32,
}

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

struct ApplicationBlock
{
    m_applicationID: u32,
    m_applicationData: Vec<u8>
}

struct VorbisCommentBlock
{
    m_vendorString: String,
    m_userCommentList: Vec<String>,
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
        m_isLast: b_last_block == 1,
        m_blockType: block_type as u8,
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

fn read_block_padding(file: &File, number_padding: u32)
{
    for i in 0..number_padding - 1
    {
        read_u8_from_file(file);
    }
}

fn read_block_application(file: &File, size_block: u32) -> ApplicationBlock
{
    let applicationID = read_u32_from_file(file);
    let dataCount = (size_block - 32)/8;
    let mut data: Vec<u8> = Vec::new();
    for i in 0..dataCount-1
    {
        data.push(read_u8_from_file(file));
    }

    return ApplicationBlock{
        m_applicationID : applicationID,
        m_applicationData: data,
    };
}

fn read_vorbis_comment_block(file: &File, size_block: u32) -> VorbisCommentBlock
{
    let vendorLength = read_u32_from_file(file);
    let mut vendorString: String = String::new();
    for i in 0..vendorLength
    {
        vendorString.push(read_u8_from_file(file) as char);
    }

    let userCommentListLength: u32 = read_u32_from_file(file);
    let mut listComment : Vec<String> = Vec::new();
    for i in 0..userCommentListLength
    {
        let commentSize = read_u32_from_file(file);
        let mut comment: String = String::new();
        for j in 0..commentSize
        {
            comment.push(read_u8_from_file(file) as char);
        }

        listComment.push(comment);
    }
    return VorbisCommentBlock
    {
        m_vendorString: vendorString,
        m_userCommentList: listComment,
    };
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
            m_str_tracknumber : std::string::String::from(""),
            m_str_album : std::string::String::from(""),
            m_str_date: std::string::String::from(""),
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
            //
            // Read the StreamInfoBlock
            let mut headerStreamInfo = read_metadata_header(&file);
            if headerStreamInfo.m_blockType == 0
            {
                let streamBlock = read_streaminfo_block(&file);

                audioReader.m_rate = streamBlock.m_rate;
                audioReader.m_bitsPerSample = streamBlock.m_bitsPerSample;
                audioReader.m_channelCount = streamBlock.m_channelCount;
            }

            //
            // Read the others metadatablock
            while(!headerStreamInfo.m_isLast)
            {
                headerStreamInfo = read_metadata_header(&file);
                if (headerStreamInfo.m_blockType == 1)
                {
                    println!("Padding of {0}", headerStreamInfo.m_length / 8);
                    read_block_padding(&file, headerStreamInfo.m_length / 8);
                }
                else if (headerStreamInfo.m_blockType == 2)
                {
                    println!("ApplicationData");
                    let applicationData = read_block_application(&file, headerStreamInfo.m_length);
                }
                else if (headerStreamInfo.m_blockType == 3)
                {
                    println!("Seektable");
                }
                else if (headerStreamInfo.m_blockType == 4)
                {
                    let vorbisComment = read_vorbis_comment_block(&file, headerStreamInfo.m_length);
                    for comment in vorbisComment.m_userCommentList
                    {
                        //
                        // TODO: Fix this bad implementation
                        if (comment.contains("ARTIST"))
                        {
                            let artistIndex = "ARTIST=".len();
                            audioReader.m_str_artist_name = comment[artistIndex..].to_string();
                        }
                        else if (comment.contains("TITLE"))
                        {
                            let titleIndex = "TITLE=".len();
                            audioReader.m_str_music_name = comment[titleIndex..].to_string();
                        }
                        else if (comment.contains("TRACKNUMBER"))
                        {
                            let tracknumberIndex = "TRACKNUMBER=".len();
                            audioReader.m_str_tracknumber = comment[tracknumberIndex..].to_string();
                        }
                        else if (comment.contains("DATE"))
                        {
                            let dateIndex = "DATE=".len();
                            audioReader.m_str_date = comment[dateIndex..].to_string();
                        }
                        else if (comment.contains("ALBUM"))
                        {
                            let albumIndex = "ALBUM=".len();
                            audioReader.m_str_album = comment[albumIndex..].to_string();
                        }
                    }
                }
                else if (headerStreamInfo.m_blockType == 5)
                {
                    println!("Cuesheet");
                }
                else if (headerStreamInfo.m_blockType == 6)
                {
                    println!("Picture");
                }
            }
        }

        return audioReader;
    }
}