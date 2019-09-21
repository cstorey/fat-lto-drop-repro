struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping a Droppable");
    }
}

fn main() {
    let _guard = Droppable;
    
    println!("About to panic");
    let x : Option<()> = None;
    x.expect("???");
}
