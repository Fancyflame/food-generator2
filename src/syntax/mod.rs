use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    path::Path,
    rc::Rc,
};

use anyhow::{anyhow, Result};
use parse::{RawSection, RawSectionBody, RawSeg, SyntaxParser};

mod parse;

pub type SectionBody = Vec<Vec<Segmentation>>;

struct SectionTable {
    table: HashMap<String, Rc<Section>>,
}

pub enum Segmentation {
    Text(String),
    Use(Rc<Section>),
}

pub struct Section {
    pub rules: Vec<Vec<Segmentation>>,
    pub str2index: IndexTable,
    pub file: Rc<Path>,
}

pub fn parse(base_dir: impl AsRef<Path>) -> Result<Rc<Section>> {
    SectionTable::parse(base_dir)?
        .table
        .remove("entry")
        .ok_or_else(|| anyhow!("a section named `entry` must be defined"))
}

impl SectionTable {
    pub fn parse(base_dir: impl AsRef<Path>) -> Result<Self> {
        let mut this = SectionTable {
            table: HashMap::new(),
        };

        let raw_sections = SyntaxParser::parse(base_dir)?.sections;
        for RawSection { name, rules, file } in raw_sections {
            let this_sec_info = SecInfo::new(&name, &file);

            if rules.is_empty() {
                return Err(anyhow!(
                    "section [{this_sec_info}] is empty, it must contains at least 1 rule"
                ));
            }
            let (new_rules, index_table) = this.link_segs(this_sec_info, rules)?;

            match this.table.entry(name) {
                Entry::Occupied(occ) => {
                    let this_sec = SecInfo::new(occ.key(), &file);
                    let old_sec = SecInfo::new(occ.key(), &occ.get().file);
                    if this_sec.file != old_sec.file {
                        return Err(anyhow!(
                            "cannot re-define section [{this_sec}], because a namesake [{old_sec}] has been defined"
                        ));
                    }
                }

                Entry::Vacant(vac) => {
                    vac.insert(Rc::new(Section {
                        rules: new_rules,
                        str2index: index_table,
                        file,
                    }));
                }
            }
        }

        Ok(this)
    }

    fn link_segs(
        &self,
        sec_info: SecInfo,
        rules: RawSectionBody,
    ) -> Result<(SectionBody, IndexTable)> {
        let mut new_rules = Vec::new();
        let mut index_table = IndexTable::default();

        for rule in rules {
            let mut new_rule = Vec::new();

            for seg in rule {
                let new_seg = match seg {
                    RawSeg::Text(txt) => Segmentation::Text(txt),
                    RawSeg::Use(r) => match self.table.get(&r) {
                        Some(rc) => Segmentation::Use(rc.clone()),
                        None => {
                            return Err(anyhow!(
                            "section `{r}` is not defined, but referenced by section [{sec_info}]"
                        ))
                        }
                    },
                };
                new_rule.push(new_seg);
            }

            index_table.insert(sec_info, &new_rule, new_rules.len())?;
            new_rules.push(new_rule);
        }

        Ok((new_rules, index_table))
    }
}

#[derive(Default)]
pub struct IndexTable {
    used_id: HashMap<String, usize>,
}

impl IndexTable {
    pub fn try_match(&self, s: &str) -> Option<usize> {
        let mut ci = s.char_indices();
        let mut result = None;

        for (index, ch) in &mut ci {
            let up_idx = index + ch.len_utf8();
            if let Some(&idx) = self.used_id.get(&s[..up_idx]) {
                result = Some(idx);
                break;
            };
        }

        result
    }

    fn insert<'a>(
        &'a mut self,
        sec_info: SecInfo,
        rule: &[Segmentation],
        insert_idx: usize,
    ) -> Result<()> {
        match &rule[0] {
            Segmentation::Text(txt) => {
                for s in self.used_id.keys() {
                    if s.starts_with(txt) || txt.starts_with(s) {
                        return Err(anyhow!(
                            "there is a rule id `{s}` that conflicts with another rule id `{txt}` in section [{sec_info}], \
                            which will cause uncertain decode results"
                        ));
                    }
                }
                self.used_id.insert(txt.clone(), insert_idx);
            }

            Segmentation::Use(r) => {
                for nested_rule in r.rules.iter() {
                    self.insert(sec_info, &nested_rule, insert_idx)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct SecInfo<'a> {
    name: &'a str,
    file: &'a Path,
}

impl<'a> SecInfo<'a> {
    fn new(name: &'a str, file: &'a Path) -> Self {
        SecInfo { name, file }
    }
}

impl Display for SecInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` in file `{}`", self.name, self.file.display())
    }
}
