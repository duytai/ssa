pragma solidity ^0.4.24;

import "./Base.sol";
import "./Hello.sol";

contract Math is Base {
  function add(uint x, uint y) returns (uint) {
    return x + y;
  }
}
