mod helpers;
mod input_parameters_rule;
mod keyword_is_capital_rule;
mod step_rule;
mod template_parameters_rule;

use anyhow::Result;

use crate::config::Config;
use crate::debug;

pub fn format_block(input: &str, config: &Config, debug_enabled: bool) -> Result<String> {
    debug::log(debug_enabled, "Starting Step block formatting");

    debug::log_rule(debug_enabled, template_parameters_rule::RULE_NAME);
    let mut content = template_parameters_rule::apply(input, config)?;

    debug::log_rule(debug_enabled, keyword_is_capital_rule::RULE_NAME);
    content = keyword_is_capital_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, input_parameters_rule::RULE_NAME);
    content = input_parameters_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, step_rule::RULE_NAME);
    content = step_rule::apply(&content, config)?;

    debug::log(debug_enabled, "End Step block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn formats_step_block_integration_case() {
        let input = r#"given this must be formatted
But this given should remain as it is
but this but also should be formatter
And here are the "input      parameters" things
And here are they again         " input     params"
and this and too but not the second one
And this remains as it is
When this is just fine
and this is template <parameter>
but this one is malformed < para>
and this one more <      para        >
and this one is not
But this is a good but
then this then should be formatter
But not the second one
And this line has multiple <   input >         <     parameters     >
"#;

        let expected = r#"Given this must be formatted
But this given should remain as it is
But this but also should be formatter
And here are the "input      parameters" things
And here are they again " input     params"
And this and too but not the second one
And this remains as it is
When this is just fine
And this is template <parameter>
But this one is malformed <para>
And this one more <para>
And this one is not
But this is a good but
Then this then should be formatter
But not the second one
And this line has multiple <input> <parameters>
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn rule_names_match_spec_rule_keyword() {
        assert_eq!(
            keyword_is_capital_rule::RULE_NAME,
            "Step definition keywords start with capital letters"
        );
        assert_eq!(step_rule::RULE_NAME, "one space between words in steps");
        assert_eq!(
            input_parameters_rule::RULE_NAME,
            "In Step definitions the content between `\"` and `\"` remains as it is"
        );
        assert_eq!(
            template_parameters_rule::RULE_NAME,
            "Template parameters in Step definitionsl"
        );
    }
}