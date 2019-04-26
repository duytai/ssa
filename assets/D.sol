contract D {
  struct Voter { uint counter; }
  uint balance = 0 ;
  Voter v;
  function pay(uint x, uint y) {
    v.counter = 100;
    if (balance > x) {
      balance -= x + y * 10; 
    } else {
      balance -= 10 + y;
    }
    msg.sender.send(balance);
  }
}
