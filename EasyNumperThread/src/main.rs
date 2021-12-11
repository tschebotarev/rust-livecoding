use std::thread;
use std::io::{stdin, BufRead};
use std::sync::mpsc::channel;
//use std::time; // thread::sleep(time::Duration::from_millis(1000));
use std::time;

use std::time::{SystemTime, UNIX_EPOCH}; // for time testing
pub fn millis() -> f64 { // https://qastack.ru/programming/26593387/how-can-i-get-the-current-time-in-milliseconds
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

fn update_stek(array: &mut Vec<u32>) {
    for i in 0..9 {
        array[i] = array[i+1];
    } 
    array[9] = next_easy_number(array[8]);
}

fn translate_vec_to_string(array: Vec<u32>) -> String {
    let mut answer = "".to_string();
    for i in 0..10 {
        answer += &array[i].to_string();
        answer += &" ".to_string();
    }
    answer
}

fn main() {
    //let mut array:Vec<u32> = vec![1,2,3,5,7,11,13,17,19,23];
    //for _ in 0..3 { update_stek(&mut array); }
    //println!("{:?}", array);

    let (tx, rx) = channel();
    let (tx1, rx1):(std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = channel();

    // генерируем числа
    let sender = thread::spawn(move || {
        let mut array:Vec<u32> = vec![1,2,3,5,7,11,13,17,19,23];
        let mut time = millis();
        loop {
            // продумываем новое простое число и все это пишем в "стек"
            update_stek(&mut array);
            
            // проверяем, если в течении 10 милисекунд обнарживаем в канале "запрос", то принимаем
            let t = rx1.recv_timeout(time::Duration::from_millis(10));
            //let test = rx1.recv().expect("Unable to receive from channel");
            let test = match t {
                Ok(t) => t,//t.unwrap(),
                Err(_) => "nothing".to_string(),
            };
            
            // действия с данными из канала (от канала people)
            if test=="".to_string() {
                tx.send(translate_vec_to_string(array.clone()).to_owned()).expect("Unable to send on channel");
            }
            else if test=="e".to_string() { 
                break;
            }

            // раз в секунду выводить вычисленные значения
            if time+1.0<=millis() {
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