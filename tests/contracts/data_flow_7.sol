pragma solidity ^0.4.24;

contract DataFlow {
  uint[] balances;
  function add(uint x, uint y) returns (uint) {
    return x + y;
  }
  
  function main() returns(uint) {
    uint x = 0;
    uint y = 0;
    y += x + this.add(10, this.add(x, x));
  }
}
