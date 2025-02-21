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
        ((parsing::extract_lenght_width("Anfield 20 15:")),Some((20,15))),
        ((parsing::extract_lenght_width("Anfield 20 15")),None),
        ((parsing::extract_lenght_width("Anfield 20")),None),
        ((parsing::extract_lenght_width("Antfield 20 16:")),None),
    ];
    for result in results {
        assert_eq!(result.0,result.1);
    }
}