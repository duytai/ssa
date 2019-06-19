pragma solidity ^0.4.24;

contract DataFlow {
  uint x;
  function main(uint n) returns(uint) {
    x += 100;
    x = n;
    return x;
  }
}
