use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind, Language};

#[rustfmt::skip::macros(language_support)]
pub fn supported_languages<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [Language] {
    macro_rules! language_support {
        ($(($entity_kind:ident, [$($language:ident),*]),)*) => {
            match entity_kind.into_builtin_kind() {
                $(
                    BuiltinEntityKind::$entity_kind => &[$(Language::$language,)*],
                )*
            }
        }
    }

    language_support!(
        (AmountOfMoney, [EN]),
        (Duration,      [EN]),
        (Number,        [EN]),
        (Ordinal,       [EN]),
        (Temperature,   [EN]),
        (Datetime,      [EN]),
        (Date,          [EN]),
        (Time,          [EN]),
        (DatePeriod,    [EN]),
        (TimePeriod,    [EN]),
        (Percentage,    [EN]),
        (MusicAlbum,    [EN]),
        (MusicArtist,   [EN]),
        (MusicTrack,    [EN]),
        (City,          [EN]),
        (Country,       [EN]),
        (Region,        [EN]),
    )
}
