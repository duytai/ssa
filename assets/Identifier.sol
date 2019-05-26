contract Identifier {
  function add(uint x, uint y) returns(uint) {
    x + y;
  }
  function test() {
    uint x = 10;
    uint y = 20;
    while (this.add(x, this.add(x, y)) > 0) {
      x += y;
    }
  }
}
