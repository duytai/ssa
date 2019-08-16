contract Sample {
  function main(uint x) {
    if (x > 0) {
      if (x < 100) {
        msg.sender.send(100);
      }
    }
  }
}
