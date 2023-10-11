// This macro is written as such to accomodate the ts_rs export macro
// which apparently does not allow the Option enum to be passed into a macro.
// Doing so would raise `unreachable!()`.
// It needs to be defined inside this macro_rules.
// Refer https://github.com/Aleph-Alpha/ts-rs/issues/83
// (I have tried the suggestion to use qualified full path `std::option::Option` resulting in the same error.)
macro_rules! export {
    {
        $dataclass:ident from $datamodel:ident with {
            $($field:ident : $type:ty),*
            $(;rename $old_field:ident to $new_field:ident: $renamed:ty),*
            $(;optional {
                $($optfield:ident : $opttype:ty),*
            })?
            $(;attachment {
                $($tagfield:ident : $tagmodel:ident => ..$tagclass:ident),*
            })?
    }
    } => {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Default, ts_rs::TS, diff::Diff)]
        #[ts(export, export_to="../../src/bindings/")]
        #[diff(attr(
            #[derive(Debug)]
        ))]
        pub struct $dataclass {
            $(
                pub $field: $type,
            )*
            $(
                pub $new_field: $renamed,
            )*
            $($(
                pub $optfield: Option::<$opttype>,
            )*)?
            $($(
                pub $tagfield: Option::<Vec::<$tagclass>>,
            )*)?
        }
        impl From<$datamodel> for $dataclass {
            fn from(value: $datamodel) -> Self {
                $dataclass {
                    $(
                        $field: value.$field,
                    )*
                    $($(
                        $optfield: value.$optfield.into(),
                    )*)?
                    $(
                        $new_field: <$renamed>::from(value.$old_field.unwrap_or("undefined".to_string())),
                    )*
                    ..Default::default()
                }
            }
        }
        $($(impl From<($datamodel, Vec<$tagmodel>)> for $dataclass {
            fn from(value: ($datamodel, Vec<$tagmodel>)) -> Self {
                let mut lem = $dataclass::from(value.0.clone());
                lem.$tagfield = Some(
                    value
                        .1
                        .into_iter()
                        .map(|kon| $tagclass::from(kon))
                        .collect::<Vec<$tagclass>>(),
                );
                lem
            }
        })*)?
    };
}
