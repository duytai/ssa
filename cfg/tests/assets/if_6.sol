pragma solidity ^0.4.24;

contract IfStatement {
  function and(bool u, bool v) public pure returns (bool) {
    return u && v;
  }
  function main() public view {
    uint x = 0;
    if (this.and(true, false)) x += 1; else x -= 1;
  }
}
