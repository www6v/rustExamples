
use hello_rust::base::base::say_hi_test;
use hello_rust::base::base::module_test;
use hello_rust::base::base::copy_test;
use hello_rust::base::base::move_test;
use hello_rust::base::base::borrow_test;
use hello_rust::base::base::fetch_ignore_error;
use hello_rust::base::base::fetch_with_error;
use hello_rust::base::rc::rc_test;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        say_hi_test();
        module_test();
        fetch_ignore_error();
        fetch_with_error();
        copy_test();
        move_test();
        borrow_test();
        rc_test();
    }

    #[test]
    fn test_in_prograss() {
      
    }
}





