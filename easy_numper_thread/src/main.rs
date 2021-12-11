use std::thread;
use std::io::{stdin, BufRead};
use std::sync::mpsc::channel;
//use std::time; // thread::sleep(time::Duration::from_millis(1000));
use std::time;

/*
Программа для поиска простых чисел
программа каждые 2 секунды выводит 10 последних найденных чисел, подходящих условию, введенного пользователем (в начале программы "", т.е. все простые числа)
(т.е. если ввести "23", то прога будет добавлять в "стек" чисел только числа, содержащие эту комбинацию, старые числа из стека не удаляются, а заменяются новыми,
поэтому сразу после ввода нового условия могут остаться старые, неподходящие числа, и если ввести в условие "wer"(невозможное условие), то в стек не будет ничего 
попадать, т.к под это условие ничего не подходит)
пользователь может ввести условие в любой момент
при вводе "e" программа завершится
*/

use std::time::{SystemTime, UNIX_EPOCH}; // for time testing
pub fn millis() -> f64 { 
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64)/(1000.0)
}

fn test_easy_number(n:u32) -> bool {
    for i in 2..n/2 {
        if n%i==0 {
            return false;
        }
    }
    true
}

fn next_easy_number(mut n:u32) -> u32 {
    loop {
        n+=1;
        if test_easy_number(n) {
            return n;
        }
    }
}

/*fn update_stek(array: &mut Vec<u32>) {
    for i in 0..9 {
        array[i] = array[i+1];
    } 
    array[9] = next_easy_number(array[8]);
}*/

fn add_to_stek(array: &mut Vec<u32>, n:u32) {
    for i in 0..9 {
        array[i] = array[i+1];
    } 
    array[9] = n;
}

fn translate_vec_to_string(array: Vec<u32>) -> String {
    let mut answer = "".to_string();
    for i in 0..10 {
        answer += &array[i].to_string();
        answer += &" ".to_string();
    }
    answer
}

/*fn string_is_number(s:String) -> bool {
    for i in s.chars() {
        print!("{} ",i);
    }
    println!();
    true
}*/

fn main() {
    //let mut array:Vec<u32> = vec![1,2,3,5,7,11,13,17,19,23];
    //for _ in 0..3 { update_stek(&mut array); }
    //println!("{:?}", array);

    let (tx, rx) = channel();
    let (tx1, rx1):(std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = channel();

    // генерируем числа
    let sender = thread::spawn(move || {
        let mut array:Vec<u32> = vec![1,2,3,5,7,11,13,17,19,23];
        let mut last_number:u32 = 23;
        let mut time = millis();
        let mut people_request = "".to_string();
        loop {
            // продумываем новое простое число и все это пишем в "стек"
            last_number = next_easy_number(last_number);
            if last_number.to_string().contains(&people_request) {
                //println!("={}",last_number);
                add_to_stek(&mut array, last_number);
            }
            //update_stek(&mut array);
            
            // проверяем, если в течении 10 милисекунд обнарживаем в канале "запрос", то принимаем, а если нет, то "nothing"
            let t = rx1.recv_timeout(time::Duration::from_millis(10));
            //let test = rx1.recv().expect("Unable to receive from channel");
            let test = match t {
                Ok(t) => t,//t.unwrap(),
                Err(_) => "nothing".to_string(),
            };
            
            // действия с данными из канала (от канала people)
            if test!="nothing".to_string() {
                if test=="e".to_string() { break; }
                else {
                    //if string_is_number(test.clone())
                    people_request = test;
                    tx.send(translate_vec_to_string(array.clone()).to_owned()).expect("Unable to send on channel");
                }
            }

            // раз в 2 секунды выводить вычисленные значения
            if time+2.0<=millis() {
                time = millis();
                println!("* {:?}", array);
            }
        }
        println!("end brain thread");
    });



    // принимаем числа и общаемся с пользователем
    let receiver = thread::spawn(move || {
        loop {
            // опрос консоли 
            let mut input = "".to_string();
            stdin().lock().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            // запрос\ответ с потоком brain
            tx1.send(input.to_owned()).expect("Unable to send on channel");
            if input=="e".to_string() { break; } // выйти если была такая команда
            else {
                let value = rx.recv().expect("Unable to receive from channel");
                println!("{}", value);
            }
            /*loop {
                let value = rx.recv().expect("X");
                if value=="X".to_string() { break; }
                else { last_value = value; }
            }*/
            //if last_value.contains(&input) { 
        }
        println!("end people thread");
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");

}