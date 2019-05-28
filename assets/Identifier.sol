contract Identifier {
  address owner;
  modifier isOwner(bool n) {
    require(n);
    _;
  } 
  function test(uint x) isOwner(x == 20) {
    x += 10;
  }
}
