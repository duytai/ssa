contract Identifier {
  address owner;
  modifier isOwner(bool n) {
    selfdestruct(this);
    _;
    n = !n;
  } 
  function test(uint x) isOwner(x == 20) {
    x += 10;
  }
}
