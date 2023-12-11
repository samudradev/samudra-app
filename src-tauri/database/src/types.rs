#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, diff::Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(untagged)]
pub enum DbProvided<T>
where
    T: ts_rs::TS + core::fmt::Debug + PartialEq + diff::Diff,
    <T as diff::Diff>::Repr: core::fmt::Debug,
{
    Known(T),
    Unknown,
}

impl<T> From<Option<T>> for DbProvided<T>
where
    T: ts_rs::TS + core::fmt::Debug + PartialEq + diff::Diff,
    <T as diff::Diff>::Repr: core::fmt::Debug,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(T) => Self::Known(T),
            None => Self::Unknown,
        }
    }
}

/// DbProvided<T> acts like an option in JS/TS
impl<T> ts_rs::TS for DbProvided<T>
where
    T: ts_rs::TS + core::fmt::Debug + PartialEq + diff::Diff,
    <T as diff::Diff>::Repr: core::fmt::Debug,
{
    fn name() -> String {
        <Option<T> as ts_rs::TS>::name()
    }

    fn dependencies() -> Vec<ts_rs::Dependency> {
        <Option<T> as ts_rs::TS>::dependencies()
    }

    fn transparent() -> bool {
        <Option<T> as ts_rs::TS>::transparent()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        <Option<T> as ts_rs::TS>::name_with_type_args(args)
    }

    fn inline() -> String {
        <Option<T> as ts_rs::TS>::inline()
    }

    fn inline_flattened() -> String {
        <Option<T> as ts_rs::TS>::inline_flattened()
    }
}
