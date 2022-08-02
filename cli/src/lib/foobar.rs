// TODO: remove this module

pub fn add(x: i32, y: i32) -> i32 {
    add_impl(x, y)
}

fn add_impl(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_add() {
        assert_eq!(add_impl(1, 2), 3);
    }

    #[test]
    fn test_read_testdata_file() -> Result<(), Box<dyn std::error::Error>> {
        let filename: PathBuf = [env!("CARGO_MANIFEST_DIR"), "testdata", "foo.txt"]
            .iter()
            .collect();
        let s = std::fs::read_to_string(&filename)?;
        assert_eq!(s, "foo\n");
        Ok(())
    }
}
