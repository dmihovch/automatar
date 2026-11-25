use std::{collections::{HashMap}, error::Error};

struct Transition<'l> {
    state: &'l str,
    next: char,
    result: &'l str,
}

pub struct FSA<'l> {
    mach: HashMap<(&'l str, char), &'l str>,
    states: Vec<&'l str>,
    alpha:Vec<char>,
    accepts:Vec<&'l str>,
    initial:&'l str,
}

impl<'l> FSA<'l> {
    pub fn new(lang:&'l str) -> Result<Self, Box<dyn Error>>{
        let states = Self::parse_single_lines(lang, "states:".into())?;
        let alpha = Self::parse_alphabet(lang)?;
        let accepts = Self::parse_single_lines(lang, "accepting:".into())?;
        let initial = Self::parse_initial(lang)?;
        let trans = Self::parse_transitions(lang, &states, &alpha)?;
        Ok(Self { 
            mach: trans.into_iter().map(|t| ((t.state, t.next), t.result)).collect(),
            states: states,
            alpha: alpha,
            accepts: accepts, 
            initial: initial,
        })
    }
    

    pub fn print_machine(&self) {
        println!("========== Printing Machine Info ==========");
        print!("States:");
        self.states.iter().for_each(|s| print!(" {}",s));
        print!("\nAlphabet:");
        self.alpha.iter().for_each(|a| print!(" {} ",a));
        print!("\nAccept States:");
        self.accepts.iter().for_each(|a| print!(" {} ",a));
        print!("\nInitial State: {}",self.initial);
        println!("\nTransitions:");
        for (state, symbol) in self.mach.keys() {
            match self.mach.get(&(*state,*symbol)) {
                Some(nstate) => println!("\t{state}, {symbol} => {nstate}"),
                None => eprintln!("in print_machine, this should never happen"),
            }
        }
        println!("===========================================")
    }
    
    fn parse_alphabet(lang: &'l str) -> Result<Vec<char>, Box<dyn Error>> {
        let alphabet:Vec<&'l str> = lang
            .lines()
            .find_map(|line| line.strip_prefix("alphabet:"))
            .ok_or("Missing alphabet field")?
            .split_whitespace()
            .collect();
        let mut chalphabet = Vec::new();
        for a in alphabet {
            chalphabet.push(a.parse::<char>()?);
        }
        Ok(chalphabet)
    }
    fn parse_single_lines(lang: &'l str, which_elem:&str) -> Result<Vec<&'l str>,Box<dyn Error>> {
        let err_msg = which_elem.char_indices()
            .next_back()
            .map_or(which_elem, |(i,_)| &which_elem[..i]);
       let elements = lang
           .lines()
           .find_map(|line| line.strip_prefix(&which_elem))
           .ok_or_else(|| format!("Missing '{err_msg}' field"))?
           .split_whitespace()
           .collect();
        Ok(elements)
    }

    fn parse_initial(lang: &'l str) -> Result<&'l str,Box<dyn Error>> {
       let init = lang
           .lines()
           .find_map(|line| line.strip_prefix("initial:"))
           .ok_or_else(|| format!("Missing initial field"))?
           .trim();
        Ok(init)
    }

    fn parse_transitions(lang: &'l str, states: &[&'l str], alpha: &[char])-> Result<Vec<Transition<'l>>, Box<dyn Error>>{
        let headers = [
            "states:",
            "alphabet:",
            "initial:",
            "accepting:",
            "transitions:",
        ];
        let mut trans = Vec::new(); 
        let mut lines = lang.lines().map(str::trim);
        lines
            .find(|&line| line == "transitions: {")
            .ok_or("Missing transitions header")?;
        for line in lines {
            if line == "}" {
                return Ok(trans);
            }
            if line.is_empty() {
                continue;
            }
            if headers.iter().any(|header| line.starts_with(header)) {
                return Err("Unclosed transitions section".into());
            }
            let mut parts = line.split_whitespace();
            let state = parts.next().ok_or_else(|| format!("Missing source state in transition: {line}"))?;
            let next = parts.next().ok_or_else(|| format!("Missing input symbol in transition: {line}"))?.parse::<char>()?;
            let result = parts.next().ok_or_else(|| format!("Missing result state in transition: {line}"))?;
            
            if !states.contains(&state) {
                return Err(format!("{state} does not exist in states. In transition: {line}").into());
            }
            if !alpha.contains(&next) {
                return Err(format!("{next} does not exist in the alphabet. In transition: {line}").into());
            }
            if !states.contains(&result) {
                return Err(format!("{result} does not exists in states. In transition: {line}").into());
            }

            trans.push(Transition { state, next, result });
        }
        Err("Missing closing brace for transitions".into())
    } 

    fn report_error_in_string(&self, idx: usize, ch: char ){
        println!("Invalid character found at index {idx}, character '{ch}'");
        println!("The string is not in the language");
    }
    
    pub fn evaluate_string(&self, input: &str)-> bool {
       let input = input.trim();
       println!("Processing input string:\n[{input}]\n-----------------------");
       let mut next_state = self.initial;
       for (i, c) in input.char_indices() {
            match self.mach.get(&(next_state, c)){
                Some(state) => next_state = state,
                None => {
                    self.report_error_in_string(i, c); 
                    return false;
                },
            }
       }
       if self.accepts.contains(&next_state){
           println!("The string is in the language!");
           return true
       }
       println!("The string contains all valid characters, but is not in the language");
       false
    }

}
