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
 * Default swatch color.
 */
const DEFAULT_COLOR = '#FFFFFF';

/**
 * Default swatch size.
 */
const DEFAULT_SIZE = 32;

/**
 * Color swatch component
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function Swatch(props: Props): ReactElement {
  const { className, color, size, x = 0, y = 0 } = props;

  const validColor = /^#[0-9a-f]{6}$/i.test(color) ? color : DEFAULT_COLOR;
  const validSize = size && size > 0 ? size : DEFAULT_SIZE;

  const style = {
    backgroundColor: validColor,
    width: validSize,
    height: validSize,
    left: x,
    top: y,
  };

  return (
    <div
      className={`flex justify-center items-center absolute z-10 translate-x-1/2 translate-y-1/2 cursor-pointer drop-shadow-md rounded-full border-white border-solid border-4 border-opacity-90 ${
        className || ''
      }`}
      style={style}
    ></div>
  );
}

export default Swatch;
