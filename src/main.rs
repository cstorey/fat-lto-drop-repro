use scopeguard::defer;

fn main() {
    defer!({
        println!("I am being dropped!");
    });
    
    println!("About to panic");
    let x : Option<()> = None;
    x.expect("???");
}
