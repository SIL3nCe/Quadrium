pub mod file_reader;

use std::io::Read;

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