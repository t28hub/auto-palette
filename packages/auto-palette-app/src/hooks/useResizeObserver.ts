import { RefObject, useEffect, useRef } from 'react';

/**
 * The callback invoked when the size of the element changes.
 */
export type ResizeCallback = (entry: ResizeObserverEntry) => void;

/**
 * The return type of the `useResizeObserver` hook.
 *
 * @typeparam T - The type of element to observe.
 */
export type ReturnType<T extends Element> = {
  /**
   * The ref to attach to the element to observe.
   */
  readonly ref: RefObject<T>;
};

/**
 * Hook that observes the size of an element.
 *
 * @param callback - The callback to invoke when the size of the element changes.
 * @param options - The options to use when observing the element.
 * @returns The return type of the `useResizeObserver` hook.
 */
export function useResizeObserver<T extends Element>(
  callback: ResizeCallback,
  options?: ResizeObserverOptions,
): ReturnType<T> {
  const elementRef = useRef<T>(null);

  useEffect(() => {
    const element = elementRef.current;
    if (element === null) {
      return;
    }

    const resizeObserver = new ResizeObserver((entries) => {
      entries.forEach((entry) => callback(entry));
    });
    resizeObserver.observe(element, options);

    return () => {
      resizeObserver.unobserve(element);
      resizeObserver.disconnect();
    };
  }, [callback, options, elementRef.current]);

  return { ref: elementRef };
}
