use std::collections::BTreeMap;

use serde::{ser::SerializeMap, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LockFile {
    pub sdks: BTreeMap<String, String>,
    pub packages: BTreeMap<String, Package>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DependencyKind {
    #[serde(rename = "direct main")]
    DirectMain,
    #[serde(rename = "direct dev")]
    DirectDev,
    #[serde(rename = "transitive")]
    Transitive,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageSource {
    Sdk,
    Hosted,
}

#[derive(Debug)]
pub enum Description {
    Flutter,
    Online { name: String, url: String },
}

impl<'de> Deserialize<'de> for Description {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TheVisitor;

        impl<'de> serde::de::Visitor<'de> for TheVisitor {
            type Value = Description;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "string or map")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "flutter" {
                    Ok(Description::Flutter)
                } else {
                    Err(E::custom(format!("Invalid string {:?}", v)))
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut url = None;
                let mut name = None;

                while let Some((key, value)) = map.next_entry()? {
                    match key {
                        "name" => name = Some(value),
                        "url" => url = Some(value),
                        _ => { /* ignore */ }
                    }
                }
                let name = name.ok_or_else(|| {
                    <A::Error as serde::de::Error>::custom("missing field 'name'")
                })?;
                let url = url
                    .ok_or_else(|| <A::Error as serde::de::Error>::custom("missing field 'url'"))?;
                Ok(Description::Online { name, url })
            }
        }

        deserializer.deserialize_any(TheVisitor)
    }
}

impl Serialize for Description {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Description::Flutter => serializer.serialize_str("flutter"),
            Description::Online { name, url } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("name", name)?;
                map.serialize_entry("url", url)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub dependency: DependencyKind,
    pub source: PackageSource,
    pub version: String,
    pub description: Description,
}
