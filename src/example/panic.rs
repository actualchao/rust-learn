pub fn test_pinic() {
  pub struct Guess {
    value: i32,
  }
  impl Guess {
    pub fn new(number: i32) -> Guess {
      Guess { value: number }
    }

    pub fn value(&self) -> i32 {
      self.value
    }
  }

  let test_guess = Guess::new(32);

  crate::dev_log("will pinic on there, because the value morethan 100");

  let test_guess = Guess::new(120);

  
}
