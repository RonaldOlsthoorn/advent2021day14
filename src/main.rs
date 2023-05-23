use std::{io::{BufReader, BufRead}, fs::File, collections::{HashMap, VecDeque, BTreeMap}};

#[derive(Debug, Clone)]
struct WalkState {
    chunk: String,
    depth: u8
}
fn main() {

    let mut lines = BufReader::new(File::open("test-input.txt").unwrap()).lines().map(|l| l.unwrap());

    let template = lines.next().unwrap();
    lines.next();

    let mut rules: HashMap::<String, char> = HashMap::new();

    for line in lines {
        rules.insert(line[..2].to_string(), line.chars().last().unwrap());
    }

    let mut rule_implications: HashMap<String, (Vec<String>, char)> = HashMap::new();
    let mut buckets: HashMap<char, usize> = HashMap::new();

    for rule in rules.iter() {

        let mut res = String::new();
        res.push(rule.0.chars().nth(0).unwrap());
        res.push(*rule.1);

        let mut children = Vec::new();

        if rules.contains_key(&res) {
            children.push(res);

        }

        res = String::new();
        res.push(*rule.1);
        res.push(rule.0.chars().nth(1).unwrap());

        if rules.contains_key(&res) {
            children.push(res);
        }

        rule_implications.insert(rule.0.to_string(), (children, *rule.1));
        buckets.insert(*rule.1, 0);
    }

    for (ch, count) in buckets.iter_mut() {
        *count = template.chars().filter(|c| c == ch).count();
    }

    let mut stack: VecDeque<WalkState> = VecDeque::new();

    for i in 0..template.len() - 1 {
        stack.push_back(WalkState{chunk: template[i..i+2].to_owned(), depth: 0});
    }

    while !stack.is_empty() {

        let state = stack.pop_front().unwrap();

        if state.depth == 40 {
            continue;
        }

        let rule_implication = rule_implications.get(&state.chunk).unwrap();
        *buckets.get_mut(&rule_implication.1).unwrap() += 1;
        
        for child in &rule_implications[&state.chunk].0 {
            stack.push_front(WalkState{chunk: child.clone(), depth: state.depth + 1});
        }
    }

    let mut ordered_buckets: BTreeMap<usize, char> = BTreeMap::new();

    buckets.iter().for_each(|(&k, &v)| { ordered_buckets.insert(v, k); });

    println!("least common {}, most common {} res {}",
        ordered_buckets.iter().next().unwrap().0,
        ordered_buckets.iter().last().unwrap().0,
        ordered_buckets.iter().last().unwrap().0 - ordered_buckets.iter().next().unwrap().0);


}
