use super::{parsing, player, AnField};

#[test]
fn ft_extract_player() {
    let results = vec![
        (parsing::extract_player("$$$ exec p1 : [robots/bender]"),Some(1)),
        (parsing::extract_player("$$$ exec p30 : [robots/bender]"),Some(30)),
        (parsing::extract_player("exec p1 : [robots/bender]"),None),
        (parsing::extract_player("exec p : [robots/bender]"),None),
        (parsing::extract_player(""),None),
    ];
    for result in results {
        assert_eq!(result.0,result.1);
    }
}

#[test]
fn ft_extract_wl() {
    let results = vec![
        ((parsing::extract_anfield_lenght_width("Anfield 20 15:")),Some((20,15))),
        ((parsing::extract_anfield_lenght_width("Anfield 20 15")),None),
        ((parsing::extract_anfield_lenght_width("Anfield 20")),None),
        ((parsing::extract_anfield_lenght_width("Antfield 20 16:")),None),
    ];
    for result in results {
        assert_eq!(result.0,result.1);
    }
}

#[test]
// depend on all previous tests
fn mt_af_parse() {
    let f1: &str = 
        "$$$ exec p1 : [robots/bender]
        Anfield 20 15:
            01234567890123456789
        000 ....................
        001 ....................
        002 .........@..........
        003 ....................
        004 ....................
        005 ....................
        006 ....................
        007 ....................
        008 ....................
        009 ....................
        010 ....................
        011 ....................
        012 .........$..........
        013 ....................
        014 ....................
        Piece 4 1:
        .OO.";

    let f1_expected = 
    "AnField { player: Player { id: 1, chars: \"@a\" }, self_owned: [Point { x: 9, y: 2 }], ennemie_owned: [Point { x: 9, y: 12 }], length: 15, width: 20, piece: Piece { slots: [Used(Point { x: 0, y: 0 }), Used(Point { x: 1, y: 0 }), Used(Point { x: 2, y: 0 }), Used(Point { x: 3, y: 0 }), Used(Point { x: 4, y: 0 }), Used(Point { x: 5, y: 0 }), Used(Point { x: 6, y: 0 }), Used(Point { x: 7, y: 0 }), Unused(Point { x: 8, y: 0 }), Used(Point { x: 9, y: 0 }), Used(Point { x: 10, y: 0 }), Unused(Point { x: 11, y: 0 })] } }";    
    let f2: &str = 
        "$$$ exec p1 : [robots/bender]
        Anfield 20 15:
            01234567890123456789
        000 ....................
        001 ....................
        002 ........a@a.........
        003 .......aaa..........
        004 .......aa...........
        005 ....................
        006 ....................
        007 ....................
        008 ....................
        009 ....................
        010 .........ss.........
        011 ........sss.........
        012 .........$s.........
        013 ....................
        014 ....................
        Piece 4 1:
        .OO.";

    let f2_expected = 
    "AnField { player: Player { id: 1, chars: \"@a\" }, self_owned: [Point { x: 8, y: 2 }, Point { x: 9, y: 2 }, Point { x: 10, y: 2 }, Point { x: 7, y: 3 }, Point { x: 8, y: 3 }, Point { x: 9, y: 3 }, Point { x: 7, y: 4 }, Point { x: 8, y: 4 }], ennemie_owned: [Point { x: 9, y: 10 }, Point { x: 10, y: 10 }, Point { x: 8, y: 11 }, Point { x: 9, y: 11 }, Point { x: 10, y: 11 }, Point { x: 9, y: 12 }, Point { x: 10, y: 12 }], length: 15, width: 20, piece: Piece { slots: [Used(Point { x: 0, y: 0 }), Used(Point { x: 1, y: 0 }), Used(Point { x: 2, y: 0 }), Used(Point { x: 3, y: 0 }), Used(Point { x: 4, y: 0 }), Used(Point { x: 5, y: 0 }), Used(Point { x: 6, y: 0 }), Used(Point { x: 7, y: 0 }), Unused(Point { x: 8, y: 0 }), Used(Point { x: 9, y: 0 }), Used(Point { x: 10, y: 0 }), Unused(Point { x: 11, y: 0 })] } }";

    assert_eq!(format!("{:?}",parsing::AnField::parse(f1)),f1_expected);
    assert_eq!(format!("{:?}",parsing::AnField::parse(f2)),f2_expected);
    
}