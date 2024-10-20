// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for line in results.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let team_1_name = parts[0].to_string();
        let team_2_name = parts[1].to_string();
        let team_1_score: u32 = parts[2].parse().unwrap();
        let team_2_score: u32 = parts[3].parse().unwrap();

        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
            name: team_1_name.clone(),
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
            name: team_2_name.clone(),
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }

    scores
}

#[derive(Debug, PartialEq, Eq)]
struct Team {
    name: String,
    goals_scored: u32,
    goals_conceded: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Spain,England,2,3\n"
            + "Germany,Spain,2,0\n"
            + "Italy,Germany,2,2\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                &"England".to_string(),
                &"France".to_string(),
                &"Germany".to_string(),
                &"Italy".to_string(),
                &"Spain".to_string(),
            ]
        );
    }

    #[test]
    fn validate_team_scores() {
        let scores = build_scores_table(get_results());

        let team = scores.get(&"England".to_string()).unwrap();
        assert_eq!(team.goals_scored, 7);
        assert_eq!(team.goals_conceded, 4);

        let team = scores.get(&"Spain".to_string()).unwrap();
        assert_eq!(team.goals_scored, 2);
        assert_eq!(team.goals_conceded, 5);
    }
}