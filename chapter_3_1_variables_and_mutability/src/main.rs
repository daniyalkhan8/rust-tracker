fn main() {
    // Variables Mutability
    let mut x = 5;
    println!("{x}");

    x = 6;
    println!("{x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let shadow1 = 5;
    let shadow1 = shadow1 + shadow1;

    {
        let shadow1 = shadow1 * 2;
        println!("{shadow1}");
    }

    println!("{shadow1}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}")
}
