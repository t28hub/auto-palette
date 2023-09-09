import clsx from 'clsx';
import { ChangeEvent, ReactElement, useCallback } from 'react';

/**
 * Component properties for TextInput.
 */
interface Props {
  readonly className?: string;
  readonly type?: 'text' | 'password' | 'email' | 'number' | 'tel' | 'url';
  readonly placeholder?: string;
  readonly required?: boolean;
  readonly onChange?: (value: string) => void;
}

/**
 * Text input component.
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function TextInput(props: Props): ReactElement {
  const { className, type, placeholder, required, onChange } = props;

  const onInputChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>): void => {
      console.info(event.target.value);
      if (onChange) {
        onChange(event.target.value);
      }
    },
    [onChange],
  );

  return (
    <input
      className={clsx('p-2', 'rounded', 'border', 'border-gray-400', className)}
      type={type || 'text'}
      placeholder={placeholder || ''}
      required={required || false}
      onChange={onInputChange}
    />
  );
}

export default TextInput;
