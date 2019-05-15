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
    uint x = now;
    uint y = x * 100;
    uint z;
    z = y + 100;
    msg.sender.send(z);
  }
}
