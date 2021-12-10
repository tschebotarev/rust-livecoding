use std::{
    io::{stdin, BufRead},
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread::{self, sleep},
    time::Duration,
};

fn number_contains(x: u64, substr: &str) -> bool {
    x.to_string().contains(substr)
}

fn calc_primes(tasks_receiver: Receiver<String>, results_sender: SyncSender<u64>) {
    let mut task = tasks_receiver.recv().unwrap();
    for i in 2u64.. {
        for j in 2..i {
            let d = i / j;
            if d * j == i {
                break;
            }
            if d < j {
                if number_contains(i, &task) {
                    results_sender.send(i).unwrap();
                }
                if let Ok(new_task) = tasks_receiver.try_recv() {
                    if !new_task.is_empty() {
                        task = new_task;
                    }
                }
                break;
            }
        }
    }
}

fn main() {
    let (results_sender, results_receiver): (SyncSender<u64>, Receiver<u64>) = sync_channel(10);
    let (intermediate_in, intermediate_out): (SyncSender<u64>, Receiver<u64>) = sync_channel(10);
    let (tasks_sender, tasks_receiver) = sync_channel(1);

    let _handle = thread::spawn(move || {
        calc_primes(tasks_receiver, results_sender);
    });
    // Дополнительный поток печатает 1 число в секунду, а остальные перекладывает в следующий канал.
    // В результате, правда, порядок не сохранится:
    //  этот поток напечатает числа №1, №11, №13, №14, №15,.. а числа №2..№10 и №12 напечатают после сигнала пользователя
    let _handle = thread::spawn(move || loop {
        let first = results_receiver.recv().unwrap();
        println!(">{}", first);
        while let Ok(res) = results_receiver.try_recv() {
            if intermediate_in.try_send(res).is_err() {
                break;
            }
        }
        sleep(Duration::from_secs(1));
    });

    loop {
        let mut input = "".to_string();
        stdin().lock().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();

        tasks_sender.send(input.clone()).unwrap();
        while let Ok(res) = intermediate_out.try_recv() {
            println!("{}", res);
        }
        println!("------------------");
    }
}
