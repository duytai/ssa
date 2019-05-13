contract Identifier {
  struct Voter { uint voted; }
  uint[] amounts;
  uint amount;
  function test() {
    Voter voter;
    Voter[] voters;
    amounts = [10, 20, 30];
    amount = 40;
    voter.voted = amount * 1000;
    msg.sender.send(amount);
    msg.sender.send(amounts[10]);
    msg.sender.send(voter.voted);
    msg.sender.send(voters[20].voted);
    msg.sender.send(voters[20].voted);
  }
}
