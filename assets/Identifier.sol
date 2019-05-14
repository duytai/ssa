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
  function test() {
    uint x = 10;
    uint amount = x;
    msg.sender.send(amount);
  }
}
