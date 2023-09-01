import { ReactElement } from 'react';

import { Color } from '../types.ts';

/**
 * Component properties.
 */
type Props = {
  readonly className?: string;
  readonly colors?: Color[];
};

/**
 * Toolbox component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function Toolbox(props: Props): ReactElement {
  const { className, colors } = props;
  return (
    <div className={`flex flex-col w-60 rounded bg-slate-200 ${className || ''}`}>
      <div className="flex-none bg-slate-950 px-4 py-2">
        <h2 className="text-base text-slate-50 font-bold">Color palette</h2>
      </div>
      <div className="flex-1 overflow-y-auto box-border p-4">
        <ul className="flex flex-col gap-4">
          {colors &&
            colors.map(({ hex: color }) => {
              const style = {
                backgroundColor: color,
              };

              return (
                <li key={color} className="flex flex-row items-center">
                  <div className="flex-none w-12 h-12 rounded shadow-xl mr-2" style={style}></div>
                  <span className="text-base text-slate-950 font-semibold leading-tight select-none ">
                    {color.toUpperCase()}
                  </span>
                </li>
              );
            })}
        </ul>
      </div>
      <div className="flex-none bg-slate-950 px-4 py-2">to be implemented</div>
    </div>
  );
}

export default Toolbox;
