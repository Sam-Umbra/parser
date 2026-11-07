#[cfg(test)]
mod basic {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        let s = Expression::from_str("1");
        assert_eq!(s.to_string(), "1");
    }

    #[test]
    fn test_2() {
        let s = Expression::from_str("1 + 2");
        assert_eq!(s.to_string(), "(+ 1 2)");
    }

    #[test]
    fn test_3() {
        let s = Expression::from_str("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
    }

    #[test]
    fn test_4() {
        let s = Expression::from_str("(1 + b) * 6");
        assert_eq!(s.to_string(), "(* (+ 1 b) 6)");
    }

    #[test]
    fn test_5() {
        let s = Expression::from_str("12");
        assert_eq!(s.to_string(), "12");
    }

    #[test]
    fn test_6() {
        let s = Expression::from_str("123 + 45");
        assert_eq!(s.to_string(), "(+ 123 45)");
    }

    #[test]
    fn test_7() {
        let s = Expression::from_str("12 * 34 + 5");
        assert_eq!(s.to_string(), "(+ (* 12 34) 5)");
    }

    #[test]
    fn test_8() {
        let s = Expression::from_str("3.14 + 2.5");
        assert_eq!(s.to_string(), "(+ 3.14 2.5)");
    }
}

#[cfg(test)]
mod associativity {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        assert_eq!(
            Expression::from_str("10 - 4 - 3").to_string(),
            "(- (- 10 4) 3)"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Expression::from_str("100 / 10 / 2").to_string(),
            "(/ (/ 100 10) 2)"
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Expression::from_str("2^3^2").to_string(), "(^ 2 (^ 3 2))");
    }
}

#[cfg(test)]
mod implicit_mul {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        let s = Expression::from_str("2a");
        assert_eq!(s.to_string(), "(* 2 a)");
    }

    #[test]
    fn test_2() {
        let s = Expression::from_str("a3");
        assert_eq!(s.to_string(), "(* a 3)");
    }

    #[test]
    fn test_3() {
        let s = Expression::from_str("5.5g");
        assert_eq!(s.to_string(), "(* 5.5 g)");
    }

    #[test]
    fn test_4() {
        let s = Expression::from_str("12x");
        assert_eq!(s.to_string(), "(* 12 x)");
    }

    #[test]
    fn test_5() {
        let s = Expression::from_str("2a3");
        assert_eq!(s.to_string(), "(* (* 2 a) 3)");
    }

    #[test]
    fn test_6() {
        let s = Expression::from_str("2a3b4");
        assert_eq!(s.to_string(), "(* (* (* (* 2 a) 3) b) 4)");
    }

    #[test]
    fn test_7() {
        let s = Expression::from_str("2a + 3");
        assert_eq!(s.to_string(), "(+ (* 2 a) 3)");
    }

    #[test]
    fn test_8() {
        let s = Expression::from_str("12x + 3y");
        assert_eq!(s.to_string(), "(+ (* 12 x) (* 3 y))");
    }

    #[test]
    fn test_9() {
        let s = Expression::from_str("1 + 2a * 3");
        assert_eq!(s.to_string(), "(+ 1 (* (* 2 a) 3))");
    }

    #[test]
    fn test_10() {
        let s = Expression::from_str("(2a + 1) * 3");
        assert_eq!(s.to_string(), "(* (+ (* 2 a) 1) 3)");
    }

    #[test]
    fn test_11() {
        let s = Expression::from_str("x - 4d");
        assert_eq!(s.to_string(), "(- x (* 4 d))");
    }

    #[test]
    fn test_12() {
        let s = Expression::from_str("2a^3");
        assert_eq!(s.to_string(), "(* 2 (^ a 3))");
    }

    #[test]
    fn test_13() {
        let s = Expression::from_str("a2^3");
        assert_eq!(s.to_string(), "(* a (^ 2 3))");
    }
}

// ---------- Novos (falham até a implementação existir) ----------

#[cfg(test)]
mod implicit_precedence {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        let s = Expression::from_str("10 / 2a");
        assert_eq!(s.to_string(), "(/ 10 (* 2 a))");
    }

    #[test]
    fn test_2() {
        let s = Expression::from_str("2a / 3");
        assert_eq!(s.to_string(), "(/ (* 2 a) 3)");
    }

    #[test]
    fn test_3() {
        let s = Expression::from_str("6 * 2a");
        assert_eq!(s.to_string(), "(* 6 (* 2 a))");
    }

    #[test]
    fn test_4() {
        let s = Expression::from_str("3a^2");
        assert_eq!(s.to_string(), "(* 3 (^ a 2))");
    }
}

#[cfg(test)]
mod implicit_extended {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        let s = Expression::from_str("2ab");
        assert_eq!(s.to_string(), "(* (* 2 a) b)");
    }

    #[test]
    fn test_2() {
        let s = Expression::from_str("3xyz");
        assert_eq!(s.to_string(), "(* (* (* 3 x) y) z)");
    }

    #[test]
    fn test_3() {
        let s = Expression::from_str("2(x+1)");
        assert_eq!(s.to_string(), "(* 2 (+ x 1))");
    }

    #[test]
    fn test_4() {
        let s = Expression::from_str("2.5(x+1)");
        assert_eq!(s.to_string(), "(* 2.5 (+ x 1))");
    }
}

#[cfg(test)]
mod spacing_errors {
    use crate::expression::Expression;

    #[test]
    #[should_panic]
    fn test_1() {
        Expression::from_str("2 a");
    }

    #[test]
    #[should_panic]
    fn test_2() {
        Expression::from_str("a 2");
    }

    #[test]
    #[should_panic]
    fn test_3() {
        Expression::from_str("a b");
    }

    #[test]
    #[should_panic]
    fn test_4() {
        Expression::from_str("12 34");
    }
}

#[cfg(test)]
mod equals {
    use crate::expression::Expression;

    #[test]
    fn test_1() {
        let s = Expression::from_str("y = 2x");
        assert_eq!(s.to_string(), "(= y (* 2 x))");
    }

    #[test]
    fn test_2() {
        let s = Expression::from_str("x = 2a + 1");
        assert_eq!(s.to_string(), "(= x (+ (* 2 a) 1))");
    }

    #[test]
    #[should_panic]
    fn test_3() {
        Expression::from_str("2 = a+1 = 3");
    }

    #[test]
    #[should_panic]
    fn test_4() {
        Expression::from_str("x = y = 3");
    }
}

#[cfg(test)]
mod invalid_input {
    use crate::expression::Expression;

    #[test]
    #[should_panic]
    fn test_1() {
        Expression::from_str("1 +");
    }

    #[test]
    #[should_panic]
    fn test_2() {
        Expression::from_str("1 + + 2");
    }

    #[test]
    #[should_panic]
    fn test_3() {
        Expression::from_str("(1 + 2");
    }

    #[test]
    #[should_panic]
    fn test_4() {
        Expression::from_str("1 + 2)");
    }

    #[test]
    #[should_panic]
    fn test_5() {
        Expression::from_str("()");
    }

    #[test]
    #[should_panic]
    fn test_6() {
        Expression::from_str("1 @ 2");
    }
}
