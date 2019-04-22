contract B {
  address owner;
  modifier onlyOwner() {
    require(msg.sender == owner);
    _;
  }

  function changeOwner(address o) onlyOwner {
    owner = o;
  } 
}
