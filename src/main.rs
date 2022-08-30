use rayon::prelude::*;
use std::io::{BufRead, BufReader};
use std::{fs, io};

struct Trie {
    mask: u32,
    children: [Option<Box<Trie>>; 26],
}

// vowels
static VOWELS: u32 = 56656000;
//QJEARIOTNSLCUDPMHGBFYWKVXZ
static ENCODING: [u32; 26] = [
    1 << 24, // A
    1 << 9,  // B
    1 << 16, // C
    1 << 14, // D
    1 << 25, // E
    1 << 8,  // F
    1 << 10, // G
    1 << 11, // H
    1 << 22, // I
    1 << 1,  // J
    1 << 5,  // K
    1 << 17, // L
    1 << 12, // M
    1 << 19, // N
    1 << 21, // O
    1 << 13, // P
    1 << 0,  // Q
    1 << 23, // R
    1 << 18, // S
    1 << 20, // T
    1 << 15, // U
    1 << 4,  // V
    1 << 6,  // W
    1 << 3,  // X
    1 << 7,  // Y
    1 << 2,  // Z
];

impl Trie {
    pub fn new() -> Trie {
        Trie {
            mask: 0,
            children: Default::default(),
        }
    }

    pub fn addword(&mut self, word: u32) -> () {
        let mut node = self;
        for i in 0..26 {
            if (word >> i) & 1 == 1 {
                node = node.addchild(i);
            }
        }
    }

    pub fn addchild(&mut self, index: u32) -> &mut Box<Trie> {
        if self.mask & (1 << index) > 0 {
            return self.children[index as usize].as_mut().unwrap();
        } else {
            self.mask |= 1 << index;
            self.children[index as usize] = Some(Box::new(Trie::new()));
            return self.children[index as usize].as_mut().unwrap();
        }
    }

    pub fn search(&self, used: u32, words: &mut Vec<u32>) {
        if words.len() < 5 {
            if (!used & VOWELS) == 0 {
                return;
            }
            self.findword(used, words);
        } else {
            println!("Found combination");
        }
    }

    pub fn findword(&self, used: u32, words: &mut Vec<u32>) {
        let mut available1 = self.mask & !used;
        if words.len() > 0 {
            let last = words.get(words.len() - 1).unwrap();
            let trailing = last.trailing_zeros();
            available1 &= !((1 << trailing) - 1);
        }
        if available1 == 0 {
            return;
        }
        let mut unused = 0;
        for i in 0..26 {
            if ((used >> i) & 1) == 0 {
                unused += 1;
                if unused > 2 {
                    return;
                }
            }
            if ((available1 >> i) & 1) > 0 {
                let root2 = self.children[i].as_ref().unwrap();
                let available2 = root2.mask & !used;
                if available2 == 0 {
                    continue;
                }
                for j in 0..26 {
                    if ((available2 >> j) & 1) > 0 {
                        let root3 = root2.children[j].as_ref().unwrap();
                        let available3 = root3.mask & !used;
                        if available3 == 0 {
                            continue;
                        }
                        for k in 0..26 {
                            if ((available3 >> k) & 1) > 0 {
                                let root4 = root3.children[k].as_ref().unwrap();
                                let available4 = root4.mask & !used;
                                if available4 == 0 {
                                    continue;
                                }
                                for l in 0..26 {
                                    if ((available4 >> l) & 1) > 0 {
                                        let root5 = root4.children[l].as_ref().unwrap();
                                        let available5 = root5.mask & !used;
                                        if available5 == 0 {
                                            continue;
                                        }
                                        for m in 0..26 {
                                            if ((available5 >> m) & 1) > 0 {
                                                let wordmask = (1 << i)
                                                    | (1 << j)
                                                    | (1 << k)
                                                    | (1 << l)
                                                    | (1 << m);
                                                let newused = used | wordmask;
                                                words.push(wordmask);
                                                self.search(newused, words);
                                                words.pop();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut words: Vec<String> = file_to_vec("wordle-nyt-allowed-guesses.txt".to_owned()).unwrap();
    words.append(&mut file_to_vec("wordle-nyt-answers-alphabetical.txt".to_owned()).unwrap());
    let mut cooked: Vec<u32> = words
        .iter()
        .map(|x| encodewords(x))
        .filter(|i| i.count_ones() == 5)
        .collect();
    cooked.sort();
    cooked.dedup();

    let mut trie: Trie = Trie::new();
    for word in &cooked {
        trie.addword(*word);
    }
    let starts: Vec<u32> = cooked.into_iter().filter(|word| (*word & 3) > 0).collect();

    starts
        .par_iter()
        .for_each(|word| trie.search(*word, &mut vec![*word]));

    //trie.search(0, &mut Vec::new());

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn encodewords(word: &String) -> u32 {
    let mut mask: u32 = 0;
    for c in word.chars() {
        mask |= ENCODING[c as usize - 97];
        //mask |= 1 << 26 >> (c as u32 - 96);
        //mask |= 1 << (c as u32 - 97);
    }
    return mask;
}
