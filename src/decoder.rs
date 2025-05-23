#[derive(Clone)]
pub struct Decoder {
    data: Vec<u8>,
    piece_byte_len: usize,
    required_piece_count: usize,
    received_piece_count: usize,
}

impl Decoder {
    pub fn new(piece_byte_len: usize, required_piece_count: usize) -> Decoder {
        let total_byte_len = required_piece_count * piece_byte_len;
        let data = Vec::with_capacity(total_byte_len);

        Decoder {
            data,
            piece_byte_len,
            required_piece_count,
            received_piece_count: 0,
        }
    }
}
