use super::consts::BOUNDARY_MARKER;
use crate::{RLNCError, common::gf256::Gf256};
use rand::Rng;

/// Represents an RLNC encoder, responsible for dividing data into pieces and
/// generating coded pieces based on random sampled coding vectors.
#[derive(Clone, Debug)]
pub struct Encoder {
    data: Vec<u8>,
    piece_count: usize,
    piece_byte_len: usize,
}

impl Encoder {
    /// Number of pieces original data got splitted into and being coded together.
    pub fn get_piece_count(&self) -> usize {
        self.piece_count
    }

    /// After padding the original data, it gets splitted into `self.get_piece_count()` many pieces, which results into these many bytes per piece.
    pub fn get_piece_byte_len(&self) -> usize {
        self.piece_byte_len
    }

    /// Each full coded piece consists of `self.get_piece_count()` random coefficients, appended by corresponding encoded piece of `self.get_piece_byte_len()` bytes.
    pub fn get_full_coded_piece_byte_len(&self) -> usize {
        self.get_piece_count() + self.get_piece_byte_len()
    }

    /// Creates a new `Encoder` without adding any padding to the input data.
    /// This is suitable if the input data length is already a multiple of the
    /// desired piece count. This interface is used by Recoder.
    ///
    /// Returns the `Encoder` or `RLNCError::DataLengthMismatch` if the data length is not a
    /// multiple of the piece count.
    pub(crate) fn without_padding(data: Vec<u8>, piece_count: usize) -> Result<Encoder, RLNCError> {
        if data.len() == 0 {
            return Err(RLNCError::DataLengthZero);
        }
        if piece_count == 0 {
            return Err(RLNCError::PieceCountZero);
        }

        let in_data_len = data.len();
        let piece_byte_len = in_data_len / piece_count;
        let computed_total_data_len = piece_byte_len * piece_count;

        if computed_total_data_len != in_data_len {
            return Err(RLNCError::DataLengthMismatch);
        }

        Ok(Encoder {
            data,
            piece_count,
            piece_byte_len,
        })
    }

    /// Creates a new `Encoder` while padding the input data.
    ///
    /// The input data is padded with zeros to ensure its length is a multiple
    /// of `piece_count * piece_byte_len`, where `piece_byte_len` is calculated
    /// such that the original data plus a boundary marker fits within
    /// `piece_count` pieces. A boundary marker (`BOUNDARY_MARKER`) is placed
    /// at the end of the original data before zero padding.
    ///
    /// Returns the `Encoder`.
    pub fn new(mut data: Vec<u8>, piece_count: usize) -> Result<Encoder, RLNCError> {
        if data.len() == 0 {
            return Err(RLNCError::DataLengthZero);
        }
        if piece_count == 0 {
            return Err(RLNCError::PieceCountZero);
        }

        let in_data_len = data.len();
        let boundary_marker_len = 1;
        let piece_byte_len = (in_data_len + boundary_marker_len).div_ceil(piece_count);
        let padded_data_len = piece_count * piece_byte_len;

        data.resize(padded_data_len, 0);
        data[in_data_len] = BOUNDARY_MARKER;

        Ok(Encoder {
            data,
            piece_count,
            piece_byte_len,
        })
    }

    /// Encodes the data held by the encoder using a provided coding vector.
    ///
    /// The resulting coded piece is returned as a `Vec<u8>`, prefixed by the
    /// coding vector itself (as `u8` values). The total length of the returned
    /// vector is `self.get_complete_coded_piece_byte_len()`.
    ///
    /// Returns `RLNCError::CodingVectorLengthMismatch` if the length of the
    /// provided `coding_vector` does not match `self.piece_count`.
    pub fn code_with_coding_vector(&self, coding_vector: &[Gf256]) -> Result<Vec<u8>, RLNCError> {
        if coding_vector.len() != self.piece_count {
            return Err(RLNCError::CodingVectorLengthMismatch);
        }

        let coded_piece = self
            .data
            .chunks_exact(self.piece_byte_len)
            .zip(coding_vector)
            .map(|(piece, &random_symbol)| piece.iter().map(|&symbol| Gf256::new(symbol) * random_symbol).collect::<Vec<Gf256>>())
            .fold(vec![Gf256::default(); self.piece_byte_len], |mut acc, cur| {
                acc.iter_mut().zip(cur).for_each(|(a, b)| {
                    *a += b;
                });

                acc
            })
            .iter()
            .map(|symbol| symbol.get())
            .collect::<Vec<u8>>();

        let mut full_coded_piece = vec![0u8; self.get_full_coded_piece_byte_len()];

        full_coded_piece[..self.piece_count].iter_mut().enumerate().for_each(|(idx, symbol)| {
            *symbol = coding_vector[idx].get();
        });
        full_coded_piece[self.piece_count..].copy_from_slice(&coded_piece);

        Ok(full_coded_piece)
    }

    /// Encodes the data held by the encoder using a randomly sampled coding vector.
    ///
    /// A coding vector of `self.piece_count` random `Gf256` symbols is generated
    /// using the provided random number generator.
    ///
    /// Calls `code_with_coding_vector` internally.
    ///
    /// Returns the coded piece prefixed by the random coding vector.
    pub fn code<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Vec<u8>, RLNCError> {
        let random_coding_vector = (0..self.piece_count).map(|_| rng.random()).collect::<Vec<Gf256>>();

        self.code_with_coding_vector(&random_coding_vector)
    }
}
