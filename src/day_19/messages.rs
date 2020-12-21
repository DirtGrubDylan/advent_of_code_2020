use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Rule {
    strings: Vec<String>,
}

impl Rule {
    fn new(info: &[String]) -> Rule {
        Rule {
            strings: info.to_vec(),
        }
    }

    fn is_valid(&self, string_to_check: &str) -> bool {
        self.strings.iter().any(|info| string_to_check.eq(info))
    }
}

#[derive(Debug, PartialEq)]
pub struct RulesAndMessages {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl RulesAndMessages {
    pub fn new(info: &[String]) -> RulesAndMessages {
        let info_split: Vec<&[String]> = info.split(|s| s.is_empty()).into_iter().collect();

        let (rules_info, messages) = (
            info_split.get(0).expect("No Rules Info"),
            info_split.get(1).expect("No Messages").to_vec(),
        );

        let rules = Self::build_rules(rules_info);

        RulesAndMessages { rules, messages }
    }

    pub fn number_of_valid_messages_for_rule(&self, rule_index: usize) -> usize {
        if let Some(rule) = self.rules.get(&rule_index) {
            self.messages
                .iter()
                .filter(|message| rule.is_valid(message))
                .count()
        } else {
            0
        }
    }

    pub fn updated_number_of_valid_messages_for_rule0(&self) -> usize {
        self.messages
            .iter()
            .filter(|message| self.updated_message_is_valid_for_rule0(message))
            .count()
    }

    fn updated_message_is_valid_for_rule0(&self, message: &str) -> bool {
        let rule42 = self.rules.get(&42).expect("No Rule 42!");
        let mut search_string_for_42s = message.to_string();
        let mut times_42_matched = 0;

        let rule31 = self.rules.get(&31).expect("No Rule 31!");
        let mut search_string_for_31s = String::new();
        let mut times_31_matched = 0;

        loop {
            let mut found_42_match = false;

            for pattern in rule42.strings.iter() {
                let starts_with = search_string_for_42s.starts_with(pattern);

                if starts_with {
                    times_42_matched += 1;

                    search_string_for_42s = search_string_for_42s.replacen(pattern, "", 1);

                    found_42_match = true;

                    break;
                }
            }

            if !found_42_match && (2 <= times_42_matched) {
                search_string_for_31s = search_string_for_42s;

                break;
            } else if !found_42_match {
                break;
            }
        }

        loop {
            let mut found_31_match = false;

            for pattern in rule31.strings.iter() {
                let starts_with = search_string_for_31s.starts_with(pattern);

                if starts_with {
                    times_31_matched += 1;

                    search_string_for_31s = search_string_for_31s.replacen(pattern, "", 1);

                    found_31_match = true;

                    break;
                }
            }

            if !found_31_match {
                break;
            }
        }

        1 <= (times_42_matched - times_31_matched)
            && (times_31_matched != 0)
            && search_string_for_31s.is_empty()
    }

    fn build_rules(rules_info: &[String]) -> HashMap<usize, Rule> {
        let mut rules = HashMap::new();
        let currated_info = Self::currate_rules_info(rules_info);

        Self::insert_rule_for_index(&mut rules, 0, &currated_info);

        rules
    }

    fn currate_rules_info(rules_info: &[String]) -> HashMap<usize, String> {
        rules_info
            .iter()
            .map(|info| {
                let mut split_info = info.split(": ");

                let index: usize = split_info.nth(0).expect(info).parse().expect(info);

                let mut new_info = split_info.nth(0).expect(info).replace("\"", "");

                new_info.push(' ');

                (index, new_info.clone())
            })
            .collect()
    }

    fn insert_rule_for_index(
        rules: &mut HashMap<usize, Rule>,
        index: usize,
        rules_info: &HashMap<usize, String>,
    ) {
        let mut results: Vec<String> = Vec::new();
        let mut result_builders: Vec<String> = vec![String::new()];
        let mut index_builder = String::new();

        for c in rules_info.get(&index).expect("No Rule Found").chars() {
            match c {
                next_index_digit if c.is_digit(10) => {
                    index_builder.push(next_index_digit);
                }
                ' ' if !index_builder.is_empty() => {
                    let next_index = index_builder.parse().expect("Cannot Parse Index");

                    if !rules.contains_key(&next_index) {
                        Self::insert_rule_for_index(rules, next_index, rules_info);
                    }

                    let next_rule = rules.get(&next_index).unwrap();

                    let mut temp_builders = Vec::new();

                    for result_builder in result_builders.iter() {
                        for rule_string in next_rule.strings.iter() {
                            temp_builders.push(result_builder.clone() + rule_string);
                        }
                    }

                    result_builders = temp_builders;
                    index_builder = String::new();
                }
                '|' => {
                    results.extend_from_slice(&result_builders);
                    result_builders = vec![String::new()];
                }
                next_rule_value if c.is_alphabetic() => {
                    results.push(next_rule_value.to_string());
                }
                ' ' if index_builder.is_empty() => {}
                _ => panic!("Unknown Character for Rule: {}", index),
            }
        }

        results.extend_from_slice(&result_builders);

        results.retain(|result| !result.is_empty());

        rules.insert(index, Rule::new(&results));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 12] = [
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "0: 4 1 5",
        "3: 4 5 | 5 4",
        "4: \"a\"",
        "5: \"b\"",
        "",
        "ababbb",
        "bababa",
        "abbbab",
        "aaabbb",
        "aaaabbb",
    ];

    const TEST_DATA_2: [&str; 47] = [
        "42: 9 14 | 10 1",
        "9: 14 27 | 1 26",
        "10: 23 14 | 28 1",
        "1: \"a\"",
        "11: 42 31",
        "5: 1 14 | 15 1",
        "19: 14 1 | 14 14",
        "12: 24 14 | 19 1",
        "16: 15 1 | 14 14",
        "31: 14 17 | 1 13",
        "6: 14 14 | 1 14",
        "2: 1 24 | 14 4",
        "0: 8 11",
        "13: 14 3 | 1 12",
        "15: 1 | 14",
        "17: 14 2 | 1 7",
        "23: 25 1 | 22 14",
        "28: 16 1",
        "4: 1 1",
        "20: 14 14 | 1 15",
        "3: 5 14 | 16 1",
        "27: 1 6 | 14 18",
        "14: \"b\"",
        "21: 14 1 | 1 14",
        "25: 1 1 | 1 14",
        "22: 14 14",
        "8: 42",
        "26: 14 22 | 1 20",
        "18: 15 15",
        "7: 14 5 | 1 21",
        "24: 14 1",
        "",
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
        "bbabbbbaabaabba",
        "babbbbaabbbbbabbbbbbaabaaabaaa",
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
        "bbbbbbbaaaabbbbaaabbabaaa",
        "bbbababbbbaaaaaaaabbababaaababaabab",
        "ababaaaaaabaaab",
        "ababaaaaabbbaba",
        "baabbaaaabbaaaababbaababb",
        "abbbbabbbbaaaababbbbbbaaaababb",
        "aaaaabbaabaaaaababaa",
        "aaaabbaaaabbaaa",
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
        "babaaabbbaaabaababbaabababaaab",
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];

    #[test]
    fn test_rules_and_messages_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let result = RulesAndMessages::new(&input);

        let expected_rules: HashMap<usize, Rule> = vec![
            (
                0,
                Rule::new(&[
                    "aaaabb".to_string(),
                    "aaabab".to_string(),
                    "abbabb".to_string(),
                    "abbbab".to_string(),
                    "aabaab".to_string(),
                    "aabbbb".to_string(),
                    "abaaab".to_string(),
                    "ababbb".to_string(),
                ]),
            ),
            (
                1,
                Rule::new(&[
                    "aaab".to_string(),
                    "aaba".to_string(),
                    "bbab".to_string(),
                    "bbba".to_string(),
                    "abaa".to_string(),
                    "abbb".to_string(),
                    "baaa".to_string(),
                    "babb".to_string(),
                ]),
            ),
            (2, Rule::new(&["aa".to_string(), "bb".to_string()])),
            (3, Rule::new(&["ab".to_string(), "ba".to_string()])),
            (4, Rule::new(&["a".to_string()])),
            (5, Rule::new(&["b".to_string()])),
        ]
        .into_iter()
        .collect();

        let expected = RulesAndMessages {
            rules: expected_rules,
            messages: vec![
                "ababbb".to_string(),
                "bababa".to_string(),
                "abbbab".to_string(),
                "aaabbb".to_string(),
                "aaaabbb".to_string(),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rules_and_message_number_of_valid_messages_for_rule_0() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let rules_and_messages = RulesAndMessages::new(&input);

        let result = rules_and_messages.number_of_valid_messages_for_rule(0);

        let expected = 2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rules_and_message_number_of_valid_messages_for_rule_0_large() {
        let input: Vec<String> = TEST_DATA_2.iter().map(|s| s.to_string()).collect();

        let rules_and_messages = RulesAndMessages::new(&input);

        let result = rules_and_messages.number_of_valid_messages_for_rule(0);

        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_updated_number_of_valid_messages_for_rule0() {
        let input: Vec<String> = TEST_DATA_2.iter().map(|s| s.to_string()).collect();

        let rules_and_messages = RulesAndMessages::new(&input);

        let result = rules_and_messages.updated_number_of_valid_messages_for_rule0();

        let expected = 12;

        assert_eq!(result, expected);
    }
}
