// use super::*;

// use simd_json;

// pub fn simd_json_unmarshal(data: &mut [u8]) -> Twitter {
//     if data.len() == 0 {
//         return Twitter{statuses: Vec::new(), search_metadata: SearchMetadata{completed_in: 0.0, max_id: 0, max_id_str: String::new(), next_results: String::new(), query: String::new(), refresh_url: String::new(), count: 0, since_id: 0, since_id_str: String::new()}};
//     }
    
//     let v = simd_json::serde::from_slice(data).unwrap();
//     v
// }

// pub fn simd_json_marshal(v: &Twitter) -> Vec<u8> {
//     simd_json::serde::to_vec(v).unwrap()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use std::fs::File;
//     use std::io::Read;

//     #[test]
//     fn test_simd_json_unmarshal() {
//         let mut file = File::open("../../testdata/twitter.json").unwrap();
//         let mut data = Vec::new();
//         file.read_to_end(&mut data).unwrap();

//         let v = simd_json_unmarshal(&mut data);
//         assert_eq!(v.statuses.len(), 4);
//     }

//     #[test]
//     fn test_simd_json_marshal() {
//         let mut file = File::open("../../testdata/twitter.json").unwrap();
//         let mut data = Vec::new();
//         file.read_to_end(&mut data).unwrap();

//         let mut data0 = data.clone();

//         let v = simd_json_unmarshal(&mut data0);
//         assert_eq!(v.statuses.len(), 4);


//         let data = simd_json_marshal(&v);
//         assert_eq!(data.len(), 8914);
//     }
// }
