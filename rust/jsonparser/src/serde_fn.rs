use super::*;

use std::fs::File;
use std::io::Read;

pub fn serde_unmarshal(data: &[u8]) -> Twitter{
    let v: Twitter = serde_json::from_slice(data).unwrap();
    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_unmarshal() {
        let mut file = File::open("../../testdata/twitter.json").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let v = serde_unmarshal(&data);
        assert_eq!(v.statuses.len(), 4);
    }
}