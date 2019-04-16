pragma solidity ^0.4.24;

contract Sample {
  uint x = 100;
  constructor() public {
    x = 1000;
  }

  function add(uint step) public {
    x += step;
  }
}
