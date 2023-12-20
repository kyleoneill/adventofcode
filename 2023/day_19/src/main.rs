use std::collections::HashMap;
use problem::{solve_main, Problem};

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl Part {
    fn sum_part_fields(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
    fn sum_accepted_parts(parts: &Vec<Self>) -> usize {
        parts.into_iter().map(Part::sum_part_fields).sum()
    }
}

#[derive(Debug)]
enum PartLabel {
    X,
    M,
    A,
    S
}

impl PartLabel {
    fn c_from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Got invalid part label, expected one of XMAS")
        }
    }
}

#[derive(Debug)]
struct Rule {
    part_label: PartLabel,
    greater_than: bool,
    value: usize,
    dest_true: String
}

impl Rule {
    fn c_from_str(s: &str) -> Self {
        let (mut split, greater_than) = if s.contains(">") { (s.split(">"), true) } else { (s.split("<"), false) };
        let part_label = PartLabel::c_from_str(split.next().unwrap());
        let mut more_splits = split.next().unwrap().split(":");
        let value: usize = more_splits.next().unwrap().parse().unwrap();
        let dest_true = more_splits.next().unwrap().to_owned();
        Self { part_label, greater_than, value, dest_true }
    }
    fn try_part(&self, part: &Part) -> Option<String> {
        let val_to_test = match self.part_label {
            PartLabel::X => part.x,
            PartLabel::M => part.m,
            PartLabel::A => part.a,
            PartLabel::S => part.s
        };
        if (self.greater_than && val_to_test > self.value) || (!self.greater_than && val_to_test < self.value) {
            return Some(self.dest_true.clone())
        }
        None
    }
}

#[derive(Debug)]
enum FallThrough {
    Accept,
    Reject,
    Function(String)
}

impl FallThrough {
    fn c_from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Function(s.to_owned())
        }
    }
}

#[derive(Debug)]
struct Function {
    rules: Vec<Rule>,
    fallthrough: FallThrough
}

impl Function {
    fn from_input(input: &Vec<String>, version: usize) -> (HashMap<String, Self>, Vec<Part>) {
        let mut map: HashMap<String, Self> = HashMap::new();
        let mut parts :Vec<Part> = Vec::new();
        let mut building_functions = true;
        for line in input {
            if line.trim().is_empty() {
                building_functions = false;
                continue;
            }
            if building_functions {
                let mut rules: Vec<Rule> = Vec::new();
                let mut first_split = line.split("{");
                let name = first_split.next().unwrap();
                let foo = first_split.next().unwrap();
                let non_split = &foo[..foo.len() - 1];
                let mut str_rules = non_split.split(",").peekable();
                loop {
                    let str_rule = str_rules.next().unwrap();
                    if str_rules.peek().is_none() {
                        // this is the last iteration and our fallthrough case
                        let fallthrough = FallThrough::c_from_str(str_rule);
                        map.insert(name.to_owned(), Self { rules, fallthrough });
                        break;
                    }
                    else {
                        // this is not the last iteration and we are going to add a rule
                        rules.push(Rule::c_from_str(str_rule));
                    }
                }
            }
            if !building_functions && version == 1 {
                // done building functions, time to get the parts being passed in
                let line_len = line.len();
                let remove_braces = &line[1..line_len - 1];
                let mut split = remove_braces.split(",");
                let x: usize = split.next().unwrap()[2..].parse().unwrap();
                let m: usize = split.next().unwrap()[2..].parse().unwrap();
                let a: usize = split.next().unwrap()[2..].parse().unwrap();
                let s: usize = split.next().unwrap()[2..].parse().unwrap();
                parts.push(Part { x, m, a, s })
            }
        }
        (map, parts)
    }
    fn map_parts(map: HashMap<String, Function>, parts: Vec<Part>) -> Vec<Part> {
        let mut ret: Vec<Part> = Vec::new();
        for part in parts {
            let mut workflow_name = "in".to_owned();
            loop {
                match workflow_name.as_str() {
                    "A" => { ret.push(part); break; },
                    "R" => break,
                    _ => {
                        let function = map.get(workflow_name.as_str()).unwrap();
                        let mut made_match = false;
                        for rule in &function.rules {
                            match rule.try_part(&part) {
                                Some(new_workflow_name) => { workflow_name = new_workflow_name; made_match = true; break },
                                None => () 
                            }
                        }
                        if !made_match {
                            match &function.fallthrough {
                                FallThrough::Accept => { ret.push(part); break; },
                                FallThrough::Reject => break,
                                FallThrough::Function(new_workflow_name) => workflow_name = new_workflow_name.to_owned()
                            }
                        }
                    }
                }
            }
        }
        ret
    }
}

struct Day19;

impl Problem for Day19 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (function_map, parts) = Function::from_input(input, 1);
        // simplify rule chain? Walk each rule chain and see if we can replace rules with other rules to simplify them
        // ex in example, crn can be folded into qkq to be `A if (x < 1416 || x > 2662) else R`. This can then be folded back into
        // px to make it larger but simpler
        let accepted_parts = Function::map_parts(function_map, parts);
        Part::sum_accepted_parts(&accepted_parts)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        // Instead of walking each part through the chain of functions, can we see what ranges of values pass through?
        // If part 1 is following a map, part 2 is flooding a maze with holes in random ends and seeing which ends have outputs
        0
    }
}

fn main() {
    solve_main::<Day19>();
}
