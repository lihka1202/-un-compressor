use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct HuffmanNode {
    symbol: Option<u8>,
    frequency: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn new_node(symbol: Option<u8>, frequency: u32) -> Box<HuffmanNode> {
    Box::new(HuffmanNode { symbol, frequency, left: None, right: None })
}