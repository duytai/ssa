contract D {
  uint x = 0;
  constructor() {}
  function t() {
    if (x > 0) {
      if (x < 1000) {
        msg.sender.send(1);
      }
    }
  }
}
