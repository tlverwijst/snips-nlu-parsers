use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind};

pub fn en_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "$10",
            "six euros",
            "around 5€",
            "ten dollars and five cents",
        ],
        BuiltinEntityKind::Duration => &[
            "1h",
            "during two minutes",
            "for 20 seconds",
            "3 months",
            "half an hour",
            "8 years and two days",
        ],
        BuiltinEntityKind::Number => &["2001", "twenty one", "three hundred and four"],
        BuiltinEntityKind::Ordinal => &["1st", "the second", "the twenty third"],
        BuiltinEntityKind::Temperature => &[
            "70K",
            "3°C",
            "Twenty three degrees",
            "one hundred degrees fahrenheit",
        ],
        BuiltinEntityKind::Datetime => &[
            "tomorrow at 9pm",
            "today",
            "on october 1st at 10am",
            "at 8 a.m.",
            "4:30 pm",
            "in 1 hour",
            "the 3rd tuesday of June",
        ],
        BuiltinEntityKind::Date => &[
            "today",
            "on Wednesday",
            "March 26th",
            "saturday january 19",
            "monday 15th april 2019",
            "the day after tomorrow",
        ],
        BuiltinEntityKind::Time => &[
            "now",
            "at noon",
            "at 8 a.m.",
            "4:30 pm",
            "in one hour",
            "for ten o'clock",
            "at ten in the evening",
        ],
        BuiltinEntityKind::DatePeriod => &[
            "january",
            "2019",
            "from monday to friday",
            "from wednesday 27th to saturday 30th",
            "this week",
        ],
        BuiltinEntityKind::TimePeriod => &[
            // "tonight" currently not interpreted as a TimePeriod because intersected with
            // today's date, which makes it interpreted as a date+time (will be fixed)
            //"tonight",
            // "this morning" currently not interpreted as a TimePeriod (same reason)
            // "this morning",
            "until dinner",
            "from five to ten",
            // This is currently bugged + interpreted as TimePeriod (same reason, with "this")
            // "this evening after eight thirty",
            "by the end of the day",
        ],
        BuiltinEntityKind::Percentage => {
            &["25%", "twenty percent", "two hundred and fifty percents"]
        }
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["San Francisco", "Los Angeles", "Beijing", "Paris"],
        BuiltinEntityKind::Country => &["France"],
        BuiltinEntityKind::Region => &["California", "Washington"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsable::ParsableEntityKind;
    use snips_nlu_ontology::Language;
    use std::collections::HashSet;

    #[test]
    fn test_entity_examples_should_be_provided_for_all_supported_languages() {
        for entity_kind in BuiltinEntityKind::all() {
            for language in entity_kind.supported_languages() {
                let examples = entity_kind.examples(*language);
                assert!(
                    examples.len() >= 1,
                    "No examples provided for entity '{:?}' in language '{:?}'",
                    entity_kind,
                    language
                )
            }
        }
    }

    #[test]
    fn test_entity_examples_should_not_be_provided_for_non_supported_languages() {
        for entity_kind in BuiltinEntityKind::all() {
            let all_languages: HashSet<&Language> = Language::all().into_iter().collect();
            let supported_languages: HashSet<&Language> =
                entity_kind.supported_languages().into_iter().collect();
            let non_supported_languages: HashSet<&&Language> =
                all_languages.difference(&supported_languages).collect();
            for language in non_supported_languages {
                let examples = entity_kind.examples(**language);
                assert!(
                    examples.is_empty(),
                    "No examples should be provided for entity '{:?}' in language '{:?}', as it is \
                    not supported",
                    entity_kind,
                    language
                )
            }
        }
    }
}
