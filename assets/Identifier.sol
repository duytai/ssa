contract Identifier {
  address owner;
  modifier isOwner(bool n) {
    if (msg.sender == owner) {
      _;
    } else {
      _;
    }
  } 
  uint z = 10;
  function test(uint x, uint y) {
    while (true) {
      if (msg.sender.send(x)) {
        y += x;
        z += y;
      }
    }
  }
}
