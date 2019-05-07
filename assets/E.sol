pragma solidity ^0.4.24;

contract A {
  uint x = 10;
}
contract B is A {
  uint x = 20;
  function hello() {
    uint x = 50;
  }
}
contract E {
  uint startAt;
  mapping(address => uint) balances;
  function pay(uint amount) {
    if(amount <= balances[msg.sender]) {
      amount += (now - startAt);
    }
    msg.sender.transfer(amount);
  }
}
