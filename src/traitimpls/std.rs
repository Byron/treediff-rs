
use traitdef::Value;

impl Value for String {
    fn is_scalar(&self) -> bool {
        true
    }

    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

