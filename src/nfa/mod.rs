//! This module is for constructing and executing
//! Nondeterministic Finite Automata

use std::{
    collections::{HashMap, HashSet, LinkedList},
    error::Error,
    hash::Hash,
};

#[cfg(test)]
use queryengine::QueryEngine;
use serde::{Deserialize, Serialize};

use crate::{languages::clike::Clike, regex2nfa::build_nfa};
type Atom = char;

pub mod matcher;
pub mod queryengine;
pub mod replacer;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum TransitionType {
    Epsilon,
    Alpha(Atom),
    Range(String),
    NegativeRange(String),
    QuerySetRange(String),
    Open(usize),
    Close(usize),
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum NodeType {
    Normal,
    Accept,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Transition {
    kind: TransitionType,
    dest: NodePointer,
}

impl Transition {
    fn new(kind: TransitionType, dest: NodePointer) -> Self {
        Self { kind, dest }
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodePointer {
    id: usize,
}

impl NodePointer {
    fn new(id: usize) -> Self {
        Self { id }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    transitions: Vec<Transition>,
    nt: NodeType,
}
impl Node {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
            nt: NodeType::Normal,
        }
    }
    fn set_type(&mut self, nt: NodeType) {
        self.nt = nt;
    }
}

/// Represents a non-deterministic finite automaton
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nfa {
    nodes: Vec<Node>,
    index: usize,
}

impl Nfa {
    /// Creates a new NFA from a Vec of nodes.
    /// This Vec can be empty, and then the `new_node` and
    /// `add_node` methods can be used to add new nodes.
    ///
    /// Internally, nodes are tracked by NodePointers,
    /// which simply index into the node vec from a given NFA.
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes, index: 0 }
    }

    /// "Dereferences" a NodePointer in the context of an NFA.
    ///
    /// # Arguments
    ///
    /// * `i` - The node pointer we are dereferencing - note that
    /// i should have been returned from `add_node` or `new_node`, or
    /// sad things can occur.
    ///
    /// # Returns
    /// An Option<&Node>, which will be None if i is not found within
    /// the NFA, or Some(&x) where x is the node that was referenced.
    pub fn get(&self, i: &NodePointer) -> Option<&Node> {
        self.nodes.get(i.id)
    }

    /// Creates a new Node in the NFA, with no transitions to/from it.
    ///
    /// # Returns
    /// A NodePointer, which identifies the newly added Node.
    pub fn new_node(&mut self) -> NodePointer {
        self.add_node(Node::new())
    }

    /// Adds a given Node to the NFA.
    ///
    /// # Arguments
    ///
    /// * `node` - The new Node we are adding.
    ///
    /// # Returns
    /// An NodePointer, which identifies the newly added Node.
    pub fn add_node(&mut self, node: Node) -> NodePointer {
        self.nodes.push(node);
        NodePointer::new(self.nodes.len() - 1)
    }

    pub fn add_transition_range(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
        s: String,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::Range(s), *to))
    }

    pub fn add_transition_queryset(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
        s: String,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::QuerySetRange(s), *to))
    }

    pub fn add_transition_negativerange(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
        s: String,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::NegativeRange(s), *to))
    }

    pub fn add_transition_alpha(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
        on: Atom,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::Alpha(on), *to))
    }

    pub fn add_transition_any(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::Any, *to))
    }

    pub fn add_transition_epsilon(
        &mut self,
        from: &NodePointer,
        to: &NodePointer,
    ) -> Result<(), Box<dyn Error>> {
        self.add_transition(from, Transition::new(TransitionType::Epsilon, *to))
    }

    pub fn add_group(
        &mut self,
        start_from: &NodePointer,
        start_to: &NodePointer,
        end_from: &NodePointer,
        end_to: &NodePointer,
    ) -> Result<(), Box<dyn Error>> {
        self.index += 1;
        self.add_transition(
            start_from,
            Transition::new(TransitionType::Open(self.index), *start_to),
        )?;
        self.add_transition(
            end_from,
            Transition::new(TransitionType::Close(self.index), *end_to),
        )
    }

    fn add_transition(&mut self, from: &NodePointer, to: Transition) -> Result<(), Box<dyn Error>> {
        let node = self.nodes.get_mut(from.id).ok_or("Invalid source!")?;
        node.transitions.push(to);
        Ok(())
    }
    pub fn e_closure(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            for transition in self.nodes[node].transitions.iter() {
                match transition.kind {
                    TransitionType::Epsilon => {
                        stack.push(transition.dest.id);
                    }
                    _ => {}
                }
            }
        }
        let mut v: Vec<usize> = visited.into_iter().collect();
        v.sort();
        v
    }

    fn set_node_type(&mut self, node: NodePointer, nt: NodeType) {
        self.nodes[node.id].set_type(nt);
    }
}
pub fn get_new_state(
    old_nfa: &Nfa,
    dfa: &mut Nfa,
    e_closure_map: &mut HashMap<Vec<usize>, NodePointer>,
    start: usize,
    old_end: usize,
) -> (NodePointer, Vec<usize>) {
    let e = old_nfa.e_closure(start);
    if let Some(i) = e_closure_map.get(&e) {
        return (*i, e);
    } else {
        let new_node = dfa.new_node();
        if e.contains(&old_end) {
            dfa.set_node_type(new_node, NodeType::Accept);
        }
        e_closure_map.insert(e.clone(), new_node);
        return (new_node, e);
    }
}
pub fn nfa_to_dfa(
    nfa: &Nfa,
    start: &NodePointer,
    end: &NodePointer,
) -> (Nfa, NodePointer, NodePointer) {
    let mut e_closure_map: HashMap<Vec<usize>, NodePointer> = HashMap::new();
    let mut visited = HashSet::new();
    let mut dfa = Nfa::new(Vec::new());
    let mut frontier = nfa.e_closure(start.id);
    let (startnode, _) = get_new_state(nfa, &mut dfa, &mut e_closure_map, start.id, end.id);
    while let Some(i) = frontier.pop() {
        if visited.contains(&i) {
            continue;
        }
        visited.insert(i);
        let (new_node, e) = get_new_state(nfa, &mut dfa, &mut e_closure_map, i, end.id);
        for j in e {
            for transition in nfa.nodes[j].transitions.iter() {
                match &transition.kind {
                    TransitionType::Epsilon => {}
                    x => {
                        let (to, _) = get_new_state(
                            nfa,
                            &mut dfa,
                            &mut e_closure_map,
                            transition.dest.id,
                            end.id,
                        );
                        let trans = Transition::new(x.clone(), to);
                        if !dfa.nodes[new_node.id].transitions.contains(&trans) {
                            dfa.add_transition(&new_node, trans).unwrap();
                        }
                    }
                }
                frontier.push(transition.dest.id);
            }
        }
    }
    let (endnode, _) = get_new_state(nfa, &mut dfa, &mut e_closure_map, end.id, end.id);

    (dfa, startnode, endnode)
}

#[derive(Debug, Clone)]
pub enum Path {
    Open(usize),
    Close(usize),
    Char,
    Query(usize)
}

pub fn find_path(
    qe: &mut crate::nfa::queryengine::QueryEngine,
    input: &String,
    index: usize,
    nfa: &Nfa,
    node: NodePointer,
    path: &mut Vec<Path>,
) -> bool {
    if let Some(cur_char) = input.chars().nth(index) {
        let mut longest_path: Option<Vec<Path>> = None;
        for transition in nfa.nodes[node.id].transitions.iter() {
            let mut new_path = Vec::new();
            let b = match &transition.kind {
                TransitionType::Alpha(c) => {
                    if *c == cur_char {
                        new_path.push(Path::Char);
                        find_path(qe, input, index + 1, nfa, transition.dest, &mut new_path)
                    } else {
                        nfa.nodes[node.id].nt == NodeType::Accept
                    }
                }
                TransitionType::Any => {
                    new_path.push(Path::Char);
                    find_path(qe, input, index + 1, nfa, transition.dest, &mut new_path)
                }
                TransitionType::NegativeRange(s) => {
                    if !s.contains(cur_char) {
                        new_path.push(Path::Char);
                        find_path(qe, input, index + 1, nfa, transition.dest, &mut new_path)
                    } else {
                        nfa.nodes[node.id].nt == NodeType::Accept
                    }
                }
                TransitionType::Range(s) => {
                    if s.contains(cur_char) {
                        new_path.push(Path::Char);
                        find_path(qe, input, index + 1, nfa, transition.dest, &mut new_path)
                    } else {
                        nfa.nodes[node.id].nt == NodeType::Accept
                    }
                }
                TransitionType::Open(i) => {
                    new_path.push(Path::Open(*i));
                    find_path(qe, input, index, nfa, transition.dest, &mut new_path)
                }
                TransitionType::Close(i) => {
                    new_path.push(Path::Close(*i));
                    find_path(qe, input, index, nfa, transition.dest, &mut new_path)
                }
                TransitionType::QuerySetRange(s) => {
                    if let Some(x) = qe.query(index, s) {
                        new_path.push(Path::Query(x));
                        find_path(qe, input, index + x, nfa, transition.dest, &mut new_path)
                    } else {
                        nfa.nodes[node.id].nt == NodeType::Accept
                    }
                }
                _ => nfa.nodes[node.id].nt == NodeType::Accept,
            };
            if b {
                if let Some(lp) = &longest_path {
                    if new_path.len() > lp.len() {
                        longest_path = Some(new_path);
                    }
                    //path.append(&mut new_path);
                    //return true;

                } else {
                    longest_path = Some(new_path);
                }
            }
        }
        if let Some(lp) = &mut longest_path {
            path.append(lp);
            true
        } else {
            nfa.nodes[node.id].nt == NodeType::Accept
        }
    } else {
        for transition in nfa.nodes[node.id].transitions.iter() {
            match &transition.kind {
                TransitionType::Close(i) => {
                    if nfa.nodes[transition.dest.id].nt == NodeType::Accept {
                        path.push(Path::Close(*i));
                        return true;
                    }
                }
                _ => {}
            }
        }
        nfa.nodes[node.id].nt == NodeType::Accept
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, Clone)]
pub struct Context {
    nodes: HashSet<NodePointer>,
    groups: Vec<Group>,
    index: usize,
}

impl Context {
    pub fn new(nodes: HashSet<NodePointer>) -> Self {
        Self {
            nodes,
            groups: Vec::new(),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.groups.clear();
        self.index = 0;
        self.nodes.clear();
    }

    pub fn contains(&self, i: &NodePointer) -> bool {
        return self.nodes.contains(i);
    }

    fn open(&mut self, i: usize) {
        while i >= self.groups.len() {
            self.groups.push(Group { start: 0, len: 0 });
        }
        self.groups.get_mut(i).unwrap().start = self.index;
    }

    fn close(&mut self, i: usize) {
        while i >= self.groups.len() {
            self.groups.push(Group { start: 0, len: 0 });
        }
        let t = self.groups.get_mut(i).unwrap();
        t.len = self.index - t.start;
    }

    pub fn step(&mut self, nfa: &Nfa, input: Atom, q: &queryengine::QueryEngine) -> usize {
        let mut nodes = HashSet::new();
        for nodeptr in &self.nodes {
            if let Some(node) = nfa.get(nodeptr) {
                for t in &node.transitions {
                    match &t.kind {
                        TransitionType::Alpha(c) if *c == input => {
                            nodes.insert(t.dest);
                        }
                        TransitionType::Any => {
                            nodes.insert(t.dest);
                        }
                        TransitionType::Range(s) if s.contains(input) => {
                            nodes.insert(t.dest);
                        }
                        TransitionType::NegativeRange(s) if !s.contains(input) => {
                            nodes.insert(t.dest);
                        }
                        TransitionType::QuerySetRange(s) => {
                            if let Some(x) = q.query(self.index, s) {
                                self.index = x - 1;
                                nodes.insert(t.dest);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        self.index += 1;
        self.add_epsilons(nodes, nfa);
        self.index
    }

    pub fn add_epsilons(&mut self, nodes: HashSet<NodePointer>, nfa: &Nfa) {
        let mut nodes = nodes;
        loop {
            let prev = nodes.clone();
            let size = nodes.len();
            for nodeptr in &prev {
                if let Some(node) = nfa.get(nodeptr) {
                    for t in &node.transitions {
                        if let TransitionType::Epsilon = t.kind {
                            nodes.insert(t.dest);
                        } else if let TransitionType::Open(s) = t.kind {
                            nodes.insert(t.dest);
                            self.open(s);
                        } else if let TransitionType::Close(s) = t.kind {
                            nodes.insert(t.dest);
                            self.close(s);
                        }
                    }
                }
            }
            if size == nodes.len() {
                self.nodes = nodes;
                return;
            }
        }
    }
}

#[derive(Debug)]
pub struct NfaModel {
    nfa: Nfa,
    start: NodePointer,
    end: NodePointer,
}

#[test]
fn test_nfa_insert() -> Result<(), Box<dyn Error>> {
    let mut nfa = Nfa::new(Vec::new());
    nfa.add_node(Node::new());
    nfa.add_node(Node::new());
    nfa.add_node(Node::new());
    assert_eq!(nfa.nodes.len(), 3);
    Ok(())
}

#[test]
fn test_nfa_alpha_transition() -> Result<(), Box<dyn Error>> {
    let mut nfa = Nfa::new(Vec::new());
    let a = nfa.add_node(Node::new());
    let b = nfa.add_node(Node::new());
    nfa.add_transition_alpha(&a, &b, 'a')?;
    let mut ctx = Context::new(vec![a].into_iter().collect());
    ctx.step(&nfa, 'b', &QueryEngine::new());
    assert_eq!(ctx.nodes.len(), 0);
    let mut ctx = Context::new(vec![a].into_iter().collect());
    ctx.step(&nfa, 'a', &QueryEngine::new());
    assert_eq!(ctx.nodes.len(), 1);
    assert!(ctx.nodes.contains(&b));
    Ok(())
}

#[test]
fn test_nfa_epsilon_transition() -> Result<(), Box<dyn Error>> {
    let mut nfa = Nfa::new(Vec::new());
    let a = nfa.new_node();
    let b = nfa.new_node();
    let c = nfa.new_node();
    nfa.add_transition_alpha(&a, &b, 'a')?;
    nfa.add_transition_epsilon(&b, &c)?;
    let mut ctx = Context::new(vec![a].into_iter().collect());
    ctx.step(&nfa, 'b', &QueryEngine::new());
    assert_eq!(ctx.nodes.len(), 0);
    let mut ctx = Context::new(vec![a].into_iter().collect());
    ctx.step(&nfa, 'a', &QueryEngine::new());
    assert_eq!(ctx.nodes.len(), 2);
    assert!(ctx.nodes.contains(&b));
    assert!(ctx.nodes.contains(&c));
    Ok(())
}
