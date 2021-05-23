mod util;
use language_tags::LanguageTag;
use util::*;

#[test]
fn language_tags() -> TestResult {
    test_default_generated_schema::<LanguageTag>("language_tags")
}
