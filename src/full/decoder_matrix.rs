use crate::{RLNCError, common::gf256::Gf256};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct DecoderMatrix {
    num_pieces_coded_together: usize,
    rows: usize,
    cols: usize,
    elements: Vec<u8>,
}

impl DecoderMatrix {
    /// Given RLNC encoding configuration, it sets up a decoder matrix.
    ///
    /// This decoder matrix can be used to add incoming erasure-coded pieces,
    /// and incrementally decode them using Gaussian Elimination, if it's a
    /// useful (i.e. linearly independent) piece.
    ///
    /// # Arguments
    /// * `num_pieces_coded_together` - The minimum number of useful coded pieces needed for decoding.
    /// * `piece_byte_length` - The byte length of each original data piece.
    ///
    /// # Returns
    /// An instance of decoder matrix - ready to use for decoding.
    pub fn new(num_pieces_coded_together: usize, piece_byte_length: usize) -> Self {
        let full_coded_piece_byte_len = num_pieces_coded_together + piece_byte_length;
        let total_byte_len = num_pieces_coded_together * full_coded_piece_byte_len;
        let elements = Vec::with_capacity(total_byte_len);

        Self {
            num_pieces_coded_together,
            rows: 0,
            cols: full_coded_piece_byte_len,
            elements,
        }
    }

    /// Adds a new row to the decoder matrix.
    ///
    /// # Arguments
    /// `row` - A byte slice, representing a full erasure-coded piece i.e. containing the coefficients followed by
    ///  the coded data for one piece. Its length must be `num_pieces_coded_together + piece_byte_length`.
    ///
    /// # Returns
    /// * Ok(&mut Self) - If full erasure-coded piece is of valid length.
    /// * Err(RLNCError::InvalidPieceLength) - If full erasure-coded piece length doesn't match expected value.
    pub fn add_row(&mut self, row: &[u8]) -> Result<&mut Self, RLNCError> {
        if row.len() != self.cols {
            return Err(RLNCError::InvalidPieceLength);
        }

        self.elements.extend_from_slice(row);
        self.rows += 1;

        Ok(self)
    }

    /// Swaps two rows in the decoder's matrix.
    ///
    /// # Arguments
    /// * `row1_idx` - The index of the first row.
    /// * `row2_idx` - The index of the second row.
    ///
    /// # Panics
    /// Panics if either row index is out of bounds.
    pub fn swap_rows(&mut self, row1_idx: usize, row2_idx: usize) -> &mut Self {
        let row1_begins_at = row1_idx * self.cols;
        let row2_begins_at = row2_idx * self.cols;

        // Swap each element in the two rows
        (0..self.cols).for_each(|cidx| self.elements.swap(row1_begins_at + cidx, row2_begins_at + cidx));

        self
    }

    /// Computes the Reduced Row Echelon Form (RREF) of the matrix.
    ///
    /// This involves forward elimination (`Self::clean_forward`), backward elimination
    /// (`Self::clean_backward`), and removing any resulting zero rows (`Self::remove_zero_rows`).
    ///
    /// This function updates the number of rows to reflect the current rank of the matrix.
    /// It is safe to call `Self::rank` after calling this function.
    pub fn rref(&mut self) -> &mut Self {
        self.clean_forward().clean_backward().remove_zero_rows()
    }

    /// Returns the current rank of the matrix, which is same as the number
    /// of rows, after calling `Self::rref`.
    pub fn rank(&self) -> usize {
        self.rows
    }

    /// Returns underlying data i.e. `self.rows` many full erasure-coded pieces.
    /// Calling this function, consumes the decoder matrix instance.
    pub fn extract_data(self) -> Vec<u8> {
        self.elements
    }

    /// Performs the forward phase of Gaussian elimination (to row echelon form).
    ///
    /// Pivots are selected, rows are swapped if necessary to get a non-zero
    /// pivot, and rows below the pivot are cleared by subtracting a multiple
    /// of the pivot row.
    fn clean_forward(&mut self) -> &mut Self {
        let boundary = self.rows.min(self.cols);

        for i in 0..boundary {
            if self[(i, i)] == Gf256::zero() {
                let mut is_non_zero_col = false;
                let mut pivot_row_idx = i + 1;

                while pivot_row_idx < self.rows {
                    if self[(pivot_row_idx, i)] != Gf256::zero() {
                        is_non_zero_col = true;
                        break;
                    }
                    pivot_row_idx += 1;
                }

                if !is_non_zero_col {
                    continue;
                }

                self.swap_rows(i, pivot_row_idx);
            }

            for j in (i + 1)..self.rows {
                if self[(j, i)] == Gf256::zero() {
                    continue;
                }

                let quotient = (self[(j, i)] / self[(i, i)]).unwrap();
                for k in i..self.cols {
                    self[(j, k)] = self[(j, k)] + self[(i, k)] * quotient;
                }
            }
        }

        self
    }

    /// Performs the backward phase of Gaussian elimination (to reduced row echelon form).
    ///
    /// Clears entries above the pivots and normalizes pivots to 1.
    fn clean_backward(&mut self) -> &mut Self {
        let boundary = self.rows.min(self.cols);

        for i in (0..boundary).rev() {
            if self[(i, i)] == Gf256::zero() {
                continue;
            }

            for j in 0..i {
                if self[(j, i)] == Gf256::zero() {
                    continue;
                }

                let quotient = (self[(j, i)] / self[(i, i)]).unwrap();
                for k in i..self.cols {
                    self[(j, k)] = self[(j, k)] + self[(i, k)] * quotient;
                }
            }

            if self[(i, i)] == Gf256::one() {
                continue;
            }

            let inv = self[(i, i)].inv().unwrap();
            self[(i, i)] = Gf256::one();

            for j in (i + 1)..self.cols {
                if self[(i, j)] == Gf256::zero() {
                    continue;
                }
                self[(i, j)] = self[(i, j)] * inv;
            }
        }

        self
    }

    /// Removes zero rows from the matrix and updates `useful_piece_count`.
    ///
    /// A row is considered a zero row if all its coefficient columns are zero.
    /// This step is crucial after RREF to determine the true rank and compact
    /// the matrix to only the useful rows.
    fn remove_zero_rows(&mut self) -> &mut Self {
        let mut i = 0;
        while i < self.rows {
            let is_nonzero_row = (0..self.num_pieces_coded_together).any(|cidx| self[(i, cidx)] != Gf256::zero());
            if is_nonzero_row {
                i += 1;
                continue;
            }

            let start_idx_of_row_to_remove = i * self.cols;
            let start_idx_of_next_row = (i + 1) * self.cols;

            if start_idx_of_next_row < self.elements.len() {
                self.elements.copy_within(start_idx_of_next_row.., start_idx_of_row_to_remove);
            }
            self.rows -= 1;
        }

        let updated_num_elements = self.rows * self.cols;
        self.elements.truncate(updated_num_elements);

        self
    }
}

impl Index<(usize, usize)> for DecoderMatrix {
    type Output = Gf256;

    /// Returns an immutable reference to an element of matrix at the specified row and column,
    /// converting it to a `Gf256` element.
    ///
    /// # Arguments
    /// * `index` - A tuple `(row_index, col_index)` specifying the position.
    ///
    /// # Returns
    /// Returns the element as a `Gf256`.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row_idx, col_idx) = index;
        let lin_idx = row_idx * self.cols + col_idx;

        unsafe { std::mem::transmute(self.elements.get_unchecked(lin_idx)) }
    }
}

impl IndexMut<(usize, usize)> for DecoderMatrix {
    /// Returns a mutable reference to an element of matrix at the specified row and column,
    /// converting it to a `Gf256` element.
    ///
    /// # Arguments
    /// * `index` - A tuple `(row_index, col_index)` specifying the position.
    /// * `val` - The `Gf256` value to set.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row_idx, col_idx) = index;
        let lin_idx = row_idx * self.cols + col_idx;

        unsafe { std::mem::transmute(self.elements.get_unchecked_mut(lin_idx)) }
    }
}
