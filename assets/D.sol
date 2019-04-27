contract D {
  struct Voter { uint counter; }
  uint balance = 0 ;
  uint[] balances;
  Voter voter;
  function pay(uint x, uint y) {
    if (balance > x) {
      voter.counter = 100;
      balance -= x + y * 10; 
    } else {
      voter.counter = 200;
      balance -= 10 + y;
    }
    msg.sender.send(balance);
  }
}
