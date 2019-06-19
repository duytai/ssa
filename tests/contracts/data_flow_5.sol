pragma solidity ^0.4.24;

contract DataFlow {
  struct Voter { uint count; }
  function main(uint x) returns(uint) {
    Voter v0;
    Voter v1;
    v0.count += x;
    v1.count += 1;
    v1 = v0;
    v1.count += x;
    v1.count = 0;
    v1.count += x;
  }
}
