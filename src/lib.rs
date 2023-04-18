pub fn important_public_fn() -> () {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn holochain_works() {
        println!("Hello, world!");
    }
}

