use super::*;

use serde_json;

pub fn serde_unmarshal(data: &[u8]) -> Twitter {
    let v: Twitter = serde_json::from_slice(data).unwrap();
    v
}

pub fn serde_marshal(v: &Twitter) -> Vec<u8> {
    let mut writer = Vec::with_capacity(9000);
    serde_json::to_writer(&mut writer, v).unwrap();
    writer
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_serde_unmarshal() {
        let mut file = File::open("../../testdata/twitter.json").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let v = serde_unmarshal(&data);
        assert_eq!(v.statuses.len(), 4);
    }

    #[test]
    fn test_serde_marshal() {
        let mut file = File::open("../../testdata/twitter.json").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let v = serde_unmarshal(&data);
        assert_eq!(v.statuses.len(), 4);

        let data = serde_marshal(&v);
        assert_eq!(data.len(), 8914);
    }
}
