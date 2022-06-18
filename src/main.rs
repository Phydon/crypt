const LST: [char; 13] = [
        'a', 
        'b', 
        'c', 
        'd', 
        'e', 
        'f', 
        'g', 
        'h', 
        'i', 
        'j', 
        'k', 
        'l', 
        'm'
    ];

#[derive(Debug)]
enum Key {
    K1(u8),
    K2(u8),
}

#[derive(Debug)]
struct ThreeCrypt {
    key: Key,
    lst: [char; 13],
}

fn main() {
    let code = ThreeCrypt {
        key: Key::K1(3),
        lst: LST,
    };

    println!("code: {:?}", code);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let indeed: bool = true;
        assert!(indeed);
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        panic!();
    }
}
