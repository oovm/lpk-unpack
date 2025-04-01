#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum MocVersion {
    /// `moc 0x6`, initial version,
    V2_6_INTIAL,
    /// `moc 0x6`, opacity support
    V2_7_OPACITY,
    /// `moc 0x8`, texture option support,
    V2_8_TEX_OPTION,
    /// `moc 0x9`, Avatar parts support,
    V2_9_AVATAR_PARTS,
    /// `moc 0xA`, SDK 2.0,
    V2_10_SDK2,
    /// `moc 0xB`, SDK 2.1,
    V2_11_SDK2_1,
    /// `moc3 0x1`
    V30,
    /// `moc3 0x2`
    V33,
    /// `moc3 0x3`
    V40,
    /// `moc3 0x4`
    V42,
    /// `moc3 0x5`
    V50,
}
