pub fn is_valid(s: String) -> bool {
    // Упражнение 1. Распакуйте строку в вектор char'ов и тогда можно обращаться к отдельным буквам по индексу (по жизни так поступать не надо!)
    //  Третий вариант -- работайте с байтами строки (s.bytes() или s.as_bytes(), а литерал будет выглядеть как b'(')
    //  Закодируйте работу со строкой как с байтами. Придумайте тест, показывающий, что этот вариант работает неправильно.
    // let s2: Vec<char> = s.as_str().chars().collect();
    // for i in 0..s2.len() {
    //     if s[i] == '(' {
    //         return true;
    //     }
    // }

    let mut opened: Vec<char> = Vec::new();        
    // Альтернативные способы создать пустой вектор:
    // let opened: Vec<char> = vec![];        
    // let opened = Vec::<char>::new();        

    for c in s.as_str().chars() {
        // Упражнение 2. Замените эти if'ы на match. От цикла по сортам скобок придётся отказаться
        if c == '(' || c == '[' || c == '{' {
            opened.push(c);
        } else {
            // Тут у нас возникла проблема, что итерация по массиву, дающая (char, char) работает только в самых новых версиях раста.
            //  В старых версиях доступна только итерация, дающая &(char, char), этот вариант вы видите ниже
            for &(open, close) in [('(', ')'), ('[', ']'), ('{', '}')].into_iter() {
                if c == close {
                    let last = opened.pop();
                    if last != Some(open) {
                        return false;
                    }
                }
            }
        }
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
