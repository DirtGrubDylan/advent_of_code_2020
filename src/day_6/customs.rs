use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Person {
    yes_answers: HashSet<char>,
}

impl Person {
    fn new(answers: &str) -> Person {
        Person {
            yes_answers: answers.chars().collect(),
        }
    }

    fn get_yes_answers(&self) -> &HashSet<char> {
        &self.yes_answers
    }
}

#[derive(Debug, PartialEq)]
pub struct Group {
    people: Vec<Person>,
}

impl Group {
    pub fn new(group_answers: &[String]) -> Group {
        Group {
            people: group_answers
                .iter()
                .map(|person_answers| Person::new(&person_answers))
                .collect(),
        }
    }

    pub fn unique_yes_answers(&self) -> HashSet<char> {
        self.people
            .iter()
            .flat_map(|person| person.get_yes_answers())
            .cloned()
            .collect()
    }

    pub fn answers_that_everyone_said_yes(&self) -> HashSet<char> {
        self.people
            .iter()
            .fold(self.unique_yes_answers(), |acc, person| {
                acc.intersection(person.get_yes_answers())
                    .cloned()
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PERSON_DATA: &str = "abcx";
    const TEST_GROUP_DATA: [&str; 3] = ["abcx", "abcy", "abcz"];

    #[test]
    fn test_person_new() {
        let result = Person::new(TEST_PERSON_DATA);

        let expected = Person {
            yes_answers: vec!['a', 'b', 'c', 'x'].into_iter().collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_new() {
        let input: Vec<String> = TEST_GROUP_DATA.iter().map(|s| String::from(*s)).collect();

        let result = Group::new(&input);

        let expected = Group {
            people: vec![
                Person {
                    yes_answers: vec!['a', 'b', 'c', 'x'].into_iter().collect(),
                },
                Person {
                    yes_answers: vec!['a', 'b', 'c', 'y'].into_iter().collect(),
                },
                Person {
                    yes_answers: vec!['a', 'b', 'c', 'z'].into_iter().collect(),
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_unique_yes_answers() {
        let input: Vec<String> = TEST_GROUP_DATA.iter().map(|s| String::from(*s)).collect();

        let group = Group::new(&input);

        let result = group.unique_yes_answers();

        let expected = vec!['a', 'b', 'c', 'x', 'y', 'z'].into_iter().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_answers_that_everyone_said_yes() {
        let input: Vec<String> = TEST_GROUP_DATA.iter().map(|s| String::from(*s)).collect();

        let group = Group::new(&input);

        let result = group.answers_that_everyone_said_yes();

        let expected = vec!['a', 'b', 'c'].into_iter().collect();

        assert_eq!(result, expected);
    }
}
