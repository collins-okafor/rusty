#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
        dbg!(b);
    }
}