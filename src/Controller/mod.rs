#[derive(PartialEq)]
pub enum QuAvailableTypeInEvent
{
    string,
    float,
    uint8,
    uint32,
    uint64,
    int8,
    int32,
    int64,
}

pub trait QuInformationData
{
    fn convert_to_key_map(&self) -> Vec<(String, QuAvailableTypeInEvent, String)>;
}

pub(crate) mod EventManager;