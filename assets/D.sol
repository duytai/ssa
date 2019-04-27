library Math {
  function div(uint x, uint y) returns (uint) {
    return x / y;
  }
}

contract A {
  function add(uint x, uint y) returns (uint) {
    return x + y;
  }
}

contract B {
  function mul(uint x, uint y) returns (uint) {
    return x * y;
  }
}

contract D is A, B {
  using Math for uint;
  function pay(uint x, uint y) {
  }
}
