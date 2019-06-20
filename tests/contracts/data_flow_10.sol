pragma solidity ^0.4.24;

contract DataFlow {
  function main() {
    uint x;
    delete x;
    x;
  }
}
