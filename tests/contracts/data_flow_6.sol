pragma solidity ^0.4.24;

contract DataFlow {
  uint[] balances;
  function main(uint x) returns(uint) {
    balances[0] += x;
    balances = [0, 1, 2];
    balances[1] += x;
  }
}
