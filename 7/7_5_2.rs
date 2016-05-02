fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for 'pair'
    
    println!("Tell me about {:?}", pair);
    println!("{}", match pair {
        (x, y) if x == y => "These are twins",
        // The 'if condition' part is a guard
        (x, y) if x + y == 0 => "Antimatter, kaboom!",
        (x, _) if x % 2 == 1 => "The first one is odd",
        _ => "No correlation..."
    });
}

