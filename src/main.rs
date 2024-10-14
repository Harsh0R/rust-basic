fn main() {
    println!("Is number even => {}" , is_even(20200));
}
fn is_even(num:i128) ->bool {
    if num%2==0{
        return true;
    }
    return false;
}
