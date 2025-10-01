use std::{collections::{HashMap, HashSet}};

use crate::cfg::preds;

pub fn reverse_post_order_sort(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut post_order = Vec::new();

    // Helper function for DFS traversal
    fn dfs(node: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, post_order: &mut Vec<String>) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_owned());
        
        // Visit all children (neighbors) first
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs(neighbor, graph, visited, post_order);
            }
        }
        
        // After visiting all children, add the current node to post_order
        post_order.push(node.to_string());
    }

    // Perform DFS for every node to cover all disconnected components
    for key in graph.keys() {
        if !visited.contains(key) {
            dfs(key, graph, &mut visited, &mut post_order);
        }
    }

    // Reverse the post-order result to get reverse post-order
    post_order.reverse();
    post_order
}

pub fn dom(cfg: HashMap<String, Vec<String>>, entry: Option<String>) -> HashMap<String, HashSet<String>> {
    let mut dom = HashMap::new();
    let preds = preds(cfg.clone());
    match entry {
        Some(e) => {
            let mut entry_set = HashSet::new();
            entry_set.insert(e.clone());
            dom.insert(e.clone(), entry_set);
            let mut changed = true;
            while changed {
                changed = false;
                for k in reverse_post_order_sort(&cfg) {
                    let mut new_set = HashSet::new();
                    if let Some(pred) = preds.get(&k) {
                        for i in pred {
                            if let Some(dominators) = dom.get(i) {
                                if new_set.is_empty() {
                                    new_set = dominators.clone();
                                } else {
                                    new_set = new_set.intersection(dominators).cloned().collect();
                                }
                            }
                        }
                    }
                    new_set.insert(k.clone());
                    if let Some(existing_set) = dom.get(&k) {
                        if *existing_set != new_set {
                            changed = true;
                            dom.insert(k.clone(), new_set);
                        }
                    } else {
                        changed = true;
                        dom.insert(k.clone(), new_set);
                    }
                }
            }
            dom
        },
        None => {
            dom
        }
    }
}

pub fn frontier(cfg: HashMap<String, Vec<String>>, dom: HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>>{
    let mut frontier = HashMap::new();
    for (k,v) in dom.iter() {
        let mut front = HashSet::new();
        for i in v {
            if let Some(desc) = cfg.get(i) {
                for j in desc {
                    if !v.contains(j) {
                        front.insert(j.clone());
                    } 
                }
            }
        }
        frontier.insert(k.clone(), front);
    }
    frontier
}

pub fn dfs_chk(cfg: &HashMap<String, Vec<String>>, curr: &str, pass: &str, end: &str, passed: HashSet<String>) -> bool {
    if curr == end && curr != pass {
        print!("Failed: Reached end node {} but it is not the pass node {}\n", end, pass);
        return false;
    }
    if passed.contains(curr) {
        return true;
    }
    if curr == pass {
        return true;
    }
    let mut new_passed = passed.clone();
    new_passed.insert(curr.to_string());
    if let Some(desc) = cfg.get(curr) {
        for i in desc {
            if !dfs_chk(cfg, i, pass, end, new_passed.clone()) {
                print!("Failed: Could not find path from {} to {}\n", curr, i);
                return false;
            }
        }
    }
    true
}


pub fn chk_dom(cfg: &HashMap<String, Vec<String>>, dom: &HashMap<String, HashSet<String>>, entry: Option<String>) -> bool {
    match entry {
        Some(e) => {
            for (k, v) in dom {
                for i in v {
                    let mut passed = HashSet::new();
                    passed.insert(e.clone());
                    if !dfs_chk(cfg, &e, &k, &i, passed) {
                        print!("{}, {}, {}\n", e, k, i);
                        return false;
                    }
                }
            }
            true
        },
        None => dom.is_empty(),
    }
}

