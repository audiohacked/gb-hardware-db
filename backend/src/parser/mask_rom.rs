use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_code: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// Sharp ROM chip (1990+)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-WJA-0 S LH534M05 JAPAN E1 9606 D").is_some());
/// assert!(parse_mask_rom("DMG-AP2J-0 S LH534MVD JAPAN E1 9639 D").is_some());
/// assert!(parse_mask_rom("DMG-HFAJ-0 S LHMN4MTI JAPAN E 9838 E").is_some());
/// ```
fn sharp() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ S\ (LH[[:alnum:]]{4})[[:alnum:]]{2} \ JAPAN\ [A-Z][0-9]?\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(map_sharp_mask_rom(&c[3]).unwrap_or(&c[3]).to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Old sharp ROM chip with no chip type (1989 - 1991)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-TRA-1 SHARP JAPAN A0 9019 D").is_some());
/// ```
fn sharp2() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^(DMG-[[:alnum:]]{3}-[0-9])\ SHARP\ JAPAN\ [A-Z][0-9]?\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Very old Sharp mask ROM chip (1989 and older)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-AWA-0 SHARP JAPAN 8909 D A").is_some());
/// ```
fn sharp3() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^(DMG-[[:alnum:]]{3}-[0-9])\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Macronix MX23C mask ROM chip (1999+)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("M003119-M MX23C1603-12A DMG-VPHP-0 G2 2C882503").is_some());
/// assert!(parse_mask_rom("E013104-M MX23C1603-12A CGB-BFPU-0 G2 1D2907A1B1").is_some());
/// assert!(parse_mask_rom("T991349-M MX23C8006-12 DMG-VPHJ-0 F 1A4891A2").is_some());
/// assert!(parse_mask_rom("M004523-M MX23C3203-11A2 CGB-B82J-0 02 H2 2D224301").is_some());
/// ```
fn macronix() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ([0-9]\ )? ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9][0-9]\ )? [A-Z][0-9]?\ [[:alnum:]]{8,10}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[5].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Macronix MX23C mask ROM chip (pre-1999)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("C9745-M MX23C4002-20 DMG-APOJ-0 E1 43824C").is_some());
/// ```
fn macronix2() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^[A-Z]([0-9]{2})([0-9]{2})-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]?\ [[:alnum:]]{6}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[4].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// OKI Semiconductor MSM538011E mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-AM6J-0 F1 M538011E-36 9085401").is_some());
/// ```
fn oki_msm538011e() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (M538011E)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("MS{}", &c[3])),
                year: Some(year1(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// OKI Semiconductor MR531614G mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("CGB-BPTE-0 G2 R531614G-44 044232E").is_some());
/// ```
fn oki_mr531614g() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (R531614G)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("M{}", &c[3])),
                year: Some(year1(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// NEC mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("NEC JAPAN DMG-SAJ-0 C1 UPD23C1001EGW-J01 9010E9702").is_some());
/// ```
fn nec() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^NEC\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (UPD23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Nec),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Unknown mask ROM with NEC-like labeling
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-ZLE-0 E1 N-4001EAGW-J14 9329X7007").is_some());
/// ```
fn nec_like() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (N-[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// AT&T mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("Ⓜ AT&T JAPAN DMG-Q6E-0 C1 23C1001EAGW-K37 9351E9005").is_some());
/// ```
fn at_t() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^Ⓜ\ AT&T\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::AtT),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Standard Microsystems mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("STANDARD MICRO DMG-BIA-0 C1 23C1001EGW-J61 9140E9017").is_some());
/// ```
fn smsc() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^STANDARD\ MICRO\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Smsc),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Glop top mask ROM.
///
/// Probably manufactured by Sharp (?)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("LR0G150 DMG-TRA-1 97141").is_some());
/// ```
fn glop_top() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^(LR0G150)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9]{2})([0-9]{2})[0-9]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: None,
                chip_type: Some(c[1].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Toshiba mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("TOSHIBA 9136EAI TC531001CF DMG-NCE-0 C1 J541 JAPAN").is_some());
/// ```
fn toshiba() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^TOSHIBA\ ([0-9]{2})([0-9]{2})EAI\ (TC53[0-9]{4}[A-Z]{2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [A-Z][0-9]{3}\ JAPAN$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[4].to_owned(),
                manufacturer: Some(Manufacturer::Toshiba),
                chip_type: (Some(c[3].to_owned())),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Samsung mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("SEC KM23C16120DT CGB-BHMJ-0 G2 K3N5C317GD").is_some());
/// ```
fn samsung() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [[:alnum:]]{10}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                year: None,
                week: None,
            })
        },
    )
}

/// Old samsung mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("SEC KM23C8000DG DMG-AAUJ-1 F1 KFX331U").is_some());
/// ```
fn samsung2() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ KF[[:alnum:]]{4}[A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                year: None,
                week: None,
            })
        },
    )
}

/// Fujitsu Mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("JAPAN DMG-GKX-0 D1 1P0 AK 9328 R09").is_some());
/// assert!(parse_mask_rom("JAPAN DMG-WJA-0 E1 3NH AK 9401 R17").is_some());
/// ```
fn fujitsu() -> MatcherDef<MaskRom> {
    MatcherDef(
        r#"^JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [0-9][A-Z][[:alnum:]]\ [A-Z]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Fujitsu),
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

fn map_sharp_mask_rom(code: &str) -> Option<&'static str> {
    match code {
        "LH5359" => Some("LH53259"),   // Sharp Memory Data Book 1992
        "LH5317" => Some("LH53517"),   // Unknown mask ROM listing scan
        "LH531H" => Some("LH530800A"), // Sharp Memory Data Book 1992
        // reasonable guesses
        "LH5308" => Some("LH530800"), // unknown 1Mb JEDEC, compatible with LH530800A
        "LH5314" => Some("LH53514"),  // unknown 512Kb JEDEC, compatible with LH53517
        "LH5321" => Some("LH532100"), // unknown 2Mb JEDEC
        // unknown 2Mb JEDEC
        // maybe: LH532100 series / LH532300 / LH532700 series
        "LH532D" => None,
        "LH532M" => None,
        "LH532W" => None,
        "LHMN2E" => None,
        // Unknown 4Mb JEDEC
        // maybe: LH534100 series / LH534300 series / LH534R00
        "LH534M" => None,
        "LH5S4M" => None,
        "LHMN4M" => None,
        // Unknown 8Mb JEDEC
        // maybe: LH538300 series / LH538400 series / LH538700 / LH538R00 series
        "LH538M" => None,
        "LH538W" => None,
        "LH5S8M" => None,
        "LHMN8J" => None,
        "LHMN8M" => None,
        // Unknown 16 Mb
        // maybe: LH5316400 / LH5316500 series / LH5316P00 series
        "LH537M" => None,
        _ => None,
    }
}

pub fn parse_mask_rom(text: &str) -> Option<MaskRom> {
    lazy_static! {
        static ref MATCHER: MatcherSet<MaskRom> = MatcherSet::new(&[
            sharp(),
            sharp2(),
            sharp3(),
            macronix(),
            macronix2(),
            oki_msm538011e(),
            oki_mr531614g(),
            nec(),
            nec_like(),
            at_t(),
            smsc(),
            glop_top(),
            toshiba(),
            samsung(),
            samsung2(),
            fujitsu(),
        ]);
    }
    MATCHER.apply(text)
}
