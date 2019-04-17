pragma solidity ^0.4.24;

contract Sample {
  uint x = 100;
  struct Voter { // Struct
    uint weight;
    bool voted;
    address delegate;
    uint vote;
  }
  constructor(uint val) public {
    uint k = 100;
    if (val + k > 0) {
      x = val;
    }
    x = x * 2;
  }

  function add(uint step) public {
    x += step;
  }
}
