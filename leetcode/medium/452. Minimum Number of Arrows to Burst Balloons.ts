function findMinArrowShots(points: number[][]): number {
  // first short the array, by the x-end of the array (ballons)
  const temp_points: number[][] = [...points].sort((a, b) => a[1] - b[1]);
  // points.sort((a,b)=>a[1]-b[1]);
  let temp_idx: number = 0;

  // we shoot the end of the ballon for maximum ballon to burst
  let total_arrows: number = 1;
  let shoot: number = temp_points[temp_idx][1];
  while (temp_idx < temp_points.length) {
    if (
      temp_points[temp_idx][1] >= shoot &&
      temp_points[temp_idx][0] <= shoot
    ) {
      // arrow hit the current ballon
      temp_idx += 1;
      continue;
    }
    // if the arrow missed the current ballon, we shoot an arrow again at the end of the current ballon
    shoot = temp_points[temp_idx][1];
    total_arrows += 1;
    temp_idx += 1;
  }
  return total_arrows;
}
