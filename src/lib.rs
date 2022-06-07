#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list==_second_list {
        return Comparison::Equal
    }
    if _first_list.len() < _second_list.len() {
        for (x,y) in _first_list.iter().zip(_second_list.iter()){
            if x==y {
                println!("Items are equal: {} {}",x,y);
            }

        }

    }
    Comparison::Unequal
}
