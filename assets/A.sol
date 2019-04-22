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
  function nested_call() {
    C c = new C();
    x = this.add(10, 20);
    x.add(40);
    c.add(50, 60);
    this.mul(30, 40);
    x = 100;
    revert();
  }

  function mul(uint m, uint n) returns(uint) {
    return m * n; 
  }
}
