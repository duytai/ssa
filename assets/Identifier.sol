contract Identifier {
  function test() {
    uint x = 0;
    if (true) {
      x += 10;
    } else if (false) {
      x += 20;
    }
  }
}
