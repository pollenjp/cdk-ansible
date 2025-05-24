//! combined types for Json Schema

use indexmap::IndexMap;
use serde::Serialize;
use std::path::PathBuf;

/// A boolean or a string
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String),
}

impl From<bool> for BoolOrString {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for BoolOrString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for BoolOrString {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

/// i64 or string
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IntOrString {
    Int(i64),
    String(String),
}

impl From<i64> for IntOrString {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<String> for IntOrString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for IntOrString {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

/// A string or a vector of strings
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrVecString {
    String(String),
    VecString(Vec<String>),
}

impl From<String> for StringOrVecString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<String>> for StringOrVecString {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}

impl From<&str> for StringOrVecString {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

/// A boolean or a string or a vector of strings
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum BoolOrStringOrVecString {
    Bool(bool),
    String(String),
    VecString(Vec<String>),
}

impl From<bool> for BoolOrStringOrVecString {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for BoolOrStringOrVecString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for BoolOrStringOrVecString {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Vec<String>> for BoolOrStringOrVecString {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}

/// String or Path
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrPath {
    String(String),
    Path(PathBuf),
}

impl From<PathBuf> for StringOrPath {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

impl From<String> for StringOrPath {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for StringOrPath {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

// String or Vec<Value>
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrVec {
    String(String),
    Vec(Vec<::serde_json::Value>),
}

impl From<Vec<::serde_json::Value>> for StringOrVec {
    fn from(value: Vec<::serde_json::Value>) -> Self {
        Self::Vec(value)
    }
}

impl From<String> for StringOrVec {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for StringOrVec {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

// String or Map<String, Value>
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrMap {
    String(String),
    Map(IndexMap<String, ::serde_json::Value>),
}

impl From<IndexMap<String, ::serde_json::Value>> for StringOrMap {
    fn from(value: IndexMap<String, ::serde_json::Value>) -> Self {
        Self::Map(value)
    }
}

impl From<String> for StringOrMap {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for StringOrMap {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
