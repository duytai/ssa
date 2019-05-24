contract Identifier {
  function test() {
    uint x;
    while (x > 0) {
      x = 10;
      x = 20;
    }
    msg.sender.send(x);
  }
}
