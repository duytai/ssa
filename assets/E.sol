pragma solidity ^0.4.24;

contract E {
  uint startAt = block.timestamp;
  mapping(address => uint) balances;
  struct Counter {
    uint count;
  }
  struct Human {
    uint amount;
    Counter counter;
  }

  function echo(uint x) returns (uint) {
    return x;
  }

  function pay(uint[] amounts) {
    uint amount = 0;
    Human human;
    human.amount = 10;
    human.counter.count = 20;
    if(amounts[0] <= balances[msg.sender]) {
      amounts[0] += (now - startAt);
    } else {
      amounts[0] = 0;
    }
    amounts[0] = 10;
    msg.sender.call.value(amounts[0])();
    msg.sender.send(amount);
    msg.sender.send(human.amount);
  }
}
