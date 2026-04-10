#[macro_export]
macro_rules! detail {
    // No metadata cases
    ($id:expr, $label:expr, String) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::String,
            type_metadata: None,
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };

    ($id:expr, $label:expr, StringArray) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::StringArray,
            type_metadata: Some(schema::DetailTypeMetadata::StringArrayMetadata(
                schema::StringArrayMetadata { length: None },
            )),
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };
    ($id:expr, $label:expr, StringArray, length = $length:expr) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::StringArray,
            type_metadata: Some(schema::DetailTypeMetadata::StringArrayMetadata(
                schema::StringArrayMetadata { length: $length },
            )),
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };

    ($id:expr, $label:expr, FormatedString, regex = $regex:expr) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::FormatedString,
            type_metadata: Some(schema::DetailTypeMetadata::FormatedStringMetadata(
                schema::FormatedStringMetadata {
                    regex: $regex.into(),
                },
            )),
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };
    ($id:expr, $label:expr, FormatedStringArray, regex = $regex:expr) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::FormatedStringArray,
            type_metadata: Some(schema::DetailTypeMetadata::FormatedStringArrayMetadata(
                schema::FormatedStringArrayMetadata {
                    length: None,
                    regex: $regex.into(),
                },
            )),
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };
    ($id:expr, $label:expr, FormatedStringArray, length = $length:expr, regex = $regex:expr) => {
        schema::DetailsJSON {
            id: $id.into(),
            data_type: schema::DataTypes::FormatedStringArray,
            type_metadata: Some(schema::DetailTypeMetadata::FormatedStringArrayMetadata(
                schema::FormatedStringArrayMetadata {
                    length: Some($length),
                    regex: $regex.into(),
                },
            )),
            confidence: None,
            interactions: None,
            label: Some($label.into()),
            data: None,
        }
    };
}
