pub enum RLNCError {
    PieceNotUseful,
}

impl std::fmt::Display for RLNCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RLNCError::PieceNotUseful => write!(f, "Received piece is not useful"),
        }
    }
}
