mod aligned_to_0_place_rule;
mod helpers;
mod preserve_lines_before_block_rule;
mod story_rule;

use anyhow::Result;

use crate::config::Config;
use crate::debug;

pub fn format_block(
    input: &str,
    config: &Config,
    debug_enabled: bool,
    file_name: Option<&str>,
) -> Result<String> {
    debug::log(debug_enabled, "Starting Feature block formatting");

    if let Some(file_name) = file_name {
        debug::log(debug_enabled, format!("File name: {file_name}"));
    }

    debug::log_rule(debug_enabled, preserve_lines_before_block_rule::RULE_NAME);
    let mut content = preserve_lines_before_block_rule::apply(input, config)?;

    debug::log_rule(debug_enabled, aligned_to_0_place_rule::RULE_NAME);
    content = aligned_to_0_place_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, story_rule::RULE_NAME);
    content = story_rule::apply(&content, config)?;

    debug::log(debug_enabled, "End Feature block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn formats_feature_block_with_comment_and_story() {
        let input = r#"# this should be preserver
Feature: this is a test feature
unindented user story
"#;

        let expected = r#"# this should be preserver
Feature: this is a test feature

  unindented user story
"#;

        assert_eq!(
            format_block(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_feature_block_with_multiple_story_lines() {
        let input = r#"# this should be preserver
# this should be preserver
Feature: this is a test feature
unindented user story
    badly indented user story
  again a badly indented line
"#;

        let expected = r#"# this should be preserver
# this should be preserver
Feature: this is a test feature

  unindented user story
  badly indented user story
  again a badly indented line
"#;

        assert_eq!(
            format_block(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_badly_indented_feature_block() {
        let input = r#"# this should be preserver
# this should be preserver

    Feature: this is a badly indented feature




unindented user story
    badly indented user story
  again a badly indented line
      again a badly indented line
"#;

        let expected = r#"# this should be preserver
# this should be preserver

Feature: this is a badly indented feature

  unindented user story
  badly indented user story
  again a badly indented line
  again a badly indented line
"#;

        assert_eq!(
            format_block(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn rule_names_match_spec_rule_keyword() {
        assert_eq!(
            preserve_lines_before_block_rule::RULE_NAME,
            "preserve lines before `Feature` block"
        );
        assert_eq!(
            aligned_to_0_place_rule::RULE_NAME,
            "`Feature` block is aligned to 0 place"
        );
        assert_eq!(
            story_rule::RULE_NAME,
            "`Feature` block's story is indented by 1 level"
        );
    }
}