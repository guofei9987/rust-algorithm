/*
前缀树
1. 基于 byte，从而支持中文等各种语言的各种编码
2. 基本元素为 char，不需要用 T 来做泛型
3. 不考虑删除关键词，因此也不需要 prev 来管理孔洞
4. 不考虑多树合并、拆分，因此也不需要用 arena list 来管理
5. idx = 0 就是 根节点
6. 前一个版本的 next_idx 使用的是 HashMap<u8, usize>，在频繁索引时有一定的性能消耗，这个版本使用 [usize; 256]
 */

struct Node {
    next_idx: [usize; 256],
    has_child: bool,
    is_word: bool,
}


pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        let mut res = Self {
            nodes: Vec::new()
        };
        res.add_node();
        res
    }

    fn add_node(&mut self) -> usize {
        self.nodes.push(Node { next_idx: [0; 256], has_child: false, is_word: false });
        self.nodes.len() - 1
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr_idx: usize = 0;
        for chr in word.bytes() {
            if self.nodes[curr_idx].next_idx[chr as usize] == 0 {
                self.nodes[curr_idx].next_idx[chr as usize] = self.add_node();
                self.nodes[curr_idx].has_child = true;
            }
            curr_idx = self.nodes[curr_idx].next_idx[chr as usize]
        }
        self.nodes[curr_idx].is_word = true;
        return;
    }

    pub fn search(&self, text: &str) -> bool {
        let mut curr_idx = 0;
        let mut idx = 0;
        let text = text.as_bytes();
        while self.nodes[curr_idx].has_child && idx < text.len() {
            let chr = text[idx] as usize;
            if self.nodes[curr_idx].next_idx[chr] == 0 {
                return false;
            } else { curr_idx = self.nodes[curr_idx].next_idx[chr] }
            idx += 1;
        }

        if idx < text.len() {
            return false;
        }

        return self.nodes[curr_idx].is_word;
    }
}