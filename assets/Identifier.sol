contract Identifier {
  function test() {
    uint x = 10;
    x + 10;
    msg.sender.send(x);
  }
}
