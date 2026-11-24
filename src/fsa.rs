use std::{collections::{HashMap}, error::Error};

struct Transition<'l> {
    state: &'l str,
    next: &'l str,
    result: &'l str,
}

pub struct FSA<'l> {
    mach: HashMap<(&'l str, &'l str), &'l str>,
    states: Vec<&'l str>,
    alpha:Vec<&'l str>,
    accepts:Vec<&'l str>,
    initial:&'l str,
}

impl<'l> FSA<'l> {
    pub fn new(lang:&'l str) -> Result<Self, Box<dyn Error>>{
        let states = Self::parse_single_lines(lang, "states:".into())?;
        let alpha = Self::parse_single_lines(lang, "alphabet:".into())?;
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
    
    pub fn get_machine(&self) -> &HashMap<(&'l str, &'l str), &'l str> {
       &self.mach 
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

    fn parse_transitions(lang: &'l str, states: &[&'l str], alpha: &[&'l str])-> Result<Vec<Transition<'l>>, Box<dyn Error>>{
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
            let next = parts.next().ok_or_else(|| format!("Missing input symbol in transition: {line}"))?;
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


}
