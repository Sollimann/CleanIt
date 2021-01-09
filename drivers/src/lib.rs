mod roomba;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::roomba::reading::list_ports;

    #[test]
    fn list_available_ports_test() {
        list_ports()
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
