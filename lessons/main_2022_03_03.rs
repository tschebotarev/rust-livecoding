// macros

/// block
/// expr используют для обозначения выражений
/// ident используют для обозначения имени переменной/функции
/// item
/// literal используется для литеральных констант
/// pat (образец)
/// path
/// stmt (единственный оператор)
/// tt (единственное дерево лексем)
/// ty (тип)
/// vis (спецификатор видимости)


macro_rules! test {
    
    ($left:expr; xexe $right:expr) => (  // expr - какая-то данная, 
        println!("{:?} xexe {:?} это {:?}", // xexe - И
                 stringify!($left), // преобразовать то что пришло в строку
                 stringify!($right),
                 $left && $right)
    );
    ($left:expr; omg $right:expr; $righ1t:expr) => ( 
        println!("{:?} omg {:?} это {:?}", // omg - ИЛИ
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
    ($func_name:ident) => ( // ident - название функции
        fn $func_name() {
            println!("Вызвана функция {:?}()",
                     stringify!($func_name))
        }
    );
    ($data:expr) => (
        println!("{:?}",stringify!($data));
    );
}

macro_rules! maximum {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (  // как работает + ?
        //println!("{:?}",stringify!($($y)));
        std::cmp::max($x, maximum!($($y),+))
    )
}

test!(qwerty1);
test!(qwerty);

// ?
// https://doc.rust-lang.ru/stable/rust-by-example/macros/dry.html

macro_rules! calculator {
    ($a:expr, plus $b:expr) => (
        $a + $b
    );
    ($a:expr, minus $b:expr) => (
        $a - $b
    );
    ($a:expr, multiply $b:expr) => (
        $a * $b
    );
    ($a:expr, share $b:expr) => (
        $a / $b
    );
}

fn main() {
    println!("test:\n");
    test!(1i32 + 1 == 2; xexe 2i32 * 2 == 4i32);
    test!(true; omg false; false);
    test!(1i32 + 1 == 2; xexe 2i32 * 2 == 4i32);
    test!(1i32 + 1 == 2; xexe 2i32 * 2 == 4i32);

    qwerty();
    qwerty();
    qwerty1();

    test!(12345);

    println!("{}", maximum!(23));
    println!("{}", maximum!(23, 123, 123, 354, 5, 333));

    println!("{}", calculator!(3, share 2));
}


