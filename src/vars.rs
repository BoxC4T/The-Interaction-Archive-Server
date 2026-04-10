use once_cell::sync::Lazy;
use tia_server::*;

use crate::schema;
//marked - marked for removal
pub const VALID_CONNECTION_STATUS: [&str; 3] = ["active", "archived", "marked"];

pub static BUILTIN_DETAILS: Lazy<Vec<schema::DetailsJSON>> = Lazy::new(|| {
    vec![
        detail!("bFirstName", "Primary First Name", String),
        detail!("bOtherFirstNames", "Secondary First Names", StringArray),
        detail!("bMiddleName", "Primary Middle Name", String),
        detail!("bOtherMiddleNames", "Secondary Middle", StringArray),
        detail!("bLastName", "Primary Last Name", String),
        detail!("bOtherLastNames", "Secondary Last Names", StringArray),
        detail!("bDisplayName", "Primary Display Name", String),
        detail!("bOtherDisplayNames", "Secondary Display Names", StringArray),
        detail!(
            "bBirthday",
            "Birthday (YYYY-MM-DD)",
            FormatedString,
            regex = r"^(19|20)\d\d([- /.])(0[1-9]|1[012])\2(0[1-9]|[12][0-9]|3[01])$"
        ),
    ]
});
