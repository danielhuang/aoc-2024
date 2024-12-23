use aoc_2024::*;

fn expand(x: &String, edges: &DefaultHashMap<String, Vec<String>>) -> HashSet<String> {
    let others = edges[x].clone();
    let mut cur = HashSet::new();
    cur.insert(x.clone());
    for other in others {
        if cur.cii().all(|e| edges[e].contains(&other)) {
            cur.insert(other);
        }
    }
    cur
}

fn main() {
    let input = load_input();

    let mut all = set([]);

    let mut edges = defaultdict(vec![]);

    for line in input.lines() {
        let [a, b] = line.alphanumeric_words().ca();
        all.insert(a.clone());
        all.insert(b.clone());
        edges[a.clone()].push(b.clone());
        edges[b.clone()].push(a.clone());
    }

    let mut triples = set([]);

    for a in all.cii() {
        if a.starts_with('t') {
            for (b, c) in edges[&a].cii().tuple_combinations() {
                if edges[b.clone()].contains(&c.clone()) {
                    triples.insert([a.clone(), b.clone(), c.clone()].cii().sorted().cv());
                }
            }
        }
    }

    cp(triples.len());

    let mut best = HashSet::new();

    // TODO: may not always find correct answer
    for node in all {
        let expanded = expand(&node, &edges);
        if expanded.len() > best.len() {
            best = expanded;
        }
    }

    cp(best.cii().sorted().cv().join(","));
}
