pub mod flac_reader;

/// \struct MusicInformation
/// Structure that define all the information that define this music
pub struct AudioInformation
{
    pub m_str_music_name : String,
    pub m_str_music_type : String,
    pub m_str_artist_name : String,
    pub m_str_tracknumber: String,
    pub m_str_album : String,
    pub m_str_date: String,
    pub m_str_duration: String,

    pub m_rate: u32,
    pub m_channelCount: u8,
    pub m_bitsPerSample: u8,
}

/// \interface MusicReader
/// \brief Interface to create reader of music file
/// Quadrium can read different audio files such as WAV, Flac... This interface defines the way to create the reader of these files.
/// This is a private interface. The user will only access to the MusicReaderManager.
pub trait AudioReader
{
    /// \brief Read information about the audio files
    fn read_information(&self, str_path_to_music : String) -> AudioInformation;
}