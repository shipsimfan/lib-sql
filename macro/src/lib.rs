//! Macros for generating structs to interact with sql databases

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use proc_macro_util::proc_macro_attribute;

mod table;

proc_macro_attribute!(
    /// Generates structures for accessing a table in a database from a `struct`
    ///
    /// This macro will generate up to three structures:
    ///  - A structure with the same name as the provided structure for selecting and deleting
    ///  - A structure prefixed with `New` for inserting
    ///  - A structure suffixed with `Update` for updating
    ///
    /// The selecting struct will have at least the following four functions, if they are not
    /// eliminated by flags:
    ///  * `select_all` - Allows select many rows from the table, returning the results as a `Vec`
    ///                   of the selecting struct
    ///  * `select` - Allows selecting one row based on the primary key, returning an `Option` of
    ///               the selecting struct
    ///  * `exists` - Allows checking if a row with the provided primary key already exists in the
    ///               table
    ///  * `delete` - Deletes a row from the table based on the primary key
    ///
    /// The inserting struct will have one function:
    ///  * `insert` - Inserts a row into the table based on the contents of the struct
    ///
    /// The updating struct will have one function:
    ///  * `update` - Updates the row with the provided primary key based on the contents of the
    ///               struct
    ///
    /// Each field of the input structure will correspond to one field in the sql table. Each table
    /// can have zero or more of the following attributes attached to them to effect how the field
    /// is treated in the generated structures and functions:
    ///  * #[primary_key] - Indicates the field is a primary key field. It will be required for
    ///                     single selects, updates, and deletes. If multiple primary keys are
    ///                     specified, additional functions will be generated to allowing selected
    ///                     all rows with only one of the primary keys specified. The field will be
    ///                     excluded from the updating struct. The field will be verified for
    ///                     uniqueness before insertion.
    ///  * #[auto_increment] - Indicates the fields value will auto-increment. This can only be on
    ///                        a primary key field. The field will be excluded from the inserting
    ///                        struct.
    ///  * #[unique] - Indicates the field must be unqiue. Before inserts or updates, unqiueness
    ///                will be verified.
    ///  * #[references(table.field)] - Indicates this field can be inner joined to another table.
    ///                                 This will auto-generate a selecting struct for the
    ///                                 resulting inner join.
    ///  * #[min = ...], #[max = ...] - Validate either the value or length, based on data type, of
    ///                                 a value before inserting or updating.
    ///  * #[default] - Uses a default value as defined in the database for inserting if one isn't
    ///                 provided
    ///  * #[default = ...] - Sets a default value for inserting if one isn't provided
    ///
    /// The following flags can be provided as a comma seperated list to #[sql_table()]:
    ///  - `not_deletable` - Prevents the generation of the delete function on the selecting struct
    ///  - `not_selectable` - Prevents the generation of the selecting struct and select functions.
    ///                       If specified without `not_deletable`, the selecting struct will be
    ///                       generated without fields and without select functions, but will have
    ///                       a `delete` function.
    ///  - `not_insertable` - Prevents generating the inserting struct and insert functions
    ///  - `not_updatable` - Prevents generating the updating struct and update function
    sql_table -> table::table
);
