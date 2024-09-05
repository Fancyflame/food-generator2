use std::collections::{hash_map::Entry, HashMap};

use crate::{
    share_str::ShareStr,
    syntax::{Layer, Section, Seg},
};

use super::{
    link::{LinkedSection, LinkedSeg},
    searcher::Trie,
    SerializeMap,
};

pub fn serialize(root_section: &LinkedSection) -> SerializeMap {
    assert_eq!(root_section.info.name.as_str(), "entry");
    let mut map = HashMap::new();
    let mut vec = Vec::new();
    serailize_sec(&mut map, &mut vec, root_section);
    vec.into_iter().map(Option::unwrap).collect()
}

fn serailize_sec(
    name2index: &mut HashMap<ShareStr, u32>,
    vec: &mut Vec<Option<Section>>,
    sec: &LinkedSection,
) -> u32 {
    let insert_index = match name2index.entry(sec.info.name.clone()) {
        Entry::Occupied(occ) => {
            return *occ.get();
        }
        Entry::Vacant(vac) => {
            assert!(u32::MAX as usize > vec.len());
            let i = *vac.insert(vec.len() as _);
            vec.push(None);
            i
        }
    };

    let rules: Vec<Vec<Seg>> = sec
        .rules
        .iter()
        .map(|rule| {
            rule.iter()
                .map(|seg| match seg {
                    LinkedSeg::Text(t) => Seg::Text(t.clone()),
                    LinkedSeg::Use(sec) => {
                        let index = match name2index.get(&sec.info.name) {
                            Some(&index) => index,
                            None => serailize_sec(name2index, vec, sec),
                        };
                        Seg::Use(index)
                    }
                })
                .collect()
        })
        .collect();

    vec[insert_index as usize] = Some(Section {
        encoder: rules,
        decoder: serialize_trie(&sec.search),
    });
    insert_index
}

fn serialize_trie(trie: &Trie) -> Layer {
    match trie {
        Trie::Branch(b) => {
            let mut map = HashMap::new();
            for (&key, content) in b {
                map.insert(key, serialize_trie(&content));
            }
            Layer::Branch(map)
        }
        &Trie::Leaf { value, .. } => Layer::Certain(value),
    }
}
