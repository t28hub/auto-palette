import { ReactElement } from 'react';

import { useAppSelector } from '../hooks';
import { paletteSelector } from '../store';

/**
 * Component properties for Toolbar.
 */
interface Props {
  readonly className?: string;
}

/**
 * Toolbar component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function Toolbar(props: Props): ReactElement {
  const { className } = props;
  const palette = useAppSelector(paletteSelector);

  return (
    <div
      className={`flex flex-col items-stretch justify-center max-h-[80%] overflow-x-hidden overflow-y-auto rounded bg-gray-100/80 backdrop-blur shadow-2xl ${
        className || ''
      }`}
    >
      <div className="flex-shrink-0 flex items-center justify-start w-full px-4 py-2 border-b border-gray-400">
        <h2 className="text-lg text-gray-900 font-semibold select-none">Color Palette</h2>
      </div>
      <div className="flex-1 p-4 overflow-y-auto">
        {palette.status === 'loading' && <p className="text-gray-900 font-semibold">Loading...</p>}
        {palette.status === 'failed' && <p className="text-gray-900 font-semibold">Failed to load palette.</p>}
        {palette.status === 'succeeded' && (
          <ul className="flex flex-col gap-4">
            {palette.result.map(({ hex: color }) => {
              const style = {
                backgroundColor: color,
              };

              return (
                <li
                  key={color}
                  className="flex flex-row items-center hover:opacity-50 transition-opacity duration-500 cursor-pointer"
                >
                  <div className="flex-shrink-0 w-8 h-8 rounded mr-2" style={style}></div>
                  <span className="text-base text-gray-900 font-semibold leading-tight select-none ">
                    {color.toUpperCase()}
                  </span>
                </li>
              );
            })}
          </ul>
        )}
      </div>
      <div className="flex-shrink-0 flex items-center justify-start w-full px-4 py-2 border-t border-gray-400"></div>
    </div>
  );
}

export default Toolbar;
