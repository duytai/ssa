contract Identifier {
  struct Tuple {
    uint x;
    uint y;
  }

  function test() {
    Tuple t0;
    uint x = t0.x;
    while (x > 0) {
      x += 10;
    }
    msg.sender.send(x);
  }
}
