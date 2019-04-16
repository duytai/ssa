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
    if (val > 0) {
      x = val;
    }
  }

  function add(uint step) public {
    x += step;
  }
}
