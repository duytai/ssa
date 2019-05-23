contract Identifier {
  function test() {
    uint x = 10;
    while (x > 0) {
      x = 100;
    }
    msg.sender.send(x);
  }
}
