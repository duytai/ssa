contract Identifier {
  function test() {
    uint x = 10;
    uint y = x;
    msg.sender.send(x);
  }
}
