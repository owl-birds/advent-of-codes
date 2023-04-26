function* fibGenerator(): Generator<number, any, number> {
  let prev_num = 0;
  let curr_num = 1;
  yield prev_num;
  yield curr_num;

  // inifinite loop
  while (true) {
    const temp_num = curr_num;
    curr_num += prev_num;
    prev_num = temp_num;
    yield curr_num;
  }
}

/**
 * const gen = fibGenerator();
 * gen.next().value; // 0
 * gen.next().value; // 1
 */
