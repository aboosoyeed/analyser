use analyzer::move_::Move;


macro_rules! gen_move_tests {
    ($($fn:ident,$san:expr, $capture:expr, $disambiguity:expr),+) => {
        $(
            #[test]
            fn $fn(){
                let mov = Move::new($san.to_string(), 0);
                assert_eq!(mov.is_capture, $capture);
                assert_eq!(mov.source.0.is_some()^mov.source.1.is_some(),$disambiguity);
            }
        )+
    };
}

gen_move_tests!{
    // fn name , san, is capture, found disambiguity, 
    simple_move, "e4", false, false,
    capture_disambiguos, "Rdxa7", true,true,
    queen_castle,"O-O-O", false, false,
    check, "Nf3+", false, false
}

    