pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        eat_at_restaurant();
    }
    #[test]
    fn another() {
        panic!("error haha ")
    }
}

mod front_of_house;

pub mod back_of_house;
use back_of_house::Appetizer;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");
    println!("我喜欢{}", meal.toast);
    let order1 = Appetizer::Salad;
}
