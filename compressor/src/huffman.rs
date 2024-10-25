struct HuffmanNode {
    symbol: Option<u8>,
    frequency: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}