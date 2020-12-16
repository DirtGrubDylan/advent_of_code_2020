use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
struct TicketRules {
    rule_names_and_ranges: HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl TicketRules {
    fn new() -> TicketRules {
        TicketRules {
            rule_names_and_ranges: HashMap::new(),
        }
    }

    fn add_rule(&mut self, info: &str) {
        let split_info: Vec<&str> = info.split(": ").collect();

        let field_name = split_info.get(0).expect(info).to_string();

        let all_range_info_split: Vec<&str> =
            split_info.get(1).expect(info).split(" or ").collect();

        let first_range_info: Vec<usize> = all_range_info_split
            .get(0)
            .expect(info)
            .split('-')
            .map(|s| s.parse().expect(info))
            .collect();

        let first_range =
            *first_range_info.get(0).expect(info)..=*first_range_info.get(1).expect(info);

        let second_range_info: Vec<usize> = all_range_info_split
            .get(1)
            .expect(info)
            .split('-')
            .map(|s| s.parse().expect(info))
            .collect();

        let second_range =
            *second_range_info.get(0).expect(info)..=*second_range_info.get(1).expect(info);

        self.rule_names_and_ranges
            .insert(field_name, (first_range, second_range));
    }

    fn follows_rules(&self, ticket: &Ticket) -> (bool, Vec<usize>) {
        let mut follows_rules = true;
        let mut invalid_values = Vec::new();

        for field_value in &ticket.fields {
            if !self.field_value_is_in_any_range(field_value) {
                follows_rules = false;
                invalid_values.push(*field_value);
            }
        }

        (follows_rules, invalid_values)
    }

    fn field_value_is_in_any_range(&self, field_value: &usize) -> bool {
        let mut field_is_is_any = false;

        for (field_value_range_1, field_value_range_2) in self.rule_names_and_ranges.values() {
            let field_value_in_ranges = field_value_range_1.contains(field_value)
                || field_value_range_2.contains(field_value);

            if field_value_in_ranges {
                field_is_is_any = true;

                break;
            }
        }

        field_is_is_any
    }

    fn fields_that_do_not_match(&self, field_value: usize) -> HashSet<String> {
        let mut fields_that_do_not_match = HashSet::new();

        for (field_name, (field_value_range_1, field_value_range_2)) in
            self.rule_names_and_ranges.iter()
        {
            let field_value_in_ranges = field_value_range_1.contains(&field_value)
                || field_value_range_2.contains(&field_value);

            if !field_value_in_ranges {
                fields_that_do_not_match.insert(field_name.clone());
            }
        }

        fields_that_do_not_match
    }

    fn get_field_names(&self) -> HashSet<String> {
        self.rule_names_and_ranges
            .keys()
            .map(|s| s.clone())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn new(info: &str) -> Ticket {
        if info.is_empty() {
            Ticket { fields: Vec::new() }
        } else {
            Ticket {
                fields: info.split(',').map(|s| s.parse().expect(info)).collect(),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TicketScanner {
    rules: TicketRules,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl TicketScanner {
    pub fn new(info: &[String]) -> TicketScanner {
        let mut in_rules_section = true;
        let mut in_your_ticket_section = false;
        let mut in_nearby_tickets_section = false;

        let mut rules = TicketRules::new();
        let mut your_ticket = Ticket::new("");
        let mut nearby_tickets = Vec::new();

        for line in info {
            if line.is_empty() {
                continue;
            } else if line == "your ticket:" {
                in_rules_section = false;
                in_your_ticket_section = true;
                in_nearby_tickets_section = false;

                continue;
            } else if line == "nearby tickets:" {
                in_rules_section = false;
                in_your_ticket_section = false;
                in_nearby_tickets_section = true;

                continue;
            }

            if in_rules_section {
                rules.add_rule(line);
            } else if in_your_ticket_section {
                your_ticket = Ticket::new(line);
            } else if in_nearby_tickets_section {
                nearby_tickets.push(Ticket::new(line));
            }
        }

        TicketScanner {
            rules,
            your_ticket,
            nearby_tickets,
        }
    }

    pub fn get_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| self.rules.follows_rules(ticket).1)
            .sum()
    }

    pub fn get_your_ticket_fields(&self) -> HashMap<String, usize> {
        let mut possible_fields: Vec<HashSet<String>> = self
            .your_ticket
            .fields
            .iter()
            .map(|_| self.rules.get_field_names())
            .collect();

        for ticket in self.nearby_tickets.iter() {
            if !self.rules.follows_rules(ticket).0 {
                continue;
            }

            for (index, field_value) in ticket.fields.iter().enumerate() {
                let possible_fields_for_index = possible_fields.get_mut(index).unwrap();

                for field_name in self.rules.fields_that_do_not_match(*field_value).iter() {
                    possible_fields_for_index.remove(field_name);
                }
            }
        }

        let mut possible_fields_clone = possible_fields.clone();

        possible_fields_clone.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        for possible_fields_for_index in possible_fields_clone {
            for fields in possible_fields.iter_mut() {
                if fields.len() == 1 {
                    continue;
                }

                for field in possible_fields_for_index.iter() {
                    fields.remove(field);
                }
            }
        }

        let mut result = HashMap::new();

        for (index, possible_field) in possible_fields.iter().enumerate() {
            let only_possible_field = possible_field
                .iter()
                .map(|s| s.clone())
                .collect::<Vec<String>>()
                .get(0)
                .unwrap()
                .clone();

            let value = self.your_ticket.fields.get(index).unwrap();

            result.insert(only_possible_field, *value);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULE_DATA: &str = "row test: 6-11 or 33-44";
    const TEST_GOOD_TICKET_DATA: &str = "7,11,33";
    const TEST_BAD_TICKET_DATA: &str = "40,4,50";

    const TEST_SCANNER_DATA: [&str; 12] = [
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "your seat: 13-40 or 45-50",
        "",
        "your ticket:",
        "7,1,14",
        "",
        "nearby tickets:",
        "7,3,47",
        "40,4,50",
        "55,2,20",
        "38,6,12",
    ];

    const TEST_SCANNER_DATA_2: [&str; 11] = [
        "class: 0-1 or 4-19",
        "row: 0-5 or 8-19",
        "seat: 0-13 or 16-19",
        "",
        "your ticket:",
        "11,12,13",
        "",
        "nearby tickets:",
        "9,3,18",
        "1,15,5",
        "14,5,9",
    ];

    #[test]
    fn test_ticket_rules_add_rules() {
        let mut rules = TicketRules::new();

        rules.add_rule(TEST_RULE_DATA);

        let expected = TicketRules {
            rule_names_and_ranges: vec![(String::from("row test"), ((6..=11), (33..=44)))]
                .into_iter()
                .collect(),
        };

        assert_eq!(rules, expected);
    }

    #[test]
    fn test_ticket_rules_follows_rules() {
        let good_ticket = Ticket::new(TEST_GOOD_TICKET_DATA);
        let bad_ticket = Ticket::new(TEST_BAD_TICKET_DATA);

        let mut rules = TicketRules::new();

        rules.add_rule(TEST_RULE_DATA);

        let result_1 = rules.follows_rules(&good_ticket);
        let result_2 = rules.follows_rules(&bad_ticket);

        let expected_1 = (true, Vec::new());
        let expected_2 = (false, vec![4, 50]);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_ticket_new() {
        let result = Ticket::new(TEST_GOOD_TICKET_DATA);

        let expected = Ticket {
            fields: vec![7, 11, 33],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ticket_scanner_new() {
        let input: Vec<String> = TEST_SCANNER_DATA.iter().map(|s| s.to_string()).collect();

        let result = TicketScanner::new(&input);

        let expected = TicketScanner {
            rules: TicketRules {
                rule_names_and_ranges: vec![
                    (String::from("class"), ((1..=3), (5..=7))),
                    (String::from("row"), ((6..=11), (33..=44))),
                    (String::from("your seat"), ((13..=40), (45..=50))),
                ]
                .into_iter()
                .collect(),
            },
            your_ticket: Ticket {
                fields: vec![7, 1, 14],
            },
            nearby_tickets: vec![
                Ticket {
                    fields: vec![7, 3, 47],
                },
                Ticket {
                    fields: vec![40, 4, 50],
                },
                Ticket {
                    fields: vec![55, 2, 20],
                },
                Ticket {
                    fields: vec![38, 6, 12],
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ticket_scanner_get_error_rate() {
        let input: Vec<String> = TEST_SCANNER_DATA.iter().map(|s| s.to_string()).collect();

        let scanner = TicketScanner::new(&input);

        let result = scanner.get_error_rate();

        let expected = 71;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ticket_scanner_get_your_ticket_fields() {
        let input: Vec<String> = TEST_SCANNER_DATA_2.iter().map(|s| s.to_string()).collect();

        let scanner = TicketScanner::new(&input);

        let result = scanner.get_your_ticket_fields();

        let expected = vec![
            (String::from("class"), 11),
            (String::from("row"), 12),
            (String::from("seat"), 13),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }
}
