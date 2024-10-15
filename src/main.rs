// struct User {
//     f_name:String,
//     l_name:String,
//     age:i32,
// }

struct Rect{
    width:i128,
    height:i128,

}
impl Rect{

    fn area(&self) -> i128{
        self.width * self.height
    }
}


fn main() {
    // println!("Is number even => {}" , is_even(20200));
    // println!("Fib num => {}" , fib1(6));

    //String
    // let my_string = String::from("Hello Word!");
    // let len = get_string_len_chars(&my_string);
    // println!("The num of char in str -> {}", len);

    //Struct
    // let user = User{
    //     f_name:String::from("Harsh"),
    //     l_name:String::from("Radadiya"),
    //     age:21,
    // };

    // println!("{}" , user.f_name);

    let  rect = Rect{
        width:5,
        height:5,
    };
    println!("Rect w => {} , h => {} , A => {}" , rect.width , rect.height , rect.area());

}

// fn get_string_len_chars(s: &str) -> usize{
//     s.chars().count()
// }

// fn is_even(num:i128) ->bool {
//     if num%2==0{
//         return true;
//     }
//     return false;
// }
// fn fib(num:i128)->i128{
//     if num == 1 || num == 0{
//         return num;
//     }
//     return fib(num-1) + fib(num-2);
// }
// fn fib1(num:i128)->i128{
//     let mut f = 0;
//     let mut s = 1;
//     if num==0 {return f};
//     if num==1 {return s};

//     for i in 1..num{
//         let temp = s;
//         s = s + f;
//         f = temp;
//     }
//     return s;
// }