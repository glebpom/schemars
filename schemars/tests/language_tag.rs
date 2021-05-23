mod util;
use language_tag::LanguageTag;
use util::*;

#[test]
fn language_tag() -> TestResult {
    test_default_generated_schema::<LanguageTag>("language_tag")
}
