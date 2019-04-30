pragma solidity ^0.4.24;

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
