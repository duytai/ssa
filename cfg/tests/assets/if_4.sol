pragma solidity ^0.4.24;

contract IfStatement {
  function main() public pure {
    uint x = 0;
    if (true) {
      x += 1;
    } else {
      x -= 1;
    }
  }
}
