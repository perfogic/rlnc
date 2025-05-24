use crate::{encoder::Encoder, errors::RLNCError};

#[derive(Clone)]
pub struct Recoder {
    coding_vectors: Vec<u8>,
    encoder: Encoder,
    num_pieces_received: usize,
    piece_byte_len: usize,
    num_pieces_coded_together: usize,
}

impl Recoder {
    pub fn new(
        data: Vec<u8>,
        piece_byte_len: usize,
        num_pieces_coded_together: usize,
    ) -> Result<Recoder, RLNCError> {
        let full_coded_piece_len = num_pieces_coded_together + piece_byte_len;
        let num_pieces_received = data.len() / full_coded_piece_len;

        if num_pieces_received == 0 {
            return Err(RLNCError::NotEnoughPiecesToRecode);
        }

        let mut coding_vectors =
            Vec::with_capacity(num_pieces_received * num_pieces_coded_together);
        let mut coded_pieces = Vec::with_capacity(num_pieces_received * piece_byte_len);

        data.chunks_exact(full_coded_piece_len)
            .for_each(|full_coded_piece| {
                let coding_vector = &full_coded_piece[..num_pieces_coded_together];
                let coded_piece = &full_coded_piece[num_pieces_coded_together..];

                coding_vectors.extend_from_slice(coding_vector);
                coded_pieces.extend_from_slice(coded_piece);
            });

        let encoder = Encoder::new(coded_pieces, num_pieces_coded_together);
        Ok(Recoder {
            coding_vectors,
            encoder,
            num_pieces_received,
            piece_byte_len,
            num_pieces_coded_together,
        })
    }
}
