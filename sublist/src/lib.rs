#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // let first_iter = _second_list.windows(size);
    // let second_iter = _second_list.into_iter();
    match _first_list.into_iter().eq(_second_list.into_iter()) {
        // They are totally equal , including size , value of the elements 
        true => Comparison::Equal,
        // part 1
        // if the first list is empty and the second list is not empty
        // then we should know that the first list is sub list of the second list 
        // part 2
        // grabs a list with the size of list A in B
        // and compares the part that is picked in B with the list A items 
        false => match (_first_list.is_empty() && !_second_list.is_empty()) || _second_list.windows(_first_list.len()).any(|x| x == _first_list)  {
            // all the elements in first list are in the second list 
            // and because they are not equal then the first list contains more elements
            // which that means that the first list is a sublist of the second list
            true => Comparison::Sublist,
            // the lists are not equal neither the first list is the sublist of the second list
            // part 1
            // if the second list is empty and the first list is not empty
            // then we should know that the first list is super list of the second list 
            // part 2
            // grabs a list with the size of list B in A
            // and compares the part that is picked in B with the list A items             
            false => match (_second_list.is_empty() && !_first_list.is_empty()) || _first_list.windows(_second_list.len()).any(|x| x == _second_list) {
                // all the elements in second list are in the first list 
                // and because they are not equal then the second list contains more elements
                // which that means that the second list is a superlist of the first list
                true => Comparison::Superlist,
                // neither the lists are equal nor the first list is the sublist || superlist of the second list
                false => Comparison::Unequal,
            } 
        }        
    }
}

