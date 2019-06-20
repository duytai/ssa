pragma solidity ^0.4.24;

contract DataFlow {
  function main() {
    uint x = 10;
    x++ + 10;
    x-- + 10;
    ++x + 10;
    --x + 10;
    x + 20;
  }
}
