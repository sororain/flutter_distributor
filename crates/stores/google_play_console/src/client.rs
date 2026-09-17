#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
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

    ///`AndroidpublisherEditsCommitAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsCommitAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsCommitAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsCommitAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsCommitAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsCommitAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsCommitAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsCommitXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsCommitXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsCommitXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsCommitXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsCommitXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsCommitXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsCommitXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsInsertAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsInsertAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsInsertAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsInsertAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsInsertAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsInsertAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsInsertAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsInsertXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsInsertXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsInsertXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsInsertXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsInsertXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsInsertXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsInsertXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsDeleteAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsDeleteAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsDeleteAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsDeleteAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsDeleteAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsDeleteAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsDeleteAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsDeleteXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsDeleteXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsDeleteXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsDeleteXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsDeleteXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsDeleteXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsDeleteXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsDeleteallAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsDeleteallAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsDeleteallAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsDeleteallAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsDeleteallAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsDeleteallAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsDeleteallAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsDeleteallXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsDeleteallXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsDeleteallXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsDeleteallXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsDeleteallXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AndroidpublisherEditsListingsDeleteallXgafv
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AndroidpublisherEditsListingsDeleteallXgafv
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsGetAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsGetAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsGetAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsGetAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsGetXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsGetXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsGetXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsGetXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsListAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsListAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsListAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsListAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsListAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsListAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsListAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsListXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsListXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsListXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsListXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsPatchAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsPatchAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsPatchAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsPatchAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsPatchXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsPatchXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsPatchXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsPatchXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsUpdateAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsUpdateAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsUpdateAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsUpdateAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsListingsUpdateXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsListingsUpdateXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsListingsUpdateXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsListingsUpdateXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsListingsUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsListingsUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsListingsUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksCreateAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksCreateAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksCreateAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksCreateAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksCreateAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksCreateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksCreateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksCreateXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksCreateXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksCreateXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksCreateXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksCreateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksCreateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksCreateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksGetAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksGetAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksGetAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksGetAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksGetAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksGetXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksGetXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksGetXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksGetXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksGetXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksListAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksListAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksListAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksListAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksListAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksListAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksListAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksListXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksListXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksListXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksListXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksListXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksPatchAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksPatchAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksPatchAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksPatchAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksPatchAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksPatchXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksPatchXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksPatchXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksPatchXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksPatchXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksUpdateAlt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json",
    ///    "media",
    ///    "proto"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksUpdateAlt {
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "media")]
        Media,
        #[serde(rename = "proto")]
        Proto,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksUpdateAlt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => f.write_str("json"),
                Self::Media => f.write_str("media"),
                Self::Proto => f.write_str("proto"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksUpdateAlt {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                "media" => Ok(Self::Media),
                "proto" => Ok(Self::Proto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksUpdateAlt {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AndroidpublisherEditsTracksUpdateXgafv`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1",
    ///    "2"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum AndroidpublisherEditsTracksUpdateXgafv {
        #[serde(rename = "1")]
        X1,
        #[serde(rename = "2")]
        X2,
    }

    impl ::std::fmt::Display for AndroidpublisherEditsTracksUpdateXgafv {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1 => f.write_str("1"),
                Self::X2 => f.write_str("2"),
            }
        }
    }

    impl ::std::str::FromStr for AndroidpublisherEditsTracksUpdateXgafv {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1" => Ok(Self::X1),
                "2" => Ok(Self::X2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AndroidpublisherEditsTracksUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AndroidpublisherEditsTracksUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AndroidpublisherEditsTracksUpdateXgafv {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///An app edit. The resource for EditsService.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An app edit. The resource for EditsService.",
    ///  "type": "object",
    ///  "properties": {
    ///    "expiryTimeSeconds": {
    ///      "description": "Output only. The time (as seconds since Epoch) at
    /// which the edit will expire and will be no longer valid for use.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Output only. Identifier of the edit. Can be used in
    /// subsequent API calls.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppEdit {
        ///Output only. The time (as seconds since Epoch) at which the edit
        /// will expire and will be no longer valid for use.
        #[serde(
            rename = "expiryTimeSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiry_time_seconds: ::std::option::Option<::std::string::String>,
        ///Output only. Identifier of the edit. Can be used in subsequent API
        /// calls.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppEdit {
        fn default() -> Self {
            Self {
                expiry_time_seconds: Default::default(),
                id: Default::default(),
            }
        }
    }

    ///Country targeting specification.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Country targeting specification.",
    ///  "type": "object",
    ///  "properties": {
    ///    "countries": {
    ///      "description": "Countries to target, specified as two letter [CLDR codes](https://unicode.org/cldr/charts/latest/supplemental/territory_containment_un_m_49.html).",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "includeRestOfWorld": {
    ///      "description": "Include \"rest of world\" as well as explicitly
    /// targeted countries.",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CountryTargeting {
        ///Countries to target, specified as two letter [CLDR codes](https://unicode.org/cldr/charts/latest/supplemental/territory_containment_un_m_49.html).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub countries: ::std::vec::Vec<::std::string::String>,
        ///Include "rest of world" as well as explicitly targeted countries.
        #[serde(
            rename = "includeRestOfWorld",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub include_rest_of_world: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for CountryTargeting {
        fn default() -> Self {
            Self {
                countries: Default::default(),
                include_rest_of_world: Default::default(),
            }
        }
    }

    ///A localized store listing. The resource for ListingsService.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A localized store listing. The resource for
    /// ListingsService.",
    ///  "type": "object",
    ///  "properties": {
    ///    "fullDescription": {
    ///      "description": "Full description of the app.",
    ///      "type": "string"
    ///    },
    ///    "language": {
    ///      "description": "Language localization code (a BCP-47 language tag;
    /// for example, \"de-AT\" for Austrian German).",
    ///      "type": "string"
    ///    },
    ///    "shortDescription": {
    ///      "description": "Short description of the app.",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "Localized title of the app.",
    ///      "type": "string"
    ///    },
    ///    "video": {
    ///      "description": "URL of a promotional YouTube video for the app.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Listing {
        ///Full description of the app.
        #[serde(
            rename = "fullDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub full_description: ::std::option::Option<::std::string::String>,
        ///Language localization code (a BCP-47 language tag; for example,
        /// "de-AT" for Austrian German).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        ///Short description of the app.
        #[serde(
            rename = "shortDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_description: ::std::option::Option<::std::string::String>,
        ///Localized title of the app.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        ///URL of a promotional YouTube video for the app.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub video: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Listing {
        fn default() -> Self {
            Self {
                full_description: Default::default(),
                language: Default::default(),
                short_description: Default::default(),
                title: Default::default(),
                video: Default::default(),
            }
        }
    }

    ///Response listing all localized listings.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response listing all localized listings.",
    ///  "type": "object",
    ///  "properties": {
    ///    "kind": {
    ///      "description": "The kind of this response
    /// (\"androidpublisher#listingsListResponse\").",
    ///      "type": "string"
    ///    },
    ///    "listings": {
    ///      "description": "All localized listings.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Listing"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListingsListResponse {
        ///The kind of this response ("androidpublisher#listingsListResponse").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        ///All localized listings.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub listings: ::std::vec::Vec<Listing>,
    }

    impl ::std::default::Default for ListingsListResponse {
        fn default() -> Self {
            Self {
                kind: Default::default(),
                listings: Default::default(),
            }
        }
    }

    ///An uploaded image. The resource for ImagesService.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An uploaded image.",
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "description": "A unique id representing this image.",
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "description": "A URL that will serve a preview of the image.",
    ///      "type": "string"
    ///    },
    ///    "sha1": {
    ///      "description": "A sha1 hash of the image.",
    ///      "type": "string"
    ///    },
    ///    "sha256": {
    ///      "description": "A sha256 hash of the image.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Image {
        ///A unique id representing this image.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///A URL that will serve a preview of the image.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        ///A sha1 hash of the image.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha1: ::std::option::Option<::std::string::String>,
        ///A sha256 hash of the image.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha256: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Image {
        fn default() -> Self {
            Self {
                id: Default::default(),
                url: Default::default(),
                sha1: Default::default(),
                sha256: Default::default(),
            }
        }
    }

    ///Response listing all images.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response listing all images.",
    ///  "type": "object",
    ///  "properties": {
    ///    "images": {
    ///      "description": "All localized images.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Image"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImagesListResponse {
        ///All images.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub images: ::std::vec::Vec<Image>,
    }

    impl ::std::default::Default for ImagesListResponse {
        fn default() -> Self {
            Self {
                images: Default::default(),
            }
        }
    }

    ///Response for uploading an image.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response for uploading an image.",
    ///  "type": "object",
    ///  "properties": {
    ///    "image": {
    ///      "$ref": "#/components/schemas/Image"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImagesUploadResponse {
        ///The uploaded image.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<Image>,
    }

    impl ::std::default::Default for ImagesUploadResponse {
        fn default() -> Self {
            Self {
                image: Default::default(),
            }
        }
    }

    ///Response for deleting all images.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response for deleting all images.",
    ///  "type": "object",
    ///  "properties": {
    ///    "deleted": {
    ///      "description": "The deleted images.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Image"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImagesDeleteAllResponse {
        ///The deleted images.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub deleted: ::std::vec::Vec<Image>,
    }

    impl ::std::default::Default for ImagesDeleteAllResponse {
        fn default() -> Self {
            Self {
                deleted: Default::default(),
            }
        }
    }

    ///Localized text in given language.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Localized text in given language.",
    ///  "type": "object",
    ///  "properties": {
    ///    "language": {
    ///      "description": "Language localization code (a BCP-47 language tag;
    /// for example, \"de-AT\" for Austrian German).",
    ///      "type": "string"
    ///    },
    ///    "text": {
    ///      "description": "The text in the given language.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LocalizedText {
        ///Language localization code (a BCP-47 language tag; for example,
        /// "de-AT" for Austrian German).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        ///The text in the given language.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for LocalizedText {
        fn default() -> Self {
            Self {
                language: Default::default(),
                text: Default::default(),
            }
        }
    }

    ///A track configuration. The resource for TracksService.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A track configuration. The resource for
    /// TracksService.",
    ///  "type": "object",
    ///  "properties": {
    ///    "releases": {
    ///      "description": "In a read request, represents all active releases
    /// in the track. In an update request, represents desired changes.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TrackRelease"
    ///      }
    ///    },
    ///    "track": {
    ///      "description": "Identifier of the track. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Track {
        ///In a read request, represents all active releases in the track. In
        /// an update request, represents desired changes.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub releases: ::std::vec::Vec<TrackRelease>,
        ///Identifier of the track. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Track {
        fn default() -> Self {
            Self {
                releases: Default::default(),
                track: Default::default(),
            }
        }
    }

    ///Configurations of the new track.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Configurations of the new track.",
    ///  "type": "object",
    ///  "properties": {
    ///    "formFactor": {
    ///      "description": "Required. Form factor of the new track. Defaults to
    /// the default track.",
    ///      "type": "string",
    ///      "enum": [
    ///        "FORM_FACTOR_UNSPECIFIED",
    ///        "DEFAULT",
    ///        "WEAR",
    ///        "AUTOMOTIVE"
    ///      ]
    ///    },
    ///    "track": {
    ///      "description": "Required. Identifier of the new track. For default tracks, this field consists of the track alias only. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. This prefix must match the value of the `form_factor` field, if it is not a default track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "Required. Type of the new track. Currently, the
    /// only supported value is closedTesting.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TRACK_TYPE_UNSPECIFIED",
    ///        "CLOSED_TESTING"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TrackConfig {
        ///Required. Form factor of the new track. Defaults to the default
        /// track.
        #[serde(
            rename = "formFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub form_factor: ::std::option::Option<TrackConfigFormFactor>,
        ///Required. Identifier of the new track. For default tracks, this field consists of the track alias only. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. This prefix must match the value of the `form_factor` field, if it is not a default track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<::std::string::String>,
        ///Required. Type of the new track. Currently, the only supported value
        /// is closedTesting.
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TrackConfigType>,
    }

    impl ::std::default::Default for TrackConfig {
        fn default() -> Self {
            Self {
                form_factor: Default::default(),
                track: Default::default(),
                type_: Default::default(),
            }
        }
    }

    ///Required. Form factor of the new track. Defaults to the default track.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Required. Form factor of the new track. Defaults to the
    /// default track.",
    ///  "type": "string",
    ///  "enum": [
    ///    "FORM_FACTOR_UNSPECIFIED",
    ///    "DEFAULT",
    ///    "WEAR",
    ///    "AUTOMOTIVE"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum TrackConfigFormFactor {
        #[serde(rename = "FORM_FACTOR_UNSPECIFIED")]
        FormFactorUnspecified,
        #[serde(rename = "DEFAULT")]
        Default,
        #[serde(rename = "WEAR")]
        Wear,
        #[serde(rename = "AUTOMOTIVE")]
        Automotive,
    }

    impl ::std::fmt::Display for TrackConfigFormFactor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FormFactorUnspecified => f.write_str("FORM_FACTOR_UNSPECIFIED"),
                Self::Default => f.write_str("DEFAULT"),
                Self::Wear => f.write_str("WEAR"),
                Self::Automotive => f.write_str("AUTOMOTIVE"),
            }
        }
    }

    impl ::std::str::FromStr for TrackConfigFormFactor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FORM_FACTOR_UNSPECIFIED" => Ok(Self::FormFactorUnspecified),
                "DEFAULT" => Ok(Self::Default),
                "WEAR" => Ok(Self::Wear),
                "AUTOMOTIVE" => Ok(Self::Automotive),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TrackConfigFormFactor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TrackConfigFormFactor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TrackConfigFormFactor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Required. Type of the new track. Currently, the only supported value is
    /// closedTesting.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Required. Type of the new track. Currently, the only
    /// supported value is closedTesting.",
    ///  "type": "string",
    ///  "enum": [
    ///    "TRACK_TYPE_UNSPECIFIED",
    ///    "CLOSED_TESTING"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum TrackConfigType {
        #[serde(rename = "TRACK_TYPE_UNSPECIFIED")]
        TrackTypeUnspecified,
        #[serde(rename = "CLOSED_TESTING")]
        ClosedTesting,
    }

    impl ::std::fmt::Display for TrackConfigType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::TrackTypeUnspecified => f.write_str("TRACK_TYPE_UNSPECIFIED"),
                Self::ClosedTesting => f.write_str("CLOSED_TESTING"),
            }
        }
    }

    impl ::std::str::FromStr for TrackConfigType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TRACK_TYPE_UNSPECIFIED" => Ok(Self::TrackTypeUnspecified),
                "CLOSED_TESTING" => Ok(Self::ClosedTesting),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TrackConfigType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TrackConfigType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TrackConfigType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A release within a track.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A release within a track.",
    ///  "type": "object",
    ///  "properties": {
    ///    "countryTargeting": {
    ///      "$ref": "#/components/schemas/CountryTargeting"
    ///    },
    ///    "inAppUpdatePriority": {
    ///      "description": "In-app update priority of the release. All newly added APKs in the release will be considered at this priority. Can take values in the range [0, 5], with 5 the highest priority. Defaults to 0. in_app_update_priority can not be updated once the release is rolled out. See https://developer.android.com/guide/playcore/in-app-updates.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "name": {
    ///      "description": "The release name. Not required to be unique. If not
    /// set, the name is generated from the APK's version_name. If the release
    /// contains multiple APKs, the name is generated from the date.",
    ///      "type": "string"
    ///    },
    ///    "releaseNotes": {
    ///      "description": "A description of what is new in this release.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LocalizedText"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "The status of the release.",
    ///      "type": "string",
    ///      "enum": [
    ///        "statusUnspecified",
    ///        "draft",
    ///        "inProgress",
    ///        "halted",
    ///        "completed"
    ///      ]
    ///    },
    ///    "userFraction": {
    ///      "description": "Fraction of users who are eligible for a staged
    /// release. 0 < fraction < 1. Can only be set when status is \"inProgress\"
    /// or \"halted\".",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "versionCodes": {
    ///      "description": "Version codes of all APKs in the release. Must
    /// include version codes to retain from previous releases.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "format": "int64"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TrackRelease {
        #[serde(
            rename = "countryTargeting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub country_targeting: ::std::option::Option<CountryTargeting>,
        ///In-app update priority of the release. All newly added APKs in the release will be considered at this priority. Can take values in the range [0, 5], with 5 the highest priority. Defaults to 0. in_app_update_priority can not be updated once the release is rolled out. See https://developer.android.com/guide/playcore/in-app-updates.
        #[serde(
            rename = "inAppUpdatePriority",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub in_app_update_priority: ::std::option::Option<i32>,
        ///The release name. Not required to be unique. If not set, the name is
        /// generated from the APK's version_name. If the release contains
        /// multiple APKs, the name is generated from the date.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///A description of what is new in this release.
        #[serde(
            rename = "releaseNotes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub release_notes: ::std::vec::Vec<LocalizedText>,
        ///The status of the release.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<TrackReleaseStatus>,
        ///Fraction of users who are eligible for a staged release. 0 <
        /// fraction < 1. Can only be set when status is "inProgress" or
        /// "halted".
        #[serde(
            rename = "userFraction",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_fraction: ::std::option::Option<f64>,
        ///Version codes of all APKs in the release. Must include version codes
        /// to retain from previous releases.
        #[serde(
            rename = "versionCodes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub version_codes: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for TrackRelease {
        fn default() -> Self {
            Self {
                country_targeting: Default::default(),
                in_app_update_priority: Default::default(),
                name: Default::default(),
                release_notes: Default::default(),
                status: Default::default(),
                user_fraction: Default::default(),
                version_codes: Default::default(),
            }
        }
    }

    ///The status of the release.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The status of the release.",
    ///  "type": "string",
    ///  "enum": [
    ///    "statusUnspecified",
    ///    "draft",
    ///    "inProgress",
    ///    "halted",
    ///    "completed"
    ///  ]
    ///}
    /// ```
    /// </details>
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
    pub enum TrackReleaseStatus {
        #[serde(rename = "statusUnspecified")]
        StatusUnspecified,
        #[serde(rename = "draft")]
        Draft,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "halted")]
        Halted,
        #[serde(rename = "completed")]
        Completed,
    }

    impl ::std::fmt::Display for TrackReleaseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StatusUnspecified => f.write_str("statusUnspecified"),
                Self::Draft => f.write_str("draft"),
                Self::InProgress => f.write_str("inProgress"),
                Self::Halted => f.write_str("halted"),
                Self::Completed => f.write_str("completed"),
            }
        }
    }

    impl ::std::str::FromStr for TrackReleaseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "statusUnspecified" => Ok(Self::StatusUnspecified),
                "draft" => Ok(Self::Draft),
                "inProgress" => Ok(Self::InProgress),
                "halted" => Ok(Self::Halted),
                "completed" => Ok(Self::Completed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TrackReleaseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TrackReleaseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TrackReleaseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Response listing all tracks.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response listing all tracks.",
    ///  "type": "object",
    ///  "properties": {
    ///    "kind": {
    ///      "description": "The kind of this response
    /// (\"androidpublisher#tracksListResponse\").",
    ///      "type": "string"
    ///    },
    ///    "tracks": {
    ///      "description": "All tracks (including tracks with no releases).",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Track"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TracksListResponse {
        ///The kind of this response ("androidpublisher#tracksListResponse").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        ///All tracks (including tracks with no releases).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tracks: ::std::vec::Vec<Track>,
    }

    impl ::std::default::Default for TracksListResponse {
        fn default() -> Self {
            Self {
                kind: Default::default(),
                tracks: Default::default(),
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Google Play Android Developer API
///
///Lets Android application developers access their Google Play accounts. At a
/// high level, the expected workflow is to "insert" an Edit, make changes as
/// necessary, and then "commit" it.
///
///https://developers.google.com/terms/
///
///Version: v3
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "v3"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Creates a new edit for an app.
    ///
    ///Sends a `POST` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_insert<'a>(
        &'a self,
        package_name: &'a str,
        xgafv: Option<types::AndroidpublisherEditsInsertXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsInsertAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::AppEdit,
    ) -> Result<ResponseValue<types::AppEdit>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits",
            self.baseurl,
            encode_path(&package_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_insert",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Lists all localized store listings.
    ///
    ///Sends a `GET` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_listings_list<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsListXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsListAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListingsListResponse>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deletes all store listings.
    ///
    ///Sends a `DELETE` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_listings_deleteall<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsDeleteallXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsDeleteallAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_deleteall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Gets a localized store listing.
    ///
    ///Sends a `GET` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings/{language}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `language`: Language localization code (a BCP-47 language tag; for
    ///   example, "de-AT" for Austrian German).
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_listings_get<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsGetXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsGetAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<types::Listing>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Creates or updates a localized store listing.
    ///
    ///Sends a `PUT` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings/{language}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `language`: Language localization code (a BCP-47 language tag; for
    ///   example, "de-AT" for Austrian German).
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_listings_update<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsUpdateXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsUpdateAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::Listing,
    ) -> Result<ResponseValue<types::Listing>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_update",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deletes a localized store listing.
    ///
    ///Sends a `DELETE` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings/{language}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `language`: Language localization code (a BCP-47 language tag; for
    ///   example, "de-AT" for Austrian German).
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_listings_delete<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsDeleteXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsDeleteAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_delete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Patches a localized store listing.
    ///
    ///Sends a `PATCH` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/
    /// listings/{language}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `language`: Language localization code (a BCP-47 language tag; for
    ///   example, "de-AT" for Austrian German).
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_listings_patch<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        xgafv: Option<types::AndroidpublisherEditsListingsPatchXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsListingsPatchAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::Listing,
    ) -> Result<ResponseValue<types::Listing>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_listings_patch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Lists all tracks.
    ///
    ///Sends a `GET` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_tracks_list<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        xgafv: Option<types::AndroidpublisherEditsTracksListXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsTracksListAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<types::TracksListResponse>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/tracks",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_tracks_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Creates a new track.
    ///
    ///Sends a `POST` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks`
    ///
    ///Arguments:
    /// - `package_name`: Required. Package name of the app.
    /// - `edit_id`: Required. Identifier of the edit.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_tracks_create<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        xgafv: Option<types::AndroidpublisherEditsTracksCreateXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsTracksCreateAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::TrackConfig,
    ) -> Result<ResponseValue<types::Track>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/tracks",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_tracks_create",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Gets a track.
    ///
    ///Sends a `GET` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks/
    /// {track}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `track`: Identifier of the track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_tracks_get<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        track: &'a str,
        xgafv: Option<types::AndroidpublisherEditsTracksGetXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsTracksGetAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<types::Track>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/tracks/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&track.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_tracks_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Updates a track.
    ///
    ///Sends a `PUT` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks/
    /// {track}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `track`: Identifier of the track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_tracks_update<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        track: &'a str,
        xgafv: Option<types::AndroidpublisherEditsTracksUpdateXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsTracksUpdateAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::Track,
    ) -> Result<ResponseValue<types::Track>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/tracks/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&track.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_tracks_update",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Patches a track.
    ///
    ///Sends a `PATCH` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks/
    /// {track}`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `track`: Identifier of the track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name)
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    /// - `body`
    pub async fn androidpublisher_edits_tracks_patch<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        track: &'a str,
        xgafv: Option<types::AndroidpublisherEditsTracksPatchXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsTracksPatchAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
        body: &'a types::Track,
    ) -> Result<ResponseValue<types::Track>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/tracks/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&track.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_tracks_patch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Commits an app edit.
    ///
    ///Sends a `POST` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}:commit`
    ///
    ///Arguments:
    /// - `package_name`: Package name of the app.
    /// - `edit_id`: Identifier of the edit.
    /// - `xgafv`: V1 error format.
    /// - `access_token`: OAuth access token.
    /// - `alt`: Data format for response.
    /// - `callback`: JSONP
    /// - `changes_not_sent_for_review`: Indicates that the changes in this edit
    ///   will not be reviewed until they are explicitly sent for review from
    ///   the Google Play Console UI. These changes will be added to any other
    ///   changes that are not yet sent for review.
    /// - `fields`: Selector specifying which fields to include in a partial
    ///   response.
    /// - `key`: API key. Your API key identifies your project and provides you
    ///   with API access, quota, and reports. Required unless you provide an
    ///   OAuth 2.0 token.
    /// - `oauth_token`: OAuth 2.0 token for the current user.
    /// - `pretty_print`: Returns response with indentations and line breaks.
    /// - `quota_user`: Available to use for quota purposes for server-side
    ///   applications. Can be any arbitrary string assigned to a user, but
    ///   should not exceed 40 characters.
    /// - `upload_type`: Legacy upload protocol for media (e.g. "media",
    ///   "multipart").
    /// - `upload_protocol`: Upload protocol for media (e.g. "raw",
    ///   "multipart").
    pub async fn androidpublisher_edits_commit<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        xgafv: Option<types::AndroidpublisherEditsCommitXgafv>,
        access_token: Option<&'a str>,
        alt: Option<types::AndroidpublisherEditsCommitAlt>,
        callback: Option<&'a str>,
        fields: Option<&'a str>,
        key: Option<&'a str>,
        oauth_token: Option<&'a str>,
        pretty_print: Option<bool>,
        quota_user: Option<&'a str>,
        upload_type: Option<&'a str>,
        upload_protocol: Option<&'a str>,
    ) -> Result<ResponseValue<types::AppEdit>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}:commit",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("$.xgafv", &xgafv))
            .query(&progenitor_client::QueryParam::new(
                "access_token",
                &access_token,
            ))
            .query(&progenitor_client::QueryParam::new("alt", &alt))
            .query(&progenitor_client::QueryParam::new("callback", &callback))
            .query(&progenitor_client::QueryParam::new("fields", &fields))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new(
                "oauth_token",
                &oauth_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "prettyPrint",
                &pretty_print,
            ))
            .query(&progenitor_client::QueryParam::new(
                "quotaUser",
                &quota_user,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadType",
                &upload_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "upload_protocol",
                &upload_protocol,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_commit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Lists all images for the specified language and image type.
    ///
    ///Sends a `GET` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/listings/{language}/images/{imageType}`
    pub async fn androidpublisher_edits_images_list<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        image_type: &'a str,
    ) -> Result<ResponseValue<types::ImagesListResponse>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
            encode_path(&image_type.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_images_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deletes all images for the specified language and image type.
    ///
    ///Sends a `DELETE` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/listings/{language}/images/{imageType}`
    pub async fn androidpublisher_edits_images_deleteall<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        image_type: &'a str,
    ) -> Result<ResponseValue<types::ImagesDeleteAllResponse>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
            encode_path(&image_type.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_images_deleteall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Uploads an image for the specified language and image type.
    ///
    ///Sends a `POST` request to
    /// `/androidpublisher/v3/applications/{packageName}/edits/{editId}/listings/{language}/images/{imageType}`
    ///with the image bytes as the request body.
    pub async fn androidpublisher_edits_images_upload<'a>(
        &'a self,
        package_name: &'a str,
        edit_id: &'a str,
        language: &'a str,
        image_type: &'a str,
        image_bytes: &'a [u8],
        mime_type: &'a str,
    ) -> Result<ResponseValue<types::ImagesUploadResponse>, Error<()>> {
        let url = format!(
            "{}/androidpublisher/v3/applications/{}/edits/{}/listings/{}/{}",
            self.baseurl,
            encode_path(&package_name.to_string()),
            encode_path(&edit_id.to_string()),
            encode_path(&language.to_string()),
            encode_path(&image_type.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .header(::reqwest::header::CONTENT_TYPE, mime_type)
            .body(image_bytes.to_vec())
            .build()?;
        let info = OperationInfo {
            operation_id: "androidpublisher_edits_images_upload",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
