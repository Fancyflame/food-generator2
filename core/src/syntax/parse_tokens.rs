use std::{fmt::Display, fs::read_to_string, path::Path, rc::Rc};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{multispace0, multispace1},
    combinator::{cut, eof, flat_map, map, opt, recognize, value, verify},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use unicode_ident::is_xid_continue;

use crate::share_str::ShareStr;

pub(super) type ExprSectionBody = Vec<Vec<ExprSeg>>;

pub fn parse(base_dir: impl AsRef<Path>) -> Result<Vec<ExprSection>> {
    SyntaxParser::parse(base_dir).map(|x| x.sections)
}

#[derive(Debug)]
pub(super) struct ExprSection {
    pub rules: ExprSectionBody,
    pub info: SecInfo,
}

#[derive(Debug)]
pub enum ExprSeg {
    Text(ShareStr),
    Use(ShareStr),
}

#[derive(Debug)]
struct SyntaxParser {
    sections: Vec<ExprSection>,
}

#[derive(Clone, Copy)]
enum SectionHeader<'a> {
    Inline(&'a str),
    File(&'a str),
}

impl SyntaxParser {
    fn parse(base_dir: impl AsRef<Path>) -> Result<Self> {
        let mut this = SyntaxParser {
            sections: Vec::new(),
        };

        this.read_file(&base_dir.as_ref().join("entry.txt"))?;
        Ok(this)
    }

    #[inline]
    fn read_file(&mut self, file_path: &Path) -> Result<()> {
        self.parse_file(file_path, &read_to_string(file_path)?)
    }

    fn parse_file(&mut self, file_path: &Path, content: &str) -> Result<()> {
        let base_dir = file_path.parent().unwrap_or(Path::new(""));
        let file_path: Rc<Path> = file_path.to_owned().into_boxed_path().into();

        let (_, clean_code) =
            remove_comments(&content).map_err(|err| anyhow!("cannot remove comments: {err}"))?;
        let clean_code = ShareStr::new(&clean_code);

        let mut no_comments_parser = terminated(
            many0(flat_map(section_header, |header: SectionHeader| {
                let this = &*self;
                let cc = &clean_code;
                map(
                    move |s| this.parse_section(cc, s, header),
                    move |body| (header, body),
                )
            })),
            eof,
        );

        let (_, sections): (_, Vec<(SectionHeader<'_>, Option<ExprSectionBody>)>) =
            no_comments_parser(&clean_code).map_err(|err| anyhow!("parse error: {err}"))?;

        drop(no_comments_parser);

        for (header, section_body) in sections {
            match header {
                SectionHeader::File(path) => self.read_file(&base_dir.join(path))?,
                SectionHeader::Inline(name) => {
                    self.sections.push(ExprSection {
                        info: SecInfo {
                            name: clean_code.recognize(name).unwrap(),
                            file: file_path.clone(),
                        },
                        rules: section_body.unwrap(),
                    });
                }
            }
        }

        Ok(())
    }

    fn parse_section<'a>(
        &self,
        origin: &ShareStr,
        s: &'a str,
        header: SectionHeader,
    ) -> IResult<&'a str, Option<ExprSectionBody>> {
        match header {
            SectionHeader::File(_) => Ok((s, None)),
            SectionHeader::Inline(_) => {
                let match_seg = alt((
                    delimited(
                        tag("{"),
                        map(cut(section_name), |id| {
                            ExprSeg::Use(origin.recognize(id).unwrap())
                        }),
                        cut(tag("}")),
                    ),
                    map(is_not("{[\r\n"), |txt: &str| {
                        ExprSeg::Text(origin.recognize(txt).unwrap())
                    }),
                ));

                map(
                    many0(terminated(
                        map(many1(match_seg), |mut vec| {
                            if let Some(ExprSeg::Text(first)) = vec.first_mut() {
                                *first = string_trim_start(first);
                            }
                            if let Some(ExprSeg::Text(last)) = vec.last_mut() {
                                *last = string_trim_end(last);
                            }
                            vec
                        }),
                        end_spaces,
                    )),
                    Some,
                )(s)
            }
        }
    }
}

fn section_name(s: &str) -> IResult<&str, &str> {
    recognize(take_while1(is_xid_continue))(s)
}

fn string_expr(s: &str) -> IResult<&str, &str> {
    delimited(tag("\""), is_not("\"\r\n"), tag("\""))(s)
}

fn section_header(s: &str) -> IResult<&str, SectionHeader> {
    delimited(
        pair(multispace0, tag("[")),
        alt((
            map(
                preceded(pair(tag("include"), multispace1), string_expr),
                SectionHeader::File,
            ),
            map(
                take_while1(|ch: char| !ch.is_whitespace() && ch != ']'),
                SectionHeader::Inline,
            ),
        )),
        pair(tag("]"), end_spaces),
    )(s)
}

fn remove_comments(s: &str) -> IResult<&str, String> {
    map(
        terminated(
            many0(alt((
                recognize(string_expr),
                is_not("#\""),
                value("", pair(tag("#"), opt(is_not("\r\n")))),
            ))),
            eof,
        ),
        |vec| vec.join(""),
    )(s)
}

fn end_spaces(s: &str) -> IResult<&str, ()> {
    alt((
        value(
            (),
            verify(multispace1, |spaces: &str| spaces.contains('\n')),
        ),
        value((), pair(multispace0, eof)),
    ))(s)
}

fn string_trim_start(s: &ShareStr) -> ShareStr {
    let skip_len = s.len() - s.trim_start().len();
    s.clone_range(skip_len..)
}

fn string_trim_end(s: &ShareStr) -> ShareStr {
    let reserve_len = s.trim_end().len();
    s.clone_range(..reserve_len)
}

#[derive(Clone, Debug)]
pub struct SecInfo {
    pub name: ShareStr,
    pub file: Rc<Path>,
}

impl Display for SecInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` in file `{}`", self.name, self.file.display())
    }
}
