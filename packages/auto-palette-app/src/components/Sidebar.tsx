import { ReactElement, useCallback } from 'react';

import { useAppDispatch } from '../hooks';
import { setImageUrl } from '../store';

import { FileInput } from './';

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

  const onFileSelect = useCallback((file: File | File[]): void => {
    const selected = Array.isArray(file) ? file[0] : file;
    const imageUrl = URL.createObjectURL(selected);
    dispatch(setImageUrl({ url: imageUrl }));
  }, []);

  const onFileSelectError = useCallback((error: Error): void => {
    console.warn(error);
  }, []);

  return (
    <div
      className={`flex flex-col items-stretch justify-center w-60 rounded bg-gray-100/80 backdrop-blur shadow-2xl ${
        className || ''
      }`}
    >
      <div className="flex-shrink-0 flex items-center w-full px-4 py-2 border-b border-gray-400">
        <h2 className="text-lg text-gray-900 font-semibold select-none">Image</h2>
      </div>
      <div className="flex-1 p-4">
        <FileInput
          className="w-full h-72"
          types={['image/jpeg', 'image/png']}
          multiple={false}
          required={true}
          minSize={1}
          maxSize={1024 * 1024} // 1MB
          onSelect={onFileSelect}
          onError={onFileSelectError}
        />
      </div>
    </div>
  );
}

export default Sidebar;
