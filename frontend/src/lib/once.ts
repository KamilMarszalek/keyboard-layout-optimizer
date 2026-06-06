export function once<T, A>(fn: (arg?: A) => Promise<T>): (arg?: A) => Promise<T> {
  let promise: Promise<T> | undefined;
  return (arg?: A) => {
    if (!promise) {
      promise = fn(arg).catch((err) => {
        promise = undefined;
        throw err;
      });
    }
    return promise;
  };
}
