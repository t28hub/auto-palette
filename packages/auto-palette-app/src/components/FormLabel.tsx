import clsx from 'clsx';
import { ReactElement, ReactNode } from 'react';

/**
 * Component properties for FormLabel.
 */
interface Props {
  readonly className?: string;
  readonly children: ReactNode;
}

/**
 * Form label component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function FormLabel(props: Props): ReactElement {
  const { className, children } = props;

  return (
    <label className={clsx('text-start', 'text-sm', 'text-gray-900', 'font-semibold', 'select-none', className)}>
      {children}
    </label>
  );
}

export default FormLabel;
