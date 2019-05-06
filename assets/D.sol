library Math {
  function div(uint x, uint y) returns (uint) {
    return x / y;
  }
}

library Math1 {
  function div(uint x, uint y) returns (uint) {
    return x / y;
  }
}

contract T {
}

contract A {
  function add(uint x, uint y) returns (uint) {
    return x + y;
  }
}

contract B {
  using Math for uint;
  enum Bool { TRUE, FALSE }
  struct Voter { string name; Bool val; } // struct definition
  function mul(uint x, uint y) returns (uint) {
    return x * y;
  }
  function lol() payable {
  }
}

contract D is A, B {
  using Math for uint;
  enum State { Created, Locked, Inactive } // enum definition
  event Deposit(uint value, State t); // event definition
  uint k = 10; // state variable
  Voter v;
  State st;
  T t = new T();
  function pay(uint x, uint y) {
    k = this.mul(100, 200);
    x.div(y) + block.timestamp;
    block.blockhash(12);
    this.lol();
    msg.sender.send(100);
  }
}
