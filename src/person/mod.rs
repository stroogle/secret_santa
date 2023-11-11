pub struct Person {
    name: String,
    number: String
}

impl Person {
    pub fn new(name: &str, number: &str) -> Self {
        Person {
            name: String::from(name),
            number: String::from(number)
        }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn get_number(&self) -> String {
        String::from(&self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_person() {
        let p = Person::new("John Doe", "0474123456");
        assert_eq!(p.get_name(), "John Doe");
        assert_eq!(p.get_number(), "0474123456");
    }

}