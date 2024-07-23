use uuid::Uuid;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_works() {
        let id = Uuid::new_v4();
        println!("{}", id);
    }
}
