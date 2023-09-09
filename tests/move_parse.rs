use analyzer::r#move::Move;
use analyzer::color::Color;

macro_rules! gen_move_tests {
    ($($fn:ident,$san:expr, $capture:expr, $disambiguity:expr,$piece:expr),+) => {
        $(
            #[test]
            fn $fn(){
                let mov = Move::new($san.to_string(), 0);
                assert_eq!(mov.is_capture, $capture, "Capture wrong");
                assert_eq!(mov.source.0.is_some()^mov.source.1.is_some(),$disambiguity, "Disambiguity wrong");
                assert_eq!(mov.piece.to_char(Color::Black),$piece, "Piece Wrong")
            }
        )+
    };
}

gen_move_tests!{
    // fn name , san, is capture, found disambiguity, 
    simple_move, "e4", false, false,'p',
    capture_disambiguos, "Rdxa7", true,true,'r',
    queen_castle,"O-O-O", false, false,'k',
    check, "Nf3+", false, false,'n'
}

    