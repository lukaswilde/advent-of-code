pub fn parse_text() -> String {
    match std::env::args().len() {
        2 => std::env::args()
            .nth(1)
            .expect("If there is only one argument, it should be the problem text"),
        3 => {
            assert!(std::env::args().nth(1).unwrap() == "-i");
            let filename = std::env::args()
                .nth(2)
                .expect("There should be a file as argument");

            std::fs::read_to_string(filename).expect("The file should exist")
        }
        _ => panic!("Either we have one argument (the problem text) or 2 (where it is -i file)"),
    }
}
