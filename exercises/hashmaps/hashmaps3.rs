// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores = HashMap::new();
    for result in results.lines() {
        let (team_1, team_2) = parse_teams(result);
        update_or_insert(&mut scores, team_1);
        update_or_insert(&mut scores, team_2);
    }
    scores
}

fn parse_teams(result: &str) -> (Team, Team) {
    let result = result.split(',').collect::<Vec<&str>>();
    let team_1_name = result[0].to_string();
    let team_1_score = result[2].parse().unwrap();
    let team_2_name = result[1].to_string();
    let team_2_score = result[3].parse().unwrap();
    let team_1 = Team {
        name: team_1_name,
        goals_scored: team_1_score,
        goals_conceded: team_2_score,
    };
    let team_2 = Team {
        name: team_2_name,
        goals_scored: team_2_score,
        goals_conceded: team_1_score,
    };
    (team_1, team_2)
}

fn update_or_insert(scores: &mut HashMap<String, Team>, team: Team) {
    scores
        .entry(team.name.clone())
        .and_modify(|t| {
            t.goals_scored += team.goals_scored;
            t.goals_conceded += team.goals_conceded;
        })
        .or_insert(team);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());
        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
