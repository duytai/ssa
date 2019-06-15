pragma solidity ^0.4.24;

contract DoWhileStatement {
  function main() public pure {
    uint x = 0;
    do { 
      break;
      x += 1;
    } while(true);
  }
}
