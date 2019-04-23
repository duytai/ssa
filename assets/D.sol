contract D {
  struct Voter {
    string name;
    uint age;
  }
  uint x = 0;
  Voter voter;
  constructor() {
    if (x > 0) {
      uint x = 100;
      x += block.number;
    }
  }
}
