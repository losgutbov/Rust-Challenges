// fn loop_label() {
//     let mut count = 0;

//     'counting_up: loop{
//         println!("count = {count}");

//         let mut remaining = 10;

//         loop{
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }

//     println!("End count = {count}");
// }

// fn while_loop(){
//     let mut number = 3;

//     while number !=0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn iteration_while(){
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is {}", a[index]);

//         index +=1;
//     }
// }

// fn for_array_loop(){
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value id: {element}");
//     }
// }

fn main(){
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}