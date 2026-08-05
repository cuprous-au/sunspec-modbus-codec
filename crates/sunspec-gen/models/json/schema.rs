#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`Group`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"comments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"count\": {"]
#[doc = "      \"default\": 1,"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"string\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"desc\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"detail\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"groups\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/group\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"points\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/point\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"group\","]
#[doc = "        \"sync\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Group {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub comments: ::std::vec::Vec<::std::string::String>,
    #[serde(default = "defaults::group_count")]
    pub count: GroupCount,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub desc: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<Group>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub points: ::std::vec::Vec<Point>,
    #[serde(rename = "type")]
    pub type_: GroupType,
}
impl Group {
    pub fn builder() -> builder::Group {
        Default::default()
    }
}
#[doc = "`GroupCount`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": 1,"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GroupCount {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::default::Default for GroupCount {
    fn default() -> Self {
        GroupCount::Integer(1_i64)
    }
}
impl ::std::fmt::Display for GroupCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for GroupCount {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`GroupType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"group\","]
#[doc = "    \"sync\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GroupType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "sync")]
    Sync,
}
impl ::std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Group => f.write_str("group"),
            Self::Sync => f.write_str("sync"),
        }
    }
}
impl ::std::str::FromStr for GroupType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group" => Ok(Self::Group),
            "sync" => Ok(Self::Sync),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GroupType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Point`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"size\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"access\": {"]
#[doc = "      \"default\": \"R\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"R\","]
#[doc = "        \"RW\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"comments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"count\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"desc\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"detail\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mandatory\": {"]
#[doc = "      \"default\": \"O\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"M\","]
#[doc = "        \"O\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"sf\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"string\""]
#[doc = "      ],"]
#[doc = "      \"maximum\": 10.0,"]
#[doc = "      \"minimum\": -10.0"]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"standards\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"static\": {"]
#[doc = "      \"default\": \"D\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"D\","]
#[doc = "        \"S\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"symbols\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/symbol\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"int16\","]
#[doc = "        \"int32\","]
#[doc = "        \"int64\","]
#[doc = "        \"raw16\","]
#[doc = "        \"uint16\","]
#[doc = "        \"uint32\","]
#[doc = "        \"uint64\","]
#[doc = "        \"acc16\","]
#[doc = "        \"acc32\","]
#[doc = "        \"acc64\","]
#[doc = "        \"bitfield16\","]
#[doc = "        \"bitfield32\","]
#[doc = "        \"bitfield64\","]
#[doc = "        \"enum16\","]
#[doc = "        \"enum32\","]
#[doc = "        \"float32\","]
#[doc = "        \"float64\","]
#[doc = "        \"string\","]
#[doc = "        \"pad\","]
#[doc = "        \"ipaddr\","]
#[doc = "        \"ipv6addr\","]
#[doc = "        \"eui48\","]
#[doc = "        \"sunssf\","]
#[doc = "        \"count\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"string\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Point {
    #[serde(default = "defaults::point_access")]
    pub access: PointAccess,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub comments: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub desc: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default = "defaults::point_mandatory")]
    pub mandatory: PointMandatory,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sf: ::std::option::Option<PointSf>,
    pub size: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub standards: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "static", default = "defaults::point_static")]
    pub static_: PointStatic,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub symbols: ::std::vec::Vec<Symbol>,
    #[serde(rename = "type")]
    pub type_: PointType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<PointValue>,
}
impl Point {
    pub fn builder() -> builder::Point {
        Default::default()
    }
}
#[doc = "`PointAccess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"R\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"R\","]
#[doc = "    \"RW\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PointAccess {
    R,
    #[serde(rename = "RW")]
    Rw,
}
impl ::std::fmt::Display for PointAccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::R => f.write_str("R"),
            Self::Rw => f.write_str("RW"),
        }
    }
}
impl ::std::str::FromStr for PointAccess {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "R" => Ok(Self::R),
            "RW" => Ok(Self::Rw),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PointAccess {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PointAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PointAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PointAccess {
    fn default() -> Self {
        PointAccess::R
    }
}
#[doc = "`PointMandatory`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"O\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"M\","]
#[doc = "    \"O\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PointMandatory {
    M,
    O,
}
impl ::std::fmt::Display for PointMandatory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::M => f.write_str("M"),
            Self::O => f.write_str("O"),
        }
    }
}
impl ::std::str::FromStr for PointMandatory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "M" => Ok(Self::M),
            "O" => Ok(Self::O),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PointMandatory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PointMandatory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PointMandatory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PointMandatory {
    fn default() -> Self {
        PointMandatory::O
    }
}
#[doc = "`PointSf`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\""]
#[doc = "  ],"]
#[doc = "  \"maximum\": 10.0,"]
#[doc = "  \"minimum\": -10.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PointSf {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for PointSf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for PointSf {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`PointStatic`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"D\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"D\","]
#[doc = "    \"S\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PointStatic {
    D,
    S,
}
impl ::std::fmt::Display for PointStatic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::D => f.write_str("D"),
            Self::S => f.write_str("S"),
        }
    }
}
impl ::std::str::FromStr for PointStatic {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "D" => Ok(Self::D),
            "S" => Ok(Self::S),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PointStatic {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PointStatic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PointStatic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PointStatic {
    fn default() -> Self {
        PointStatic::D
    }
}
#[doc = "`PointType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"int16\","]
#[doc = "    \"int32\","]
#[doc = "    \"int64\","]
#[doc = "    \"raw16\","]
#[doc = "    \"uint16\","]
#[doc = "    \"uint32\","]
#[doc = "    \"uint64\","]
#[doc = "    \"acc16\","]
#[doc = "    \"acc32\","]
#[doc = "    \"acc64\","]
#[doc = "    \"bitfield16\","]
#[doc = "    \"bitfield32\","]
#[doc = "    \"bitfield64\","]
#[doc = "    \"enum16\","]
#[doc = "    \"enum32\","]
#[doc = "    \"float32\","]
#[doc = "    \"float64\","]
#[doc = "    \"string\","]
#[doc = "    \"pad\","]
#[doc = "    \"ipaddr\","]
#[doc = "    \"ipv6addr\","]
#[doc = "    \"eui48\","]
#[doc = "    \"sunssf\","]
#[doc = "    \"count\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PointType {
    #[serde(rename = "int16")]
    Int16,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "int64")]
    Int64,
    #[serde(rename = "raw16")]
    Raw16,
    #[serde(rename = "uint16")]
    Uint16,
    #[serde(rename = "uint32")]
    Uint32,
    #[serde(rename = "uint64")]
    Uint64,
    #[serde(rename = "acc16")]
    Acc16,
    #[serde(rename = "acc32")]
    Acc32,
    #[serde(rename = "acc64")]
    Acc64,
    #[serde(rename = "bitfield16")]
    Bitfield16,
    #[serde(rename = "bitfield32")]
    Bitfield32,
    #[serde(rename = "bitfield64")]
    Bitfield64,
    #[serde(rename = "enum16")]
    Enum16,
    #[serde(rename = "enum32")]
    Enum32,
    #[serde(rename = "float32")]
    Float32,
    #[serde(rename = "float64")]
    Float64,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "pad")]
    Pad,
    #[serde(rename = "ipaddr")]
    Ipaddr,
    #[serde(rename = "ipv6addr")]
    Ipv6addr,
    #[serde(rename = "eui48")]
    Eui48,
    #[serde(rename = "sunssf")]
    Sunssf,
    #[serde(rename = "count")]
    Count,
}
impl ::std::fmt::Display for PointType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Int16 => f.write_str("int16"),
            Self::Int32 => f.write_str("int32"),
            Self::Int64 => f.write_str("int64"),
            Self::Raw16 => f.write_str("raw16"),
            Self::Uint16 => f.write_str("uint16"),
            Self::Uint32 => f.write_str("uint32"),
            Self::Uint64 => f.write_str("uint64"),
            Self::Acc16 => f.write_str("acc16"),
            Self::Acc32 => f.write_str("acc32"),
            Self::Acc64 => f.write_str("acc64"),
            Self::Bitfield16 => f.write_str("bitfield16"),
            Self::Bitfield32 => f.write_str("bitfield32"),
            Self::Bitfield64 => f.write_str("bitfield64"),
            Self::Enum16 => f.write_str("enum16"),
            Self::Enum32 => f.write_str("enum32"),
            Self::Float32 => f.write_str("float32"),
            Self::Float64 => f.write_str("float64"),
            Self::String => f.write_str("string"),
            Self::Pad => f.write_str("pad"),
            Self::Ipaddr => f.write_str("ipaddr"),
            Self::Ipv6addr => f.write_str("ipv6addr"),
            Self::Eui48 => f.write_str("eui48"),
            Self::Sunssf => f.write_str("sunssf"),
            Self::Count => f.write_str("count"),
        }
    }
}
impl ::std::str::FromStr for PointType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "int16" => Ok(Self::Int16),
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "raw16" => Ok(Self::Raw16),
            "uint16" => Ok(Self::Uint16),
            "uint32" => Ok(Self::Uint32),
            "uint64" => Ok(Self::Uint64),
            "acc16" => Ok(Self::Acc16),
            "acc32" => Ok(Self::Acc32),
            "acc64" => Ok(Self::Acc64),
            "bitfield16" => Ok(Self::Bitfield16),
            "bitfield32" => Ok(Self::Bitfield32),
            "bitfield64" => Ok(Self::Bitfield64),
            "enum16" => Ok(Self::Enum16),
            "enum32" => Ok(Self::Enum32),
            "float32" => Ok(Self::Float32),
            "float64" => Ok(Self::Float64),
            "string" => Ok(Self::String),
            "pad" => Ok(Self::Pad),
            "ipaddr" => Ok(Self::Ipaddr),
            "ipv6addr" => Ok(Self::Ipv6addr),
            "eui48" => Ok(Self::Eui48),
            "sunssf" => Ok(Self::Sunssf),
            "count" => Ok(Self::Count),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PointType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PointType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PointType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PointValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PointValue {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for PointValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for PointValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`Symbol`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"comments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"desc\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"detail\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {}"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Symbol {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub comments: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub desc: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    pub value: ::serde_json::Value,
}
impl Symbol {
    pub fn builder() -> builder::Symbol {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Group {
        comments:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        count: ::std::result::Result<super::GroupCount, ::std::string::String>,
        desc: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        groups: ::std::result::Result<::std::vec::Vec<super::Group>, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        points: ::std::result::Result<::std::vec::Vec<super::Point>, ::std::string::String>,
        type_: ::std::result::Result<super::GroupType, ::std::string::String>,
    }
    impl ::std::default::Default for Group {
        fn default() -> Self {
            Self {
                comments: Ok(Default::default()),
                count: Ok(super::defaults::group_count()),
                desc: Ok(Default::default()),
                detail: Ok(Default::default()),
                groups: Ok(Default::default()),
                label: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                points: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Group {
        pub fn comments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments: {e}"));
            self
        }
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupCount>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {e}"));
            self
        }
        pub fn desc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.desc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for desc: {e}"));
            self
        }
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Group>>,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn points<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.points = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for points: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Group> for super::Group {
        type Error = super::error::ConversionError;
        fn try_from(value: Group) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comments: value.comments?,
                count: value.count?,
                desc: value.desc?,
                detail: value.detail?,
                groups: value.groups?,
                label: value.label?,
                name: value.name?,
                notes: value.notes?,
                points: value.points?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Group> for Group {
        fn from(value: super::Group) -> Self {
            Self {
                comments: Ok(value.comments),
                count: Ok(value.count),
                desc: Ok(value.desc),
                detail: Ok(value.detail),
                groups: Ok(value.groups),
                label: Ok(value.label),
                name: Ok(value.name),
                notes: Ok(value.notes),
                points: Ok(value.points),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Point {
        access: ::std::result::Result<super::PointAccess, ::std::string::String>,
        comments:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        desc: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        mandatory: ::std::result::Result<super::PointMandatory, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sf: ::std::result::Result<::std::option::Option<super::PointSf>, ::std::string::String>,
        size: ::std::result::Result<i64, ::std::string::String>,
        standards:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        static_: ::std::result::Result<super::PointStatic, ::std::string::String>,
        symbols: ::std::result::Result<::std::vec::Vec<super::Symbol>, ::std::string::String>,
        type_: ::std::result::Result<super::PointType, ::std::string::String>,
        units: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value:
            ::std::result::Result<::std::option::Option<super::PointValue>, ::std::string::String>,
    }
    impl ::std::default::Default for Point {
        fn default() -> Self {
            Self {
                access: Ok(super::defaults::point_access()),
                comments: Ok(Default::default()),
                count: Ok(Default::default()),
                desc: Ok(Default::default()),
                detail: Ok(Default::default()),
                label: Ok(Default::default()),
                mandatory: Ok(super::defaults::point_mandatory()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                sf: Ok(Default::default()),
                size: Err("no value supplied for size".to_string()),
                standards: Ok(Default::default()),
                static_: Ok(super::defaults::point_static()),
                symbols: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                units: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Point {
        pub fn access<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PointAccess>,
            T::Error: ::std::fmt::Display,
        {
            self.access = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access: {e}"));
            self
        }
        pub fn comments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments: {e}"));
            self
        }
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {e}"));
            self
        }
        pub fn desc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.desc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for desc: {e}"));
            self
        }
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn mandatory<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PointMandatory>,
            T::Error: ::std::fmt::Display,
        {
            self.mandatory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mandatory: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn sf<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PointSf>>,
            T::Error: ::std::fmt::Display,
        {
            self.sf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sf: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn standards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.standards = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for standards: {e}"));
            self
        }
        pub fn static_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PointStatic>,
            T::Error: ::std::fmt::Display,
        {
            self.static_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for static_: {e}"));
            self
        }
        pub fn symbols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Symbol>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbols: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PointType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PointValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Point> for super::Point {
        type Error = super::error::ConversionError;
        fn try_from(value: Point) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                access: value.access?,
                comments: value.comments?,
                count: value.count?,
                desc: value.desc?,
                detail: value.detail?,
                label: value.label?,
                mandatory: value.mandatory?,
                name: value.name?,
                notes: value.notes?,
                sf: value.sf?,
                size: value.size?,
                standards: value.standards?,
                static_: value.static_?,
                symbols: value.symbols?,
                type_: value.type_?,
                units: value.units?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Point> for Point {
        fn from(value: super::Point) -> Self {
            Self {
                access: Ok(value.access),
                comments: Ok(value.comments),
                count: Ok(value.count),
                desc: Ok(value.desc),
                detail: Ok(value.detail),
                label: Ok(value.label),
                mandatory: Ok(value.mandatory),
                name: Ok(value.name),
                notes: Ok(value.notes),
                sf: Ok(value.sf),
                size: Ok(value.size),
                standards: Ok(value.standards),
                static_: Ok(value.static_),
                symbols: Ok(value.symbols),
                type_: Ok(value.type_),
                units: Ok(value.units),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Symbol {
        comments:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        desc: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for Symbol {
        fn default() -> Self {
            Self {
                comments: Ok(Default::default()),
                desc: Ok(Default::default()),
                detail: Ok(Default::default()),
                label: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Symbol {
        pub fn comments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments: {e}"));
            self
        }
        pub fn desc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.desc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for desc: {e}"));
            self
        }
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Symbol> for super::Symbol {
        type Error = super::error::ConversionError;
        fn try_from(value: Symbol) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comments: value.comments?,
                desc: value.desc?,
                detail: value.detail?,
                label: value.label?,
                name: value.name?,
                notes: value.notes?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Symbol> for Symbol {
        fn from(value: super::Symbol) -> Self {
            Self {
                comments: Ok(value.comments),
                desc: Ok(value.desc),
                detail: Ok(value.detail),
                label: Ok(value.label),
                name: Ok(value.name),
                notes: Ok(value.notes),
                value: Ok(value.value),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn group_count() -> super::GroupCount {
        super::GroupCount::Integer(1_i64)
    }
    pub(super) fn point_access() -> super::PointAccess {
        super::PointAccess::R
    }
    pub(super) fn point_mandatory() -> super::PointMandatory {
        super::PointMandatory::O
    }
    pub(super) fn point_static() -> super::PointStatic {
        super::PointStatic::D
    }
}
