/*
Author: Ezekiel Enns
Description: command line DFA uses configuration files as arguments
Assumptions: program will quite if to little arguments are given
             program will panic if description failed to parse
             program will ignore excess transitions
             errors are the only prints
*/

use std::{i32, str, string, char};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    // error check for to little arguments
    if args.len() < 3 {
        println!("Error to little arguments");
        return;
    }

    // populating definition
    let (
        alphabet,                 //a a string representing the alphabet indexes are used
        startState,
        accepting,            // accepting[state] = isAccepting
        transitions       // transitions[letter index][state] = state to transition to
        ) = getDeff(&args[1]);

    let input = fs::read_to_string(&args[2]).unwrap();

    let mut out= String::new();

    // going through each line of input file
    for line in input.lines() {
        let mut currState = startState;
        let mut vaild = true;               // used to ignore extra concatenation of out string when invalid

        // going through input lines characters
        for char in line.chars() {
            // there is probably a better way to do this check but it works
            if let  Some(inAlpha) = alphabet.find(char) {
             // transition state if in alphabet
             currState = transitions[inAlpha][currState];
            }
            else {
             //its out of our alphabet so just leave
             out += "INVALID" ;
             vaild = false;
             break;
         }
        }
        // adding final result of DFA to out string
        if vaild {
            out += if accepting[currState] { "ACCEPT" } else { "REJECT" };
        }
        out += "\n";
    }

    // storing output to out file
    let path = Path::new(&args[3]);
    let dis = path.display();
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {println!("Error Making Out : {}",err); return;},
    };

    match file.write_all(out.as_bytes()) {
        Ok(_) => {}
        Err(err) =>  println!("Error Writing Out : {}",err),
    }

}

/*
    fills a tuple with required data about dfa
    by parsing a definition string 'path'
*/

fn getDeff(path: &String) -> (String, usize, Vec<bool>, Vec<Vec<usize>>) {
    let fileStream = fs::read_to_string(path).expect("error with Deff file stream");
    let mut deffFile = fileStream.lines();                                                  // iterator is used to easily get the first unique lines

    let alphabet = deffFile.next().unwrap().to_string();
    let totStates=
        if let Ok(parse) = deffFile.next().unwrap().parse::<usize>() {
            parse
        }
        else { panic!("failed to parse total states") };

    /*
        state transitions are stored in a multi denominational array
        transitions[letter][state] = state to transition to;
        using -1 as a place holder
    */
    let mut transitions = vec![vec![0; totStates]; alphabet.len()];

    let startState =
        if let Ok(parse) = deffFile.next().unwrap().parse::<usize>() {
            parse
        }
        else { panic!("failed to parse start state") };

    // vec who's index represents a state and when true is accepting
    let mut accepting = vec![false; totStates];
    for acceptState in deffFile.next().unwrap().split_whitespace() {
        if let Ok(parse) = acceptState.parse::<usize>() {
            accepting[parse] = true;
        }
        else { panic!("Failed to parse accepting state") }
    }

    for i in 0..totStates {
        // a least amount of code to parse through the data
        let line = deffFile.next().unwrap().split_whitespace()
            .map(|s| s.parse().expect("parse Error in description")).collect::<Vec<usize>>();
        for (lTrans,iState) in line.iter().enumerate() {
            transitions[lTrans][i] = *iState;
        }

    }
    return (alphabet,startState,accepting,transitions)
}