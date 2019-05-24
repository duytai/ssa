contract Identifier {
  function test() {
    uint x = 10;
    while (x > 0) {
      x = 100;
      if (x > 0) {
        x += 10;
      }
    }
    msg.sender.send(x);
  }
}
