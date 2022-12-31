// A bus has n stops numbered from 0 to n - 1 that
// form a circle.We know the distance between all
// pairs of neighboring stops where distance[i]
// is the distance between the stops number i
// and(i + 1) % n.

// The bus goes along both directions i.e. clockwise
// and counterclockwise.

// Return the shortest distance between the given
// start and destination stops.
function distanceBetweenBusStops(
  distance: number[],
  start: number,
  destination: number
): number {
  // increasing and decreasingi index
  let clockwise = 0;
  let counterclockwise = 0;
  for (let i = start; i !== destination; i += 1) {
    if (i === distance.length) i = 0;
    clockwise += distance[i];
    if (i === destination - 1) break;
    if (destination - 1 === -1 && i === 0) {
      clockwise -= distance[i];
      break;
    }
  }
  for (let i = start - 1; i !== destination - 1; i -= 1) {
    if (i === -1) i = distance.length - 1;
    if (i === destination - 1) break;
    counterclockwise += distance[i];
  }
  return Math.min(clockwise, counterclockwise);
}
console.log(
  distanceBetweenBusStops(
    [
      6, 47, 48, 31, 10, 27, 46, 33, 52, 33, 38, 25, 32, 45, 36, 3, 0, 33, 22,
      53, 8, 13, 18, 1, 44, 41, 14, 5, 38, 25, 48,
    ],
    22,
    0
  )
);
console.log(distanceBetweenBusStops([1, 2, 3, 4], 0, 2));
console.log(distanceBetweenBusStops([1, 2, 3, 4], 0, 3));
console.log(distanceBetweenBusStops([7, 10, 1, 12, 11, 14, 5, 0], 7, 2));
