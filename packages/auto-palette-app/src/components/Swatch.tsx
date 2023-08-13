import { Position } from 'auto-palette';
import { ReactElement } from 'react';

/**
 * Component properties.
 */
type Props = {
  readonly className?: string;
  readonly color: string;
  readonly position: Position;
  readonly size?: number;
};

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
  const { className, color, position, size } = props;

  const validColor = /^#[0-9a-f]{6}$/i.test(color) ? color : DEFAULT_COLOR;
  const validSize = size && size > 0 ? size : DEFAULT_SIZE;

  const style = {
    backgroundColor: validColor,
    width: validSize,
    height: validSize,
    left: position.x,
    top: position.y,
  };

  return (
    <div
      className={`flex justify-center items-center absolute z-10 translate-x-1/2 translate-y-1/2 cursor-pointer drop-shadow-md rounded-full border-white border-solid border-4 border-opacity-90 ${className}`}
      style={style}
    ></div>
  );
}

export default Swatch;
