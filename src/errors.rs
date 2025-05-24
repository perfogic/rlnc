pub enum RLNCError {
    PieceNotUseful,
    ReceivedAllPieces,
    NotAllPiecesReceivedYet,
    NotEnoughPiecesToRecode,
    CodingVectorLengthMismatch,
}

impl std::fmt::Display for RLNCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RLNCError::PieceNotUseful => write!(f, "Received piece is not useful"),
            RLNCError::ReceivedAllPieces => write!(f, "Received all pieces"),
            RLNCError::NotAllPiecesReceivedYet => write!(f, "Not all pieces are received yet"),
            RLNCError::NotEnoughPiecesToRecode => write!(f, "Not enough pieces received to recode"),
            RLNCError::CodingVectorLengthMismatch => write!(f, "Coding vector length mismatch"),
        }
    }
}
