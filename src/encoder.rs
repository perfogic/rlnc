use crate::gf256::Gf256;
use rand::Rng;

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

    pub fn code<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<u8> {
        let random_coding_vector = (0..self.piece_count)
            .map(|_| rng.random())
            .collect::<Vec<Gf256>>();

        let piece_byte_len = self.data.len() / self.piece_count;

        let coded_piece = self
            .data
            .chunks_exact(piece_byte_len)
            .zip(&random_coding_vector)
            .map(|(piece, &random_symbol)| {
                piece
                    .iter()
                    .map(|&symbol| Gf256::new(symbol) * random_symbol)
                    .collect::<Vec<Gf256>>()
            })
            .fold(vec![Gf256::default(); piece_byte_len], |mut acc, cur| {
                acc.iter_mut().zip(cur).for_each(|(a, b)| {
                    *a = *a + b;
                });

                acc
            })
            .iter()
            .map(|symbol| symbol.get())
            .collect::<Vec<u8>>();

        let mut full_coded_piece = vec![0u8; self.piece_count + piece_byte_len];

        full_coded_piece[..self.piece_count]
            .iter_mut()
            .enumerate()
            .for_each(|(idx, symbol)| {
                *symbol = random_coding_vector[idx].get();
            });

        full_coded_piece[self.piece_count..].copy_from_slice(&coded_piece);

        full_coded_piece
    }
}
