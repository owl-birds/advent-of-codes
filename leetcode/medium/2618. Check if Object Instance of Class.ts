function checkIfInstanceOf(obj: any, classFunction: any): boolean {
  // console.log(obj["__proto__"]);
  // console.log(classFunction.prototype);
  // Object.getPrototypeOf(Object.getPrototypeOf(d))
  // Object.getPrototypeOf(d).constructor
  let temp_obj = obj;
  while (temp_obj != null) {
    if (temp_obj.constructor === classFunction) return true;
    temp_obj = Object.getPrototypeOf(temp_obj);
  }
  return false;
}

/**
 * checkIfInstanceOf(new Date(), Date); // true
 */
