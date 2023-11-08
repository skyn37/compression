Creating a Huffman Encoder/Decoder in Rust
===========================================

This is a Huffman Encoder/Decoder implemented in Rust for learning purposes. Huffman coding is a data compression algorithm that assigns variable-length codes to input characters based on their frequencies.

![Huffman Tree](https://upload.wikimedia.org/wikipedia/commons/thumb/8/82/Huffman_tree_2.svg/250px-Huffman_tree_2.svg.png)


About Huffman Coding
--------------------

Huffman coding is a variable-length prefix coding algorithm used for lossless data compression. It's based on the frequency of characters in the input data, with more frequent characters getting shorter codes.

The encoding process includes:
1. Reading the text and determining the frequency of each character.
2. Building a binary tree from the character frequencies.
3. Generating a prefix-code table from the tree.
4. Encoding the text using the code table.
5. Writing the encoded tree and text to an output file.

The decoding process includes:
1. Reading the encoded file.
2. Rebuilding the prefix table from the header.
3. Decoding the encoded text.
4. Writing the decoded text to an output file.

Acknowledgments
--------------

This project was inspired by the [Huffman Challenge](https://codingchallenges.fyi/challenges/challenge-huffman) from CodingChallenges.fyi and the concepts of Huffman coding. For more information, refer to the book "Huffman Coding" by David A. Huffman.

[![Rust](https://www.rust-lang.org/logos/rust-logo-32x32.png)](https://www.rust-lang.org/)

This implementation uses Rust, a systems programming language that provides memory safety and low-level control over hardware.

Contributors
------------

- [skyn37](https://github.com/yourusername) - Initial work

Feel free to contribute, report issues, and make suggestions. Happy learning!

⚠️ Please note that this is not a production tool. It is primarily for educational purposes, specifically learning Rust.
