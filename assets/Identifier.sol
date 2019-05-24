contract Identifier {
  struct Tuple {
    uint x;
    uint y;
  }

  function test() {
    Tuple t0;
    msg.sender.send(t0.x);
  }
}
