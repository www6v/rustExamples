

use hello_rust::base::rc::rc_test;
use hello_rust::base::trait_mod::copy_trait_test;
use hello_rust::base::trait_mod::drop_trait_test;
use hello_rust::base::trait_mod::refcell_is_send;
use hello_rust::base::trait_mod::arc_mutext_is_send_sync;

use hello_rust::base::base::Base;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let mut cb_ctx = Base::new();

        Base::module_test();
        Base::fetch_ignore_error();
        Base::fetch_with_error();  
        Base::say_hi_test();

        cb_ctx.move_test();
        cb_ctx.borrow_test();
        cb_ctx.copy_test();

        rc_test();
    }

    #[test]
    fn test_in_prograss() {
        copy_trait_test();
        drop_trait_test();
        refcell_is_send();
        arc_mutext_is_send_sync();
    }
}






