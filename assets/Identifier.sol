contract Identifier {
  address owner;
  modifier isOwner() {
    require(msg.sender == owner);
    _;
  } 

  function test(uint x, uint y) {
    while (true) {
      x += y;
    }
  }
}
