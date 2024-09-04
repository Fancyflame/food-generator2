use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::{
    parse_tokens::{ExprSection, ExprSectionBody, ExprSeg, SecInfo},
    searcher::{self, Trie},
};
use anyhow::{anyhow, Result};

use crate::share_str::ShareStr;

pub type LinkedSectionBody = Vec<Vec<LinkedSeg>>;

pub fn link_secs(sections: Vec<ExprSection>) -> Result<Rc<LinkedSection>> {
    SectionTable::parse(sections)?
        .table
        .remove("entry")
        .ok_or_else(|| anyhow!("a section named `entry` must be defined"))
}

pub enum LinkedSeg {
    Text(ShareStr),
    Use(Rc<LinkedSection>),
}

pub struct LinkedSection {
    pub rules: Vec<Vec<LinkedSeg>>,
    pub info: SecInfo,
    pub search: Rc<Trie>,
}

struct SectionTable {
    table: HashMap<ShareStr, Rc<LinkedSection>>,
}

impl SectionTable {
    pub fn parse(raw_sections: Vec<ExprSection>) -> Result<Self> {
        let mut this = SectionTable {
            table: HashMap::new(),
        };

        for ExprSection {
            rules,
            info: this_sec_info,
        } in raw_sections
        {
            if rules.is_empty() {
                return Err(anyhow!(
                    "section [{this_sec_info}] is empty, it must contains at least 1 rule"
                ));
            }
            let new_rules = this.link_segs(this_sec_info.clone(), rules)?;

            match this.table.entry(this_sec_info.name.clone()) {
                Entry::Occupied(occ) => {
                    let old_sec = &occ.get().info;
                    if this_sec_info.file != old_sec.file {
                        return Err(anyhow!(
                            "cannot re-define section [{this_sec_info}], \
                            because a namesake [{old_sec}] has been defined"
                        ));
                    }
                }

                Entry::Vacant(vac) => {
                    vac.insert(Rc::new(LinkedSection {
                        search: searcher::compile(&new_rules, &this_sec_info)?.into(),
                        rules: new_rules,
                        info: this_sec_info,
                    }));
                }
            }
        }

        Ok(this)
    }

    fn link_segs(&self, sec_info: SecInfo, rules: ExprSectionBody) -> Result<LinkedSectionBody> {
        let mut new_rules = Vec::new();

        for rule in rules {
            let mut new_rule = Vec::new();

            for seg in rule {
                let new_seg = match seg {
                    ExprSeg::Text(txt) => LinkedSeg::Text(txt),
                    ExprSeg::Use(r) => match self.table.get(&r) {
                        Some(rc) => LinkedSeg::Use(rc.clone()),
                        None => {
                            return Err(anyhow!(
                            "section `{r}` is not defined, but referenced by section [{sec_info}]"
                        ))
                        }
                    },
                };
                new_rule.push(new_seg);
            }

            new_rules.push(new_rule);
        }

        Ok(new_rules)
    }
}
