import clsx from 'clsx';
import { FormEvent, ReactElement, useCallback, useState } from 'react';

import { useAppDispatch } from '../hooks';
import { setImageUrl } from '../store';

import { FileInput, FormLabel, TextInput } from './';

/**
 * Component properties for Sidebar.
 */
interface Props {
  readonly className?: string;
}

/**
 * Sidebar component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function Sidebar(props: Props): ReactElement {
  const { className } = props;
  const dispatch = useAppDispatch();
  const [url, setUrl] = useState<string>('');

  const onFileSelect = useCallback((file: File | File[]): void => {
    const selected = Array.isArray(file) ? file[0] : file;
    const imageUrl = URL.createObjectURL(selected);
    dispatch(setImageUrl({ url: imageUrl }));
  }, []);

  const onFileSelectError = useCallback((error: Error): void => {
    console.warn(error);
  }, []);

  const onUrlChange = useCallback((value: string): void => {
    console.info(`URL changed: ${value}`);
    setUrl(value);
  }, []);

  const onUrlSubmit = useCallback(
    (event: FormEvent<HTMLFormElement>): void => {
      event.preventDefault();
      console.info(`Loading image from URL: ${url}`);
      dispatch(setImageUrl({ url }));
    },
    [url],
  );

  return (
    <section
      className={clsx(
        'flex',
        'flex-col',
        'items-stretch',
        'justify-center',
        'rounded',
        'bg-gray-100/50',
        'backdrop-blur-xl',
        'shadow-2xl',
        className,
      )}
    >
      <div className="flex-shrink-0 flex items-center w-full px-4 py-2 border-b border-gray-400/80">
        <h2 className="text-lg text-gray-900 font-semibold select-none">Image</h2>
      </div>
      <div className="flex-1 flex flex-col p-4 pt-2 border-b border-gray-400/80">
        <FormLabel className="flex-shrink-0 pb-2">Image File:</FormLabel>
        <FileInput
          className="w-full h-48"
          types={['image/jpeg', 'image/png']}
          multiple={false}
          required={true}
          minSize={1}
          maxSize={1024 * 1024} // 1MB
          onSelect={onFileSelect}
          onError={onFileSelectError}
        />
      </div>
      <form className="flex-shrink-0 flex flex-col w-full p-4" onSubmit={onUrlSubmit}>
        <FormLabel className="flex-shrink-0 pb-2">Image URL:</FormLabel>
        <TextInput className="w-full opacity-80" type="url" placeholder="Enter an image URL" onChange={onUrlChange} />
      </form>
    </section>
  );
}

export default Sidebar;
