contract Identifier {
  address owner;
  function test(uint x) {
    selfdestruct(this);
    x += 10;
  }
}
