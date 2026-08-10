#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn counters() {
        let mut pos = TakPosition::<5>::startpos();
        assert_eq!(pos.fullmoves, 1);

        let a1 = pos.parse_move("a1").unwrap();
        pos.makemove(&a1);
        assert_eq!(pos.fullmoves, 1);

        let b1 = pos.parse_move("b1").unwrap();
        pos.makemove(&b1);
        assert_eq!(pos.fullmoves, 2);

        let c1 = pos.parse_move("c1").unwrap();
        pos.makemove(&c1);
        assert_eq!(pos.fullmoves, 2);

        let d1 = pos.parse_move("d1").unwrap();
        pos.makemove(&d1);
        assert_eq!(pos.fullmoves, 3);

        pos.undomove(&d1);
        assert_eq!(pos.fullmoves, 2);

        pos.undomove(&c1);
        assert_eq!(pos.fullmoves, 2);

        pos.undomove(&b1);
        assert_eq!(pos.fullmoves, 1);

        pos.undomove(&a1);
        assert_eq!(pos.fullmoves, 1);
    }
}
