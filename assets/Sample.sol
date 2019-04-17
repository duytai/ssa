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
      if (x == val) {
        x + val;
      } else {
        x - val;
      }
    } else {
      x+= val;
    }
    x = x * 2;
    while(k > 0) {
      x = x + 3;
    }
  }

  function add(uint step) public {
    x += step;
  }
}
