pragma solidity ^0.4.24;

contract E {
  uint startAt = now;
  mapping(address => uint) balances;

  function echo(uint x) returns (uint) {
    return x;
  }

  function pay(uint amount) {
    if(amount <= balances[msg.sender]) {
      amount += (now - startAt);
    } else {
      amount = 0;
    }
    amount = 10;
    //msg.sender.transfer(amount);
    //msg.sender.transfer(0);
    //msg.sender.send(amount);
    //msg.sender.send(0);
    msg.sender.call.value(amount)();
    //msg.sender.call();
    //msg.sender.callcode();
    //msg.sender.call(100);
  }
}
