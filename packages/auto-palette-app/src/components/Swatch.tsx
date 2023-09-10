import clsx from 'clsx';
import { ReactElement } from 'react';

/**
 * Component properties for Swatch.
 */
interface Props {
  readonly className?: string;
  readonly color: string;
  readonly size?: number;
  readonly x?: number;
  readonly y?: number;
}

/**
 * Color swatch component
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function Swatch({ className = '', color = '#FFFFFF', size = 32, x = 0, y = 0 }: Props): ReactElement {
  const style = {
    backgroundColor: color,
    width: size,
    height: size,
    left: x,
    top: y,
  };

  return (
    <div
      className={clsx(
        'flex',
        'justify-center',
        'items-center',
        'absolute',
        'z-10',
        '-translate-x-1/2',
        '-translate-y-1/2',
        'cursor-pointer',
        'drop-shadow-md',
        'rounded-full',
        'border-white',
        'border-solid',
        'border-4',
        'border-opacity-90',
        className,
      )}
      style={style}
    ></div>
  );
}

export default Swatch;
