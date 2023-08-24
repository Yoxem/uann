/// (c) 2023 Tan Kian-ting <yoxem@kianting.info> 
/// Under MIT License
/// 习包子 梁家河小学博士 清零宗 习炀帝 庆丰大帝
/// 独裁国贼 新疆集中营 光复香港时代革命 祈翠 南蒙古独立 香港独立
/// 

///
/// pairs of string for matching and parsing
/// 
/// - `matched` : the string being accumulatedly matched.
/// - `remained` : the string to be matched
#[derive(Debug, Clone)]
pub struct Matchee {
    matched : String,
    remained : String,
}

/// macro convert a str as a string, equivalent to `x.to_string()`.
/// 
/// # Example:
/// ```
/// let a = "abc";
/// assert_eq!(string!(a), a.to_string());
/// ```
///
macro_rules! string {
    ($name:expr) => {$name.to_string()}
}


/// convert a string to 
/// a vector of char
/// 
///  * s : input `str`'s reference
/// 
/// # Example
/// 
/// ```
/// let s = "Lí 好！";
/// let char_vec: Vec<char> = str_to_char_vec(s);
/// assert_eq!(char_vec, vec!['L','í',' ','好','！'])
/// ```
fn str_to_char_vec (s : &str) -> Vec<char>{
    return s.chars().collect();
}

/// return a closure such that
/// if the 1st char of `Matchee.matched` matches `ch`,
/// then return the new `Some<Matchee>`. Otherwise, it returns `None`.
fn match_one_char(ch : char) -> Box<dyn Fn(Matchee) -> Option<Matchee>>{
    return match_range(ch, ch)
}

/// return a closure such that
/// if the codepoint of the 1st char of `Matchee.matched` between
/// that of `lower_ch` (lower bound) and that of `upper_ch` (upper bound)
/// then return the new updated `Some<Matchee>`.
/// Otherwise, it returns `None`.
/// 
fn match_range(lower_ch : char, upper_ch: char) ->
    Box<dyn Fn(Matchee) -> Option<Matchee>> {
    Box::new(move | x : Matchee| -> Option<Matchee> {
        let x_remained_str = x.remained.as_str();
        let x_remained_char_vec = str_to_char_vec(x_remained_str);

        if (x_remained_char_vec[0] as u32) >= (lower_ch as u32) && 
        (x_remained_char_vec[0] as u32) <= (upper_ch as u32){
            let remained_string = x_remained_char_vec[1..].iter()
                                    .collect::<String>();
            return Some(Matchee{
                matched : x.matched + &x_remained_char_vec[0].to_string(),
                remained : remained_string,
            });
        }else{
            return None;
        }
    })
}

///
/// like the infix `==>` monad, i.e. `inputee ==> closure` in OCaml,
/// return a combinable closure.
/// - `inputee` : input string wrapped by Some() or None
/// - `closure` : the input to be processed
fn then_do(inputee : Option<Matchee>, closure : &dyn Fn(Matchee) -> Option<Matchee>)
    -> Option<Matchee>{
        return match inputee {
            Some(inner) => closure(inner),
            None => inputee,
        }
    }

/// return a combined closure. if `closure1` is not passed, then 
/// use `closure2`, i.e. : `(closure1 || closure2)`
fn or_do(
    closure1 : Box<dyn Fn(Matchee) -> Option<Matchee>>,
    closure2 : Box<dyn Fn(Matchee) -> Option<Matchee>>) -> 
    Box<dyn Fn(Matchee) -> Option<Matchee>>{
        Box::new(
            move |inputee|{
                let inputee_after_c1 = closure1(inputee.clone());
                match inputee_after_c1 {
                    None => closure2(inputee.clone()),
                    _ => inputee_after_c1,
                }
            }
        )    
    }


fn main() {
    let ex1 = Matchee{
                matched : string!(""),
                remained : string!("112")};

    let d = match_range('0', '9');
    println!("{:?}", then_do(then_do(then_do(Some(ex1.clone()), &d), &d), &d));

    println!("{:?}", (ex1.clone()));
    println!("{:?}", match_range('2', '9')(ex1.clone()));
    println!("{:?}", match_one_char('0')(ex1.clone()));
    println!("{:?}", match_one_char('1')(ex1.clone()));

    let ex2 = Matchee{
        matched : string!(""),
        remained : string!("012")};
        println!("~~~{:?}",
        then_do(then_do(Some(ex2.clone()), &or_do(match_one_char('1'),
            match_one_char('0'))),&d));



}


