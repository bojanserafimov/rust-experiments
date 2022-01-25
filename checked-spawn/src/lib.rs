
mod token {
    struct Private {}
    pub struct Public {
        _private: Private
    }

    pub fn make_token() -> Public {
        Public {
            _private: Private {}
        }
    }
}

fn foo() {
    // No other way to make token, have to call this function
    let token = token::make_token();
}
