contract Identifier {
  address owner;
  modifier isOwner(bool n) {
    if (msg.sender == owner) {
      _;
    } else {
      _;
    }
  } 
  function test(uint x) isOwner(x == 20) {
    x += 10;
  }
}
