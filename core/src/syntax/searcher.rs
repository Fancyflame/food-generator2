use std::{
    collections::{
        hash_map::{Entry, VacantEntry},
        HashMap,
    },
    rc::Rc,
};

use anyhow::{anyhow, Result};

use crate::share_str::ShareStr;

use super::{link::LinkedSeg, parse_tokens::SecInfo};

type Table = HashMap<char, Rc<Trie>>;

#[derive(Clone)]
pub enum SearchSeg {
    Text(ShareStr),
    Use(Rc<Trie>),
}

#[derive(Clone)]
pub enum Trie {
    Branch(Table),
    Leaf {
        value: u32,
        rest_nodes: Vec<SearchSeg>, // reversed
    },
}

impl Trie {
    fn new() -> Self {
        Trie::Branch(HashMap::new())
    }

    fn insert(&mut self, expect: char) -> Result<InsertResult> {
        let table = match self {
            Self::Branch(b) => b,
            Self::Leaf { .. } => self.expand_to_table()?,
        };

        let r = match table.entry(expect) {
            Entry::Occupied(occ) => InsertResult::Occupied(Rc::get_mut(occ.into_mut()).unwrap()),
            Entry::Vacant(vac) => InsertResult::Vacant(vac),
        };

        Ok(r)
    }

    fn expand_to_table(&mut self) -> Result<&mut Table> {
        let this = std::mem::replace(self, Self::new());
        let (
            Self::Leaf {
                value,
                rest_nodes: mut org_rest_nodes,
            },
            Self::Branch(new_table),
        ) = (this, self)
        else {
            panic!("self must be a leaf");
        };

        loop {
            match org_rest_nodes.pop() {
                Some(SearchSeg::Text(txt)) => {
                    let Some((org_ch, rest)) = split_first(&txt) else {
                        continue;
                    };

                    org_rest_nodes.push(SearchSeg::Text(rest));
                    new_table.insert(
                        org_ch,
                        Trie::Leaf {
                            value,
                            rest_nodes: org_rest_nodes,
                        }
                        .into(),
                    );
                    break;
                }
                Some(SearchSeg::Use(trie)) => match &*trie {
                    Trie::Branch(b) => {
                        for (&key, use_trie) in b.iter() {
                            let mut rn = org_rest_nodes.clone();
                            rn.push(SearchSeg::Use(use_trie.clone()));
                            new_table.insert(
                                key,
                                Trie::Leaf {
                                    value,
                                    rest_nodes: rn,
                                }
                                .into(),
                            );
                        }
                        break;
                    }
                    Trie::Leaf { rest_nodes, .. } => {
                        for seg in rest_nodes.iter() {
                            org_rest_nodes.push(seg.clone());
                        }
                    }
                },
                None => {
                    return Err(anyhow!("cannot expand this node as it has reached the end"));
                }
            }
        }

        Ok(new_table)
    }
}

enum InsertResult<'a> {
    Occupied(&'a mut Trie),
    Vacant(VacantEntry<'a, char, Rc<Trie>>),
}

pub fn compile(rules: &Vec<Vec<LinkedSeg>>, info: &SecInfo) -> Result<Trie> {
    let mut trie = Trie::new();
    for (rule, value) in rules.iter().zip(0u32..) {
        let mut buffer = Vec::new();
        for seg in rule.iter().rev() {
            match seg {
                LinkedSeg::Text(t) => buffer.push(SearchSeg::Text(t.clone())),
                LinkedSeg::Use(u) => buffer.push(SearchSeg::Use(u.search.clone())),
            }
        }

        compile_rule(&mut trie, buffer, value)
            .map_err(|err| anyhow!("section [{info}] decode tree build failed: {err}"))?;
    }
    Ok(trie)
}

fn compile_rule(mut trie: &mut Trie, mut buffer: Vec<SearchSeg>, value: u32) -> Result<()> {
    while let Some(seg) = buffer.pop() {
        match seg {
            SearchSeg::Text(txt) => {
                let mut chars = txt.chars();
                for ch in &mut chars {
                    match trie.insert(ch)? {
                        InsertResult::Occupied(occ) => trie = occ,
                        InsertResult::Vacant(vac) => {
                            buffer.push(SearchSeg::Text(txt.recognize(chars.as_str()).unwrap()));
                            vac.insert(
                                Trie::Leaf {
                                    value,
                                    rest_nodes: Vec::from_iter(buffer.drain(..)),
                                }
                                .into(),
                            );
                            return Ok(());
                        }
                    }
                }
            }
            SearchSeg::Use(u) => match &*u {
                Trie::Leaf { rest_nodes, .. } => {
                    buffer.extend(rest_nodes.iter().cloned().rev());
                    continue;
                }
                Trie::Branch(b) => {
                    for (&key, content) in b.iter() {
                        let mut buffer2 = buffer.clone();
                        buffer2.push(SearchSeg::Use(content.clone()));
                        match trie.insert(key)? {
                            InsertResult::Occupied(occ) => compile_rule(occ, buffer2, value)?,
                            InsertResult::Vacant(vac) => {
                                vac.insert(
                                    Trie::Leaf {
                                        value,
                                        rest_nodes: buffer2,
                                    }
                                    .into(),
                                );
                            }
                        }
                    }
                    return Ok(());
                }
            },
        }
    }

    Err(anyhow!("this rule is contained by other rule"))
}

fn split_first(s: &ShareStr) -> Option<(char, ShareStr)> {
    let ch = s.chars().next()?;
    let rest = s.clone_range(ch.len_utf8()..);
    Some((ch, rest))
}
