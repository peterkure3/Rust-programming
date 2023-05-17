fn main(){
    let company_string = "TutorialsPoint"; //string
    let rating_float=4.5; //float
    let is_growing_boolean=true; //boolean
    let icon_char='♥'; //unicode
    let result=10; // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize=10;
    let count:usize = 30;

    println!("company name is:{}", company_string);
    println!("company rating on 5 is:{}", rating_float);
    println!("company is growing  :{}",is_growing_boolean );
    println!("company icon is:{}", icon_char);
    println!("result is: {}", result);   
    println!("sum is {} and
     age is {}", sum, age);
    println!("mark is {} and count {}",mark,count);
}