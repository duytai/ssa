pragma solidity ^0.4.24;

contract Sample {
  uint x = 100;
  struct Voter { // Struct
    uint weight;
    bool voted;
    address delegate;
    uint vote;
  }
  constructor() public {
    x = 1000;
  }

  function add(uint step) public {
    x += step;
  }
}
