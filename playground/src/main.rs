// slices does not take the ownership of data 
fn main(){
    match (Foo("a"), Foo("b"), Foo("c")) {
        (a, _, c) => {
            println!("match arm");
        } // c, a dropped here
    }
}