#[cfg(not(target_arch = "arm"))]
fn main() {
    println!("\nHello from non-ARM arch!\n");
}

#[cfg(target_arch = "arm")]
fn main() {
    println!("\nHello from ARM arch!\n");
}

