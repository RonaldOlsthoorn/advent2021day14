use std::{io::{BufReader, BufRead}, fs::File, collections::{HashMap}};

#[derive(Debug, Clone)]
struct SimulateResult {
    first_char: Option<char>,
    outcome_tokens: HashMap<String, usize>,
    outcome_distribution: HashMap<char, usize>,
}

impl SimulateResult {

    fn zero() -> Self {
        Self{first_char: None, outcome_tokens: HashMap::new(), outcome_distribution: HashMap::new()}
    }
}

impl std::ops::Add for SimulateResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {

        Self{
            first_char: self.first_char.clone(),
            outcome_tokens: add_tokens(&self.outcome_tokens, &rhs.outcome_tokens),
            outcome_distribution: add_distributions(
                &self.outcome_distribution,
                &rhs.outcome_distribution,
                &rhs.first_char)}
    }
}

fn add_distributions(lhs_dist: &HashMap<char, usize>, rhs_dist: &HashMap<char, usize>, rhs_first_char: &Option<char>) -> HashMap<char, usize> {
    let mut res = HashMap::new();

    lhs_dist.iter().for_each(|(ch, count)|{
        if let Some(other_count) = rhs_dist.get(ch) {
            res.insert(*ch, count + other_count);
        } else {
            res.insert(*ch, *count);
        }
    });

    rhs_dist.iter()
        .filter(|(&k, _)| !lhs_dist.contains_key(&k))
        .for_each(|(&k, &v)| {res.insert(k, v);});

    if let Some(fc) = rhs_first_char {
        *res.get_mut(fc).unwrap() -= 1;
    }
    
    return res;
}

fn add_tokens(lhs_tokens: &HashMap<String, usize>, rhs_tokens: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut res: HashMap<String, usize> = HashMap::new();

    lhs_tokens.iter().for_each(|(token, count)|{
        if let Some(other_count) = rhs_tokens.get(token) {
            res.insert(token.clone(), count + other_count);
        } else {
            res.insert(token.clone(), *count);
        }
    });

    rhs_tokens.iter()
        .filter(|(t, _)| !lhs_tokens.contains_key(*t))
        .for_each(|(k, &v)| {res.insert(k.clone(), v);});

    return res;
}

impl std::ops::Mul<usize> for SimulateResult {
    type Output = SimulateResult;

    fn mul(self, rhs: usize) -> Self::Output {
       mul(&self, rhs)
    }
}

impl std::ops::Mul<SimulateResult> for usize {
    type Output = SimulateResult;

    fn mul(self, rhs: SimulateResult) -> Self::Output {
       mul(&rhs, self)
    }
}

fn mul(dist: &SimulateResult, constant: usize) -> SimulateResult {
    SimulateResult { 
        first_char: dist.first_char.clone(),
        outcome_tokens: mult_tokens(&dist.outcome_tokens, constant),
        outcome_distribution: mult_distribution(&dist.outcome_distribution, constant, &dist.first_char.clone()) }
}

fn mult_distribution(dist: &HashMap<char, usize>, constant: usize, first_char: &Option<char>) -> HashMap<char, usize> {
    let mut res = HashMap::new();

    dist.iter().for_each(|(&k, v)| {res.insert(k, constant * v);});

    if let Some(fc) = first_char {
        *res.get_mut(fc).unwrap() -= constant - 1;
    }

    return res;
}

fn mult_tokens(tokens: &HashMap<String, usize>, constant: usize) -> HashMap<String, usize> {

    let mut res = HashMap::new();
    tokens.iter().for_each(|(k, v)| {res.insert(k.clone(), constant * v);});

    return res;
}

fn simulate_single_token(start_token: &str, rounds: usize, rules: &HashMap::<String, char>) -> SimulateResult {

    let mut polymer = start_token.to_string();

    for _ in 0..rounds {

        let mut polymer_index = 0;

        while polymer_index < polymer.len() - 1 {

            if let Some(insert_char) = rules.get(&polymer[polymer_index..polymer_index + 2]) {
                polymer.insert(polymer_index + 1, *insert_char);
                polymer_index += 1;
            }
            polymer_index += 1;
        }
    }

    let mut token_buckets: HashMap<String, usize> = HashMap::new();

    for index in 0..polymer.len() - 1 {
        if let Some(count) = token_buckets.get_mut(&polymer[index..index+2]) {
            *count += 1;
        } else {
            token_buckets.insert(polymer[index..index+2].to_string(), 1);
        }
    }

    let mut char_buckets: HashMap<char, usize> = HashMap::new();

    polymer.chars().for_each(|c| {
        if let Some(count) = char_buckets.get_mut(&c) {
            *count += 1;
        } else {
            char_buckets.insert(c, 1);
        }
    });

    SimulateResult{
        first_char: Some(start_token.chars().nth(0).unwrap()),
        outcome_tokens: token_buckets,
        outcome_distribution: char_buckets
    }

}

fn simulate_full(start_token: &str, rounds: usize, rules: &HashMap::<String, char>) -> SimulateResult {

    let mut res = simulate_single_token(start_token, 5, rules);
    let mut cache: HashMap<String, SimulateResult> = HashMap::new();

    for _ in (0..rounds - 5).step_by(5) {

        let mut new_res: SimulateResult = SimulateResult::zero();

        for (token, freq) in res.outcome_tokens.iter() {
            if let Some(sim_res) = cache.get(token) {
                new_res = new_res + (*freq * sim_res.clone());
            } else {
                let sim_res = simulate_single_token(&token, 5, rules);
                new_res = new_res + (*freq * sim_res.clone());
                cache.insert(token.clone(), sim_res);
            }
        }

        res = new_res;
    }

    return res;
}

fn main() {

    let mut lines = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap());

    let template = lines.next().unwrap();
    lines.next();

    let mut rules: HashMap::<String, char> = HashMap::new();
    let mut buckets: HashMap<char, usize> = HashMap::new();

    for line in lines {
        rules.insert(line[..2].to_string(), line.chars().last().unwrap());
        buckets.insert(line.chars().last().unwrap(), 0);
    }

    let mut res = SimulateResult::zero();

    for index in 0..template.len()-1 {
        res = res + simulate_full(&template[index..index + 2], 40, &rules);
    }

    let (_, lc) = res.outcome_distribution.iter().min_by(|(_, &v1), (_, &v2)| v1.cmp(&v2)).unwrap();
    let (_, mc) = res.outcome_distribution.iter().max_by(|(_, &v1), (_, &v2)| v1.cmp(&v2)).unwrap();
    
    println!("least common {}, most common {} res {}", lc, mc, mc - lc);


}
