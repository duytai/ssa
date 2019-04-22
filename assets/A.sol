library SafeMath {
  function add(uint x, uint y) returns(uint) {
    return x + y;
  }
}

contract B {
  function add(uint x, uint y) returns(uint) {
    return x + y;
  }
}

contract C {
  bool locked = true;
  constructor() {
    locked = false;
  }
  function add(uint x, uint y) returns(uint) {
    return x + y;
  }
}

contract A is B {
  using SafeMath for uint;
  uint x = 0;
  address owner;
  function nested_call() {
    x = 100 + x.add(200) + x.add(0) > 100 ? 0 : this.add(20, 30);
    C c = new C();
    x = this.add(10, 20);
    x.add(40);
    c.add(50, 60);
    uint k = this.mul(30, this.add(20, 20));
    x = 100;
    owner.send(100);
    revert();
  }

  function mul(uint m, uint n) returns(uint) {
    return m * n; 
  }
}
