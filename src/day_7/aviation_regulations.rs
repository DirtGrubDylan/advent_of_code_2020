use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
struct BagRule {
    bag_description: String,
    bags_containing: HashSet<(String, u64)>,
    bags_contained_by: HashSet<String>,
}

impl BagRule {
    fn new(info: &str) -> BagRule {
        let curated_info = info
            .replace(" bags", "")
            .replace(" bag", "")
            .replace(".", "")
            .replace("no other", "0 ");

        let description_err = format!("No Description: {}", info);
        let containing_err = format!("No Containing Info: {}", info);

        let split_info: Vec<&str> = curated_info.split(" contain ").collect();
        let description = split_info
            .get(0)
            .expect(&description_err)
            .to_owned()
            .to_string();

        let mut containing = HashSet::new();

        for containing_info in split_info.get(1).expect(&containing_err).split(", ") {
            let containing_amt_err = format!("No Containing Amount: {}", containing_info);

            let first_space = containing_info.find(" ").unwrap();

            let temp: (&str, &str) = containing_info.split_at(first_space);

            let containing_amt = temp
                .0
                .trim_start()
                .trim_end()
                .parse()
                .expect(&containing_amt_err);

            if containing_amt != 0 {
                let containing_desc = temp.1.trim_start().trim_end().to_string();

                containing.insert((containing_desc, containing_amt));
            }
        }

        BagRule {
            bag_description: description,
            bags_containing: containing,
            bags_contained_by: HashSet::new(),
        }
    }

    fn get_description(&self) -> String {
        self.bag_description.clone()
    }

    fn get_bags_contained_by(&self) -> HashSet<(String, u64)> {
        self.bags_containing.clone()
    }

    fn get_bags_containing(&self) -> &HashSet<String> {
        &self.bags_contained_by
    }

    fn add_bag_containing(&mut self, bag_description_containing_self: &str) {
        self.bags_contained_by
            .insert(String::from(bag_description_containing_self));
    }
}

#[derive(Debug, PartialEq)]
pub struct LuggageRegulations {
    rules: HashMap<String, BagRule>,
}

impl LuggageRegulations {
    pub fn new(rules_info: &[String]) -> LuggageRegulations {
        let mut rules = HashMap::new();

        for rule_info in rules_info.iter() {
            let bag = BagRule::new(rule_info);

            rules.insert(bag.get_description(), bag);
        }

        let descriptions: Vec<String> = rules.keys().cloned().collect();

        for description in descriptions.iter() {
            let bags_contained_by = rules.get(description).unwrap().get_bags_contained_by();

            for (contained_desc, _) in bags_contained_by.iter() {
                let contained_bag = rules.get_mut(contained_desc).unwrap();

                contained_bag.add_bag_containing(description);
            }
        }

        LuggageRegulations { rules: rules }
    }

    pub fn bags_that_can_contain(&self, bag_description: &str) -> HashSet<String> {
        let mut bags_that_can_contain = HashSet::new();
        let mut work_queue = VecDeque::new();

        work_queue.push_back(String::from(bag_description));

        while let Some(current_desc) = work_queue.pop_front() {
            let rules_get_err = format!("Couldn't Find Desc: {}", current_desc);

            if bags_that_can_contain.contains(&current_desc) {
                continue;
            }

            let current_bag = self.rules.get(&current_desc).expect(&rules_get_err);

            for bag_desc_containing_current in current_bag.get_bags_containing().iter() {
                work_queue.push_back(bag_desc_containing_current.to_string());
            }

            if current_desc != bag_description {
                bags_that_can_contain.insert(current_desc);
            }
        }

        bags_that_can_contain
    }

    pub fn number_of_bags_to_fill(&self, bag_description: &str) -> u64 {
        let mut result = 1;

        let bags_contained_by = self
            .rules
            .get(bag_description)
            .unwrap()
            .get_bags_contained_by();

        for (contained_desc, contained_amount) in bags_contained_by.iter() {
            result += contained_amount * self.number_of_bags_to_fill(contained_desc);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BAG_RULE_DATA_1: &str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    const TEST_BAG_RULE_DATA_2: &str = "faded blue bags contain no other bags.";

    const TEST_LUGGAGE_REGULATIONS_DATA: [&str; 9] = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    const TEST_LUGGAGE_REGULATIONS_DATA_2: [&str; 7] = [
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];

    #[test]
    fn test_bag_rule_new() {
        let result_1 = BagRule::new(TEST_BAG_RULE_DATA_1);
        let result_2 = BagRule::new(TEST_BAG_RULE_DATA_2);

        let expected_1 = BagRule {
            bag_description: String::from("light red"),
            bags_containing: vec![
                (String::from("bright white"), 1),
                (String::from("muted yellow"), 2),
            ]
            .into_iter()
            .collect(),
            bags_contained_by: HashSet::new(),
        };
        let expected_2 = BagRule {
            bag_description: String::from("faded blue"),
            bags_containing: HashSet::new(),
            bags_contained_by: HashSet::new(),
        };

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_luggage_regulations_new() {
        let input: Vec<String> = TEST_LUGGAGE_REGULATIONS_DATA
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let result = LuggageRegulations::new(&input);

        let expected = LuggageRegulations {
            rules: vec![
                (
                    String::from("light red"),
                    BagRule {
                        bag_description: String::from("light red"),
                        bags_containing: vec![
                            (String::from("bright white"), 1),
                            (String::from("muted yellow"), 2),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: HashSet::new(),
                    },
                ),
                (
                    String::from("dark orange"),
                    BagRule {
                        bag_description: String::from("dark orange"),
                        bags_containing: vec![
                            (String::from("bright white"), 3),
                            (String::from("muted yellow"), 4),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: HashSet::new(),
                    },
                ),
                (
                    String::from("bright white"),
                    BagRule {
                        bag_description: String::from("bright white"),
                        bags_containing: vec![(String::from("shiny gold"), 1)]
                            .into_iter()
                            .collect(),
                        bags_contained_by: vec![
                            String::from("light red"),
                            String::from("dark orange"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
                (
                    String::from("muted yellow"),
                    BagRule {
                        bag_description: String::from("muted yellow"),
                        bags_containing: vec![
                            (String::from("shiny gold"), 2),
                            (String::from("faded blue"), 9),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: vec![
                            String::from("light red"),
                            String::from("dark orange"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
                (
                    String::from("shiny gold"),
                    BagRule {
                        bag_description: String::from("shiny gold"),
                        bags_containing: vec![
                            (String::from("dark olive"), 1),
                            (String::from("vibrant plum"), 2),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: vec![
                            String::from("bright white"),
                            String::from("muted yellow"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
                (
                    String::from("dark olive"),
                    BagRule {
                        bag_description: String::from("dark olive"),
                        bags_containing: vec![
                            (String::from("faded blue"), 3),
                            (String::from("dotted black"), 4),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: vec![String::from("shiny gold")].into_iter().collect(),
                    },
                ),
                (
                    String::from("vibrant plum"),
                    BagRule {
                        bag_description: String::from("vibrant plum"),
                        bags_containing: vec![
                            (String::from("faded blue"), 5),
                            (String::from("dotted black"), 6),
                        ]
                        .into_iter()
                        .collect(),
                        bags_contained_by: vec![String::from("shiny gold")].into_iter().collect(),
                    },
                ),
                (
                    String::from("faded blue"),
                    BagRule {
                        bag_description: String::from("faded blue"),
                        bags_containing: HashSet::new(),
                        bags_contained_by: vec![
                            String::from("muted yellow"),
                            String::from("dark olive"),
                            String::from("vibrant plum"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
                (
                    String::from("dotted black"),
                    BagRule {
                        bag_description: String::from("dotted black"),
                        bags_containing: HashSet::new(),
                        bags_contained_by: vec![
                            String::from("dark olive"),
                            String::from("vibrant plum"),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_luggage_regulations_bags_that_can_contain() {
        let input: Vec<String> = TEST_LUGGAGE_REGULATIONS_DATA
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let luggage_regulations = LuggageRegulations::new(&input);

        let result = luggage_regulations.bags_that_can_contain("shiny gold");

        let expected = vec![
            String::from("bright white"),
            String::from("dark orange"),
            String::from("muted yellow"),
            String::from("light red"),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_luggage_regulations_number_of_bags_to_fill() {
        let input: Vec<String> = TEST_LUGGAGE_REGULATIONS_DATA_2
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let luggage_regulations = LuggageRegulations::new(&input);

        let result = luggage_regulations.number_of_bags_to_fill("shiny gold");

        let expected = 127;

        assert_eq!(result, expected);
    }
}
