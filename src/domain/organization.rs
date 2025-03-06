use anyhow::{Error, Result};

pub struct Organization {
    short_name: String,
    long_name: String,
    bucket_name: String
}

impl TryFrom<&str> for Organization {

    type Error = Error;

    fn try_from(short_name: &str) -> Result<Self> {
        match short_name {
            "ads" => Ok(Organization {
                short_name: short_name.to_string(),
                long_name: "Association for Dressings & Sauces".to_string(),
                bucket_name: format!("{}_asset_bucket", short_name),
            }),
            "tmsu" => Ok(Organization {
                short_name: short_name.to_string(),
                long_name: "The Mad Scientists Union".to_string(),
                bucket_name: format!("{}_asset_bucket", short_name),
            }),
            "tucr" => Ok(Organization {
                short_name: short_name.to_string(),
                long_name: "The Union for Carolean Restoration".to_string(),
                bucket_name: format!("{}_asset_bucket", short_name),
            }),
            _ => Err(Error::msg(format!("Could not find org with short name: {}", short_name)))
        }
    }
}

