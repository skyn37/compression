use rmp_serde;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Node {
    character: Option<char>,
    frequency: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct ForWrite {
    tree: Node,
    codes: Vec<u8>,
}

impl ForWrite {
    fn new(tree: Node, codes: Vec<u8>) -> ForWrite {
        ForWrite { tree, codes }
    }
}

impl Node {
    fn new(char: char, frequency: i32) -> Node {
        Node {
            character: Some(char),
            frequency,
            left: None,
            right: None,
        }
    }

    fn new_internal(frequency: i32, left: Node, right: Node) -> Node {
        Node {
            character: None,
            frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

fn generate_huffman_codes(node: &Node, code: String, codes: &mut Vec<(char, String)>) {
    if let Some(symbol) = node.character {
        codes.push((symbol, code));
    } else {
        if let Some(left) = &node.left {
            let mut left_code = code.clone();
            left_code.push('0');
            generate_huffman_codes(left, left_code, codes);
        }
        if let Some(right) = &node.right {
            let mut right_code = code.clone();
            right_code.push('1');
            generate_huffman_codes(right, right_code, codes);
        }
    }
}

fn bytes_to_binary_string(bytes: &[u8]) -> String {
    let mut binary_string = String::new();
    let last_padding_count = bytes[0];
    for (index, &byte) in bytes.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let mut binary = format!("{:08b}", byte);
        if index == bytes.len() - 1 {
            let (first, _) = binary.split_at(last_padding_count as usize);
            binary = first.into();
        }
        binary_string.push_str(&binary);
    }

    return binary_string;
}

fn build_tree(freq_map: HashMap<char, i32>) -> Option<Node> {
    let mut heap = BinaryHeap::new();

    for (key, value) in freq_map {
        let node = Node::new(key, value);
        heap.push(Reverse(node));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap().0;
        let right = heap.pop().unwrap().0;

        let combined_freq = left.frequency + right.frequency;
        let internal_node = Node::new_internal(combined_freq, left, right); // Reverse the order

        heap.push(Reverse(internal_node));
    }

    return heap.pop().map(|node| node.0);
}

fn encode_data(data: &str, huffman_codes: &Vec<(char, String)>) -> (String, Vec<u8>) {
    let mut encoded_data = String::new();
    let mut encoded_data_bytes = Vec::new();

    let mut bit_count = 0;
    let mut bit_buffer = String::new();

    for ch in data.chars() {
        if let Some(code) = huffman_codes.iter().find(|(c, _)| *c == ch) {
            let code_str = &code.1;
            encoded_data.push_str(code_str);
        }
    }

    let remainder = format!("{:b}", encoded_data.len() % 8);
    let mut index_byte = String::new();
    let mut pad = String::new();
    for _ in 0..8 - remainder.len() {
        pad.push('0');
    }

    index_byte.push_str(&pad);
    index_byte.push_str(&remainder);

    encoded_data_bytes.push(u8::from_str_radix(&index_byte, 2).expect("Eerr"));
    for bit in encoded_data.chars() {
        if bit == '0' || bit == '1' {
            bit_buffer.push(bit);
            bit_count += 1;
            if bit_count == 8 {
                encoded_data_bytes.push(u8::from_str_radix(&bit_buffer, 2).expect("Eerr"));
                bit_count = 0;
                bit_buffer.clear();
            }
        }
    }
    if bit_buffer.len() > 0 {
        for _ in 0..8 - bit_buffer.len() {
            bit_buffer.push('0');
        }
        encoded_data_bytes.push(u8::from_str_radix(&bit_buffer, 2).expect("err"));
    }

    (encoded_data, encoded_data_bytes)
}

fn decode_huffman(encoded_data: &str, huffman_tree: &Node) -> String {
    let mut decoded_data = String::new();
    let mut current_node = huffman_tree;

    for bit in encoded_data.chars() {
        if bit == '0' {
            // Move to the left child for '0'
            if let Some(left) = &current_node.left {
                current_node = left;
            }
        } else if bit == '1' {
            // Move to the right child for '1'
            if let Some(right) = &current_node.right {
                current_node = right;
            }
        }

        if let Some(symbol) = current_node.character {
            // Reached a leaf node, found a symbol
            decoded_data.push(symbol);
            // Reset to the root of the tree
            current_node = huffman_tree;
        }
    }

    decoded_data
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);
    let text = fs::read_to_string(file_path).expect("error has occured when reading from a file");
    let data = text.as_str();
    // let data = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEKKKKKKKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMMMMMMMMMMMMMMMMMMMMMMUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUZZ";
    // let text = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEKKKKKKKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMMMMMMMMMMMMMMMMMMMMMMUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUZZ";
    // let data = "ABC CCC";
    // let text = "ABC CCC";
    let mut character_frequencies: HashMap<char, i32> = HashMap::new();

    for char in data.chars() {
        match character_frequencies.get(&char) {
            Some(val) => {
                let count = val.clone() + 1;
                character_frequencies.insert(char, count);
            }
            None => {
                character_frequencies.insert(char, 1);
            }
        }
    }
    let huffman_tree = build_tree(character_frequencies);

    if let Some(value) = huffman_tree {
        let mut huffman_codes: Vec<(char, String)> = vec![];
        generate_huffman_codes(&value, String::new(), &mut huffman_codes);

        let (_, encoded_data_bytes) = encode_data(data, &huffman_codes);

        let testo_mesto = ForWrite::new(value, encoded_data_bytes);
        let serialized_tree = rmp_serde::encode::to_vec(&testo_mesto).expect("err");
        let data_to_write = serialized_tree;
        if let Ok(mut file) = File::create("combined_data.bin") {
            if let Err(err) = file.write_all(&data_to_write) {
                println!("Error writing combined data: {:?}", err);
            }
        }
        let mut serialized_tree_new = Vec::new();
        if let Ok(mut file) = File::open("combined_data.bin") {
            if let Err(err) = file.read_to_end(&mut serialized_tree_new) {
                println!("Error reading combined data: {:?}", err);
            }
        }
        let read_data: ForWrite = rmp_serde::decode::from_slice(&serialized_tree_new).expect("err");
        let encoded_data_string = bytes_to_binary_string(&read_data.codes);
        let decoded = decode_huffman(&encoded_data_string, &read_data.tree);
    }
}
