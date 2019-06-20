pragma solidity ^0.4.24;

contract DataFlow {
  mapping(uint => uint) balances;
  function add(uint x, uint y) returns(uint) {
    return x + y;
  }
  function balance_of(uint x, uint y) returns(uint) {
    uint k = balances[x + this.add(x, y)];
    return k;
  }
}
