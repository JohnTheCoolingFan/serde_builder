pub trait Validator<T> {
    fn validate(self, value: &T) -> Result<(), String>;
}

impl<T> Validator<T> for () {
    fn validate(self, _value: &T) -> Result<(), String> {
        Ok(())
    }
}

impl<T, FN: FnOnce(&T) -> Result<(), String>> Validator<T> for FN {
    fn validate(self, value: &T) -> Result<(), String> {
        self(value)
    }
}
