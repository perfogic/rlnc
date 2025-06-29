use super::encoder::Encoder;
use crate::{RLNCError, common::gf256::Gf256};
use rand::Rng;

/// `Recoder` takes already coded pieces and recodes these coded pieces using
/// a new random sampled coding vector. This is useful for distributing coded
/// pieces more widely without needing to decode back to original data.
///
/// A `Recoder` essentially acts as a new encoder, but it operates on the *encoded* source pieces.
///
/// The `Recoder` stores the coding vectors and coded pieces from the input.
/// Internally, it uses an `Encoder` initialized with the coded pieces.
#[derive(Clone, Debug)]
pub struct Recoder {
    coding_vectors: Vec<Gf256>,
    encoder: Encoder,
    num_pieces_received: usize,
    full_coded_piece_byte_len: usize,
    num_pieces_coded_together: usize,
}

impl Recoder {
    /// Number of pieces original data got splitted into to be coded together.
    pub fn get_original_num_pieces_coded_together(&self) -> usize {
        self.num_pieces_coded_together
    }

    /// Number of pieces received by Recoder, which is getting recoded together, producing new pieces.
    pub fn get_num_pieces_recoded_together(&self) -> usize {
        self.num_pieces_received
    }

    /// After padding the original data, it gets splitted into `self.get_original_num_pieces_coded_together()` many pieces, which results into these many bytes per piece.
    pub fn get_piece_byte_len(&self) -> usize {
        self.full_coded_piece_byte_len - self.num_pieces_coded_together
    }

    /// Each full coded piece consists of `self.get_original_num_pieces_coded_together()` random coefficients, appended by corresponding encoded piece of `self.get_piece_byte_len()` bytes.
    pub fn get_full_coded_piece_byte_len(&self) -> usize {
        self.full_coded_piece_byte_len
    }

    /// Creates a new `Recoder` instance from a vector of received coded pieces.
    ///
    /// Each full coded piece in `data` is of `full_coded_piece_byte_len` bytes.
    /// A full coded piece = coding vector ++ coded piece
    ///
    /// The `Recoder` extracts the coding vectors and coded pieces from the input
    /// data. It then initializes an internal `Encoder` that implicitly
    /// represents the source pieces extracted from the input.
    ///
    /// # Arguments
    ///
    /// * `data`: A vector of bytes containing the concatenated full coded pieces, each of
    ///   `full_coded_piece_byte_len` bytes length.
    /// * `full_coded_piece_byte_len`: The byte length of a full coded piece.
    /// * `num_pieces_coded_together`: The number of original pieces that were
    ///   linearly combined to create each coded piece. This is also the length
    ///   of the coding vector prepended to each full coded piece.
    ///
    /// # Returns
    /// Returns `Ok(Recoder)` on successful creation.
    /// Returns `Err(RLNCError::NotEnoughPiecesToRecode)` if the input `data` is empty
    /// or does not contain at least one full coded piece.
    /// Returns `Err(RLNCError::PieceLengthZero)` if `full_coded_piece_byte_len` is zero.
    /// Returns `Err(RLNCError::PieceCountZero)` if `num_pieces_coded_together` is zero.
    /// Returns `Err(RLNCError::PieceLengthTooShort)` if `full_coded_piece_byte_len` is not greater than `num_pieces_coded_together`.
    pub fn new(data: Vec<u8>, full_coded_piece_byte_len: usize, num_pieces_coded_together: usize) -> Result<Recoder, RLNCError> {
        if data.len() == 0 {
            return Err(RLNCError::NotEnoughPiecesToRecode);
        }
        if full_coded_piece_byte_len == 0 {
            return Err(RLNCError::PieceLengthZero);
        }
        if num_pieces_coded_together == 0 {
            return Err(RLNCError::PieceCountZero);
        }
        if full_coded_piece_byte_len <= num_pieces_coded_together {
            return Err(RLNCError::PieceLengthTooShort);
        }

        let piece_byte_len = full_coded_piece_byte_len - num_pieces_coded_together;
        let num_pieces_received = data.len() / full_coded_piece_byte_len;

        let mut coding_vectors = Vec::with_capacity(num_pieces_received * num_pieces_coded_together);
        let mut coded_pieces = Vec::with_capacity(num_pieces_received * piece_byte_len);

        data.chunks_exact(full_coded_piece_byte_len).for_each(|full_coded_piece| {
            let coding_vector = &full_coded_piece[..num_pieces_coded_together];
            let coded_piece = &full_coded_piece[num_pieces_coded_together..];

            coding_vectors.extend(coding_vector.iter().map(|&symbol| Gf256::new(symbol)));
            coded_pieces.extend_from_slice(coded_piece);
        });

        let encoder = unsafe { Encoder::without_padding(coded_pieces, num_pieces_received).unwrap_unchecked() };

        Ok(Recoder {
            coding_vectors,
            encoder,
            num_pieces_received,
            full_coded_piece_byte_len,
            num_pieces_coded_together,
        })
    }

    /// Generates a new coded piece by recoding the source pieces using a randomly sampled coding vector.
    ///
    /// This method generates a random recoding vector (length `self.get_num_pieces_recoded_together()`),
    /// computes the resulting coding vector for the original source pieces by
    /// multiplying the random vector by the matrix of received coding vectors,
    /// and then uses the internal `Encoder` to produce a new coded piece based
    /// on this computed coding vector.
    ///
    /// The output is a vector containing the computed source coding vector
    /// prepended to the newly generated coded piece.
    ///
    /// # Arguments
    ///
    /// * `rng`: Used to sample the random recoding vector.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<u8>` representing the new coded piece prepended with its
    /// source coding vector. The length of the returned vector is
    /// `self.get_full_coded_piece_byte_len()`.
    pub fn recode<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<u8> {
        let random_recoding_vector = (0..self.num_pieces_received).map(|_| rng.random()).collect::<Vec<Gf256>>();

        // Compute the resulting coding vector for the original source pieces
        // by multiplying the random sampled recoding vector by the matrix of received coding vectors.
        let computed_coding_vector = (0..self.num_pieces_coded_together)
            .map(|coeff_idx| {
                random_recoding_vector
                    .iter()
                    .enumerate()
                    .fold(Gf256::default(), |acc, (recoding_vec_idx, &cur)| {
                        let row_begins_at = recoding_vec_idx * self.num_pieces_coded_together;
                        acc + cur * self.coding_vectors[row_begins_at + coeff_idx]
                    })
                    .get()
            })
            .collect::<Vec<u8>>();

        let full_coded_piece = unsafe { self.encoder.code_with_coding_vector(&random_recoding_vector).unwrap_unchecked() };
        let coded_piece = &full_coded_piece[self.num_pieces_received..];

        let mut full_recoded_piece = vec![0u8; self.full_coded_piece_byte_len];

        full_recoded_piece[..self.num_pieces_coded_together].copy_from_slice(&computed_coding_vector);
        full_recoded_piece[self.num_pieces_coded_together..].copy_from_slice(coded_piece);

        full_recoded_piece
    }
}
