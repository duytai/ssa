contract Identifier {
  address owner;
  function pay(uint amount) {
    if (msg.sender.send(amount)) {
      msg.sender.send(amount);
    }
  }
}
