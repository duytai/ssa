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
    uint kanzo = k % 10 == 0 ? 10 : 20;
    if (val + k > 0) {
      x = val;
      if (x == val) {
        x + val;
      } else {
        x - val;
      }
    } else {
      x+= val;
      revert();
    }
    require(x > 0);
    x = x * 2;
    while(k > 0) {
      x = x + 3;
      if(k == 10) continue;
      if(k == 90) break;
      for (uint i = 0; i < 100; i++) {
        x += 100;
      }
    }

    assert(k == 10);

    for(k = 0; x < 0;) x += 2; 
    for(;k < 0;) x += 2; 
    for(;x > 0;k++) x += 2; 
    for(;x == 0;) k += 7;

    do {
      x += 10;
    } while(x % 2 != 0);

    throw;

    if (x > 0) {
      x-= 1;
      return;
      x+= 1;
    }
    x += 999;
  }

  function add(uint step) public returns(uint) {
    x += step;
  }
}
