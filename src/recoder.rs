#[derive(Clone)]
pub struct Recoder {
    data: Vec<u8>,
    num_pieces_received: usize,
    piece_byte_len: usize,
    num_pieces_coded_together: usize,
}

impl Recoder {
    pub fn new(data: Vec<u8>, piece_byte_len: usize, num_pieces_coded_together: usize) -> Recoder {
        let full_coded_piece_len = num_pieces_coded_together + piece_byte_len;
        let num_pieces_received = data.len() / full_coded_piece_len;

        Recoder {
            data,
            num_pieces_received,
            piece_byte_len,
            num_pieces_coded_together,
        }
    }
}
