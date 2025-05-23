use crate::gf256::Gf256;

#[derive(Clone)]
pub struct Decoder {
    data: Vec<u8>,
    piece_byte_len: usize,
    required_piece_count: usize,
    received_piece_count: usize,
    useful_piece_count: usize,
}

impl Decoder {
    pub fn new(piece_byte_len: usize, required_piece_count: usize) -> Decoder {
        let full_coded_piece_byte_len = required_piece_count + piece_byte_len;
        let total_byte_len = required_piece_count * full_coded_piece_byte_len;
        let data = Vec::with_capacity(total_byte_len);

        Decoder {
            data,
            piece_byte_len,
            required_piece_count,
            received_piece_count: 0,
            useful_piece_count: 0,
        }
    }

    fn get(&self, index: (usize, usize)) -> Gf256 {
        let (r_index, c_index) = index;
        Gf256::new(self.data[r_index * self.useful_piece_count + c_index])
    }

    fn set(&mut self, index: (usize, usize), val: Gf256) {
        let (r_index, c_index) = index;
        self.data[r_index * self.useful_piece_count + c_index] = val.get();
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        let cols = self.required_piece_count + self.piece_byte_len;

        for c in 0..cols {
            let idx1 = row1 * cols + c;
            let idx2 = row2 * cols + c;
            self.data.swap(idx1, idx2);
        }
    }

    fn clean_forward(&mut self) {
        let rows = self.useful_piece_count;
        let cols = self.required_piece_count + self.piece_byte_len;
        let boundary = rows.min(cols);

        for i in 0..boundary {
            if self.get((i, i)) == Gf256::zero() {
                let mut is_non_zero_col = false;
                let mut pivot = i + 1;

                while pivot < rows {
                    if self.get((pivot, i)) != Gf256::zero() {
                        is_non_zero_col = true;
                        break;
                    }

                    pivot += 1;
                }

                if !is_non_zero_col {
                    continue;
                }

                self.swap_rows(i, pivot);
            }

            for j in (i + 1)..rows {
                if self.get((j, i)) == Gf256::zero() {
                    continue;
                }

                let quotient = (self.get((j, i)) / self.get((i, i))).unwrap();
                for k in i..cols {
                    self.set((j, k), self.get((j, k)) + self.get((i, k)) * quotient);
                }
            }
        }
    }

    fn clean_backward(&mut self) {
        let rows = self.useful_piece_count;
        let cols = self.required_piece_count + self.piece_byte_len;
        let boundary = rows.min(cols);

        for i in (0..boundary).rev() {
            if self.get((i, i)) == Gf256::zero() {
                continue;
            }

            for j in 0..i {
                if self.get((j, i)) == Gf256::zero() {
                    continue;
                }

                let quotient = (self.get((j, i)) / self.get((i, i))).unwrap();
                for k in i..cols {
                    self.set((j, k), self.get((j, k)) + self.get((i, k)) * quotient);
                }
            }

            if self.get((i, i)) == Gf256::one() {
                continue;
            }

            let inv = self.get((i, i)).inv().unwrap();
            self.set((i, i), Gf256::one());

            for j in (i + 1)..cols {
                if self.get((i, j)) == Gf256::zero() {
                    continue;
                }

                self.set((i, j), self.get((i, j)) * inv);
            }
        }
    }

    fn remove_zero_rows(&mut self) {
        let mut rows = self.useful_piece_count;
        let cols = self.required_piece_count + self.piece_byte_len;
        let coeff_cols = self.required_piece_count;

        let mut i = 0;
        while i < rows {
            let is_nonzero_row = (0..coeff_cols).fold(false, |is_nonzero, cidx| {
                is_nonzero || (self.get((i, cidx)) != Gf256::zero())
            });

            if is_nonzero_row {
                i += 1;
                continue;
            }

            let start_index_of_row_to_remove = i * cols;
            let start_index_of_next_row = (i + 1) * cols;
            let end_index_of_useful_data = self.useful_piece_count * cols;

            if start_index_of_next_row < end_index_of_useful_data {
                self.data.copy_within(
                    start_index_of_next_row..end_index_of_useful_data,
                    start_index_of_row_to_remove,
                );
            }

            rows -= 1;
        }

        let total_byte_len = rows * cols;
        self.data.truncate(total_byte_len);
    }

    fn rref(&mut self) {
        self.clean_forward();
        self.clean_backward();
        self.remove_zero_rows();
    }
}
