mod day1;
mod day2;
mod day3;

use day1::*;
use day2::*;
use day3::*;

pub fn exec(day: i32) {
    match day {
        1 => {
            println!();
            let mut arr = [0; 10];
            for i in 0..10 {
                arr[i] = fibonacci(i as i32);
            }
            println!("斐波那契数列Fn(10): {:?}", arr);
           
            println!("{:?}", Fibonacci::array(10));
            
            println!();
            println!("*************************九九乘法表**********************");
            multiplication_table();
            println!();
        }
        2 => {
            println!();
            println!("下期双色球号码是：{:?}", generate_lottery_nums());
            println!("必中~！");
        }
        3 => {
            println!();
            let mut arr = [3,1,2,5,4];
            println!("数组排序：{:?}",bubble_sort(&mut arr));
        }
        _ => {
            println!();
            println!("***************************************************");
            println!("***************************************************");
            println!("****                                           ****");
            println!("****    ^.^ you have not reach to this day.    ****");
            println!("****                                           ****");
            println!("***************************************************");
            println!("***************************************************");
            println!();
        }
    }
}

