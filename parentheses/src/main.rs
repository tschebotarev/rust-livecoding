pub fn is_valid(s: String) -> bool {
    // let opened: Vec<char> = vec![];        
    // let opened = Vec::<char>::new();        

    // let s2: Vec<char> = s.as_str().chars().collect();
    // for i in 0..s2.len() {
    //     if s[i] == '(' {
    //         return true;
    //     }
    // }

    let mut opened: Vec<char> = Vec::new();        
    for c in s.as_str().chars() {
        if c == '(' || c == '[' || c == '{' {
            opened.push(c);
        } else {
            for &(open, close) in [('(', ')'), ('[', ']'), ('{', '}')].into_iter() {
                if c == close {
                    let last = opened.pop();
                    if last != Some(open) {
                        return false;
                    }
                }
            }
        }

        // ! match

        // if c == ')' {
        //     let last = opened.pop();
        //     if last != Some('(') {
        //         return false;
        //     }
        // }
        // if c == ']' {
        //     let last = opened.pop();
        //     if last != Some('[') {
        //         return false;
        //     }
        // }
        // if c == '}' {
        //     let last = opened.pop();
        //     if last != Some('{') {
        //         return false;
        //     }
        // }
    }
    opened.is_empty()
}

fn main() {
    // let s: String = "(sжопа)".into();
    // println!("L={}", s.len())
}

#[test]
fn test_basic() {
    assert!(is_valid("[]".into()), "Test for [] failed")
}

#[test]
fn test0() {
    assert!(!is_valid("[s)e]".into()))
}

#[test]
fn test1() {
    assert!(is_valid("[s()e]".into()))
}

#[test]
fn test_unordered() {
    assert!(!is_valid("([s)e]".into()))
}
