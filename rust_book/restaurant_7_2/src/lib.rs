mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order(){}
}

fn deliver_order(){}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();

}
