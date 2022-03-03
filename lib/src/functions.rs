pub fn add(x: i32, y: i32) -> i32 {
    add_impl(x, y)
}

fn add_impl(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add_impl(1, 2), 3);
    }
}
