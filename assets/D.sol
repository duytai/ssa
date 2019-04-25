contract D {
  uint balance = 0 ;
  function pay(uint x, uint y) {
    if (balance > x) {
      balance -= x; 
    } else {
      balance -= 10 + y;
    }
    msg.sender.send(balance);
  }
}
