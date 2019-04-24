contract D {
  struct Voter { string name; }
  uint balance = 0 ;
  Voter voter;
  uint[] balances;
  function pay(uint x, uint y) {
    if (balance > x) {
      balance -= x; 
    }
    msg.sender.send(x);
  }
}
