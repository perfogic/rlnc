#![cfg(test)]

use crate::{decoder::Decoder, encoder::Encoder, errors::RLNCError};
use rand::Rng;

#[test]
fn test_rlnc_encoder_decoder_execution_tests() {
    const NUM_TEST_ITERATIONS: usize = 10;

    const MIN_DATA_BYTE_LEN: usize = 1usize << 10;
    const MAX_DATA_BYTE_LEN: usize = 1usize << 16;

    const MIN_PIECE_COUNT: usize = 1usize << 5;
    const MAX_PIECE_COUNT: usize = 1usize << 11;

    let mut rng = rand::rng();

    (0..NUM_TEST_ITERATIONS).for_each(|_| {
        let data_byte_len = rng.random_range(MIN_DATA_BYTE_LEN..=MAX_DATA_BYTE_LEN);
        let piece_count = rng.random_range(MIN_PIECE_COUNT..=MAX_PIECE_COUNT);

        let data = (0..data_byte_len).map(|_| rng.random()).collect::<Vec<u8>>();
        let data_copy = data.clone();

        let (encoder, piece_byte_len) = Encoder::new(data, piece_count);
        let mut decoder = Decoder::new(piece_byte_len, piece_count);

        loop {
            let coded_piece = encoder.code(&mut rng).expect("Generating new RLNC coded piece must not fail!");

            match decoder.decode(&coded_piece) {
                Ok(_) => {}
                Err(e) => match e {
                    RLNCError::ReceivedAllPieces => break,
                    RLNCError::PieceNotUseful => continue,
                    _ => panic!("Did not expect this error during decoding: {}", e),
                },
            };
        }

        assert_eq!(decoder.is_already_decoded(), true);
        let decoded_data = decoder.get_decoded_data().expect("Extracting decoded data must not fail!");

        assert_eq!(data_copy, decoded_data);
    });
}
