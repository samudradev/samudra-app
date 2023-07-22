macro_rules! export {
    {$dataclass:ident from $datamodel:ident
        {
            $($field:ident : $type:ty),*;
            $($tagfield:ident : $tagmodel:ident => ..$tagclass:ident),*
        }
    } => {
        #[derive(Debug, Serialize, Deserialize, Default)]
        pub struct $dataclass {
            $(
                pub $field: $type,
            )*
            $(
                $tagfield: Option::<Vec::<$tagclass>>,
            )*
        }
        impl From<$datamodel> for $dataclass {
            fn from(value: $datamodel) -> Self {
                $dataclass {
                    $(
                        $field: value.$field,
                    )*
                    ..Default::default()
                }
            }
        }
        $(impl From<($datamodel, Vec<$tagmodel>)> for $dataclass {
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
        })*
    };
}
