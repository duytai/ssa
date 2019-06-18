pragma solidity ^0.4.24;

contract Block {
  function and(bool u, bool v) public pure returns (bool) {
    return u && v;
  }
  function main() public view {
    bool x = 0 == 1;
    x = false;
    this.and(false, true);
    this.and(true, this.and(false, false));
    x = this.and(false, x);
  }
}
