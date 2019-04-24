contract D {
  uint balance = 0 ;
  function pay(uint x) {
    if (balance > x) {
      balance -= x; 
    }
    msg.sender.send(x);
  }
}
