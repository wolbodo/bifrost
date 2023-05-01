type TimeoutId = ReturnType<typeof setTimeout>;

export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeoutId: TimeoutId | undefined;

  return function debounced(...args: Parameters<T>) {
    clearTimeout(timeoutId);

    if (!timeoutId) {
      func(...args);
    }

    timeoutId = setTimeout(() => {
      timeoutId = undefined;
    }, wait);
  };
}
