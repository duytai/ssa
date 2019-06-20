pragma solidity ^0.4.24;

contract DataFlow {
  function main() {
    uint x = 0;
    msg.sender.send(x++);
    x + 20;
  }
}
