extern crate l10_1;

fn main() {
    l10_1::public_function();

    // Error! 'private_function' is private
    //l10_1::private_function();

    l10_1::indirect_access();
}

