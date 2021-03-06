use lazy_static::lazy_static;

use super::{week2, year2_u16, MatcherDef, MatcherSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gen1CpuKind {
    Dmg0,
    DmgA,
    DmgB,
    DmgC,
    DmgBlobB,
    DmgBlobC,
    Sgb,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gen1Cpu {
    pub kind: Gen1CpuKind,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen1_cpu;
/// assert!(parse_gen1_cpu("DMG-CPU LR35902 8907 D").is_some());
/// ```
fn dmg_cpu_lr35902() -> MatcherDef<Gen1Cpu> {
    MatcherDef(
        r#"^DMG-CPU\ LR35902\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Gen1Cpu {
                kind: Gen1CpuKind::Dmg0,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen1_cpu;
/// assert!(parse_gen1_cpu("DMG-CPU © 1989 Nintendo JAPAN 8913 D").is_some());
/// assert!(parse_gen1_cpu("DMG-CPU A © 1989 Nintendo JAPAN 8937 D").is_some());
/// assert!(parse_gen1_cpu("DMG-CPU B © 1989 Nintendo JAPAN 9207 D").is_some());
/// assert!(parse_gen1_cpu("DMG-CPU C © 1989 Nintendo JAPAN 9835 D").is_some());
/// ```
fn dmg_cpu() -> MatcherDef<Gen1Cpu> {
    MatcherDef(
        r#"^DMG-CPU(\ [ABC])?\ ©\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen1Cpu {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(Gen1CpuKind::DmgA),
                    Some(" B") => Ok(Gen1CpuKind::DmgB),
                    Some(" C") => Ok(Gen1CpuKind::DmgC),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(Gen1CpuKind::Dmg0),
                })?,
                year: Some(year2_u16(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

fn dmg_cpu_deprecated() -> MatcherDef<Gen1Cpu> {
    MatcherDef(
        r#"^DMG-CPU(\ [A-B])?\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen1Cpu {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(Gen1CpuKind::DmgA),
                    Some(" B") => Ok(Gen1CpuKind::DmgB),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(Gen1CpuKind::Dmg0),
                })?,
                year: Some(year2_u16(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen1_cpu;
/// assert!(parse_gen1_cpu("B").is_some());
/// assert!(parse_gen1_cpu("C").is_some());
/// ```
fn dmg_cpu_blob() -> MatcherDef<Gen1Cpu> {
    MatcherDef(r#"^[BC]$"#, move |c| {
        Ok(Gen1Cpu {
            kind: (match &c[0] {
                "B" => Ok(Gen1CpuKind::DmgBlobB),
                "C" => Ok(Gen1CpuKind::DmgBlobC),
                text => Err(format!("Invalid DMG-CPU part name: {}", text)),
            })?,
            year: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen1_cpu;
/// assert!(parse_gen1_cpu("SGB-CPU 01 © 1994 Nintendo Ⓜ 1989 Nintendo JAPAN 9434 7 D").is_some());
/// ```
fn sgb_cpu() -> MatcherDef<Gen1Cpu> {
    MatcherDef(
        r#"^SGB-CPU\ 01\ ©\ 1994\ Nintendo\ Ⓜ\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Gen1Cpu {
                kind: Gen1CpuKind::Sgb,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_gen1_cpu(text: &str) -> Option<Gen1Cpu> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Gen1Cpu> = MatcherSet::new(&[
            dmg_cpu(),
            dmg_cpu_blob(),
            dmg_cpu_lr35902(),
            dmg_cpu_deprecated(),
            sgb_cpu()
        ]);
    }
    MATCHER.apply(text)
}
