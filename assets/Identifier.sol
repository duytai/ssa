//contract Identifier {
//  struct Voter { uint voted; }
//  uint[] amounts;
//  function test() {
//    Voter voter;
//    Voter[] voters;
//    uint amount;
//    amount = voter.voted;
//    amount = 10;
//    msg.sender.send(amount);
//    msg.sender.send(amounts[10]);
//    msg.sender.send(voter.voted);
//    msg.sender.send(voters[20].voted);
//  }
//}

contract Identifier {
  struct Voter {
    uint voted;
  }
  function test() {
    Voter v0;
    Voter v1;
    v0 = v1;
    msg.sender.send(v0.voted);
  }
}
