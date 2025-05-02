mod parse;

/// Flags influencing how to generate the table
pub struct TableFlags {
    /// This table has been indicated to be not deletable
    pub not_deletable: bool,

    /// This table has been indicated to be not insertable
    pub not_insertable: bool,

    /// This table has been indicated to be not updatable
    pub not_updatable: bool,

    /// This table has been indicated to be not selectable
    pub not_selectable: bool,
}
