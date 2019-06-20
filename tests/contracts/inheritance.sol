pragma solidity ^0.4.24;

contract X {
  uint a;
  uint b;
}

contract Y {
  uint c;
  uint d;
}

contract Z is X, Y {
  uint e;
  uint f;
  function main() {}
}
