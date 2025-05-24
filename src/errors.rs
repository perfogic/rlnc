pub enum RLNCError {
    PieceNotUseful,
    ReceivedAllPieces,
    NotAllPiecesReceivedYet,
}

impl std::fmt::Display for RLNCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RLNCError::PieceNotUseful => write!(f, "Received piece is not useful"),
            RLNCError::ReceivedAllPieces => write!(f, "Received all pieces"),
            RLNCError::NotAllPiecesReceivedYet => write!(f, "Not all pieces are received yet"),
        }
    }
}
