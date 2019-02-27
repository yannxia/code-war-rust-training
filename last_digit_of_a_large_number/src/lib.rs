#[cfg(test)]
mod tests {
    fn last_digit(str1: &str, str2: &str) -> i32 {
        let s1;
        if str1.len() > 1 {
            s1 = (str1.split_at(str1.len() - 1).1).parse::<u32>().unwrap();
        } else {
            s1 = str1.parse::<u32>().unwrap();
        }

        let mut s2 = str2.parse::<u64>().unwrap();

        let mut result = s1;

        while s2 > 1 {
            result = result * s1;
            if result > 10 {
                result = result % 10;
            }

            s2 -= 1
        }


        result as i32
    }

    #[test]
    fn returns_expected() {
        assert_eq!(last_digit("4", "1"), 4);
        assert_eq!(last_digit("4", "2"), 6);
        assert_eq!(last_digit("9", "7"), 9);
        assert_eq!(last_digit("10", "10000000000"), 0);
        assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
        assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
    }
}
