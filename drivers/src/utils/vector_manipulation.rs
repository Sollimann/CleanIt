use crate::utils::checksum::Checksum;
use std::alloc::Global;

pub fn extract_sublist(
    byte_data: &mut Vec<u8, Global>,
    seq: [u8; 2],
    slice_size: usize,
    checksum: &mut Checksum,
) -> bool {
    println!("slice size: {}", slice_size);
    let first_index = byte_data.iter().position(|&r| r == seq[0]);
    match first_index {
        None => return false,
        Some(index) => {
            let last_index = byte_data.len() - 1;
            if (index == byte_data.len() - 1) || (index + slice_size >= last_index) {
                return false;
            }

            if byte_data[index + 1] == seq[1] {
                // remove every element from index: 0 -> first_index - 1
                for _ in 0..index {
                    byte_data.remove(0);
                }

                // remove every element from after: first_index + slice_size
                byte_data.truncate(slice_size);

                // double-check that size is right
                assert_eq!(byte_data.len(), slice_size);

                checksum.push_slice(&byte_data);
                let checksum_low_byte = checksum.calculate_low_byte_sum();
                checksum.reset();
                return checksum_low_byte == 0;
            }
        }
    }
    false
}
