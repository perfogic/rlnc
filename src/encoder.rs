use crate::gf256::Gf256;

pub const BOUNDARY_MARKER: u8 = 0x81;

#[derive(Clone)]
pub struct Encoder {
    data: Vec<u8>,
    piece_count: usize,
}

impl Encoder {
    pub fn new(mut data: Vec<u8>, piece_count: usize) -> Encoder {
        let in_data_len = data.len();
        let piece_byte_len = (in_data_len + (piece_count - 1)) / piece_count;
        let padded_data_len = piece_count * piece_byte_len;

        data.resize(padded_data_len, 0);

        if padded_data_len > in_data_len {
            data[in_data_len] = BOUNDARY_MARKER;
        }

        Encoder { data, piece_count }
    }

    pub fn code() -> Vec<u8> {
        todo!()
    }
}
