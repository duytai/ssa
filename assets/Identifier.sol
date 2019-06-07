contract Identifier {
  address owner;
  function pay(uint amount) {
    if (amount < 1000) {
      msg.sender.send(amount);
    }
  }
}
