pragma solidity ^0.4.24;

contract WhileStatement {
  function main() public pure {
    uint x = 0;
    while(true) {
      x += 1;
      continue;
      x -= 1;
    }
  }
}
