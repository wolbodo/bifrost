type TimeoutId = ReturnType<typeof setTimeout>;

export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeoutId: TimeoutId | undefined;
  let latestArgs: Parameters<T> | undefined;

  return function debounced(...args: Parameters<T>) {
    clearTimeout(timeoutId);

    if (!timeoutId) {
      func(...args);
    } else {
      latestArgs = args;
    }

    timeoutId = setTimeout(() => {
      timeoutId = undefined;
      if (latestArgs) {
        func(...latestArgs);
        latestArgs = undefined;
      }
    }, wait);
  };
}
