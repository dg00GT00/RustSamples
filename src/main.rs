mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::indirect_access_from_inaccessible();
    my::nested::function();
}
