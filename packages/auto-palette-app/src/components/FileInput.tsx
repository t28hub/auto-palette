import {
  ChangeEvent,
  DragEvent,
  MouseEvent,
  ReactElement,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';

/**
 * Component properties for FileInput.
 */
interface Props {
  readonly name?: string;
  readonly types?: string[];
  readonly className?: string;
  readonly children?: ReactElement | ReactElement[];
  readonly disabled?: boolean;
  readonly multiple?: boolean;
  readonly required?: boolean;
  readonly minSize?: number;
  readonly maxSize?: number;
  readonly onSelect?: (file: File | File[]) => void;
  readonly onError?: (error: Error) => void;
}

export class FileInputError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'FileInputError';
  }
}

export class FileInputTypeError extends FileInputError {
  constructor(message: string) {
    super(message);
    this.name = 'FileInputTypeError';
  }
}

export class FileInputSizeError extends FileInputError {
  constructor(message: string) {
    super(message);
    this.name = 'FileInputSizeError';
  }
}

/**
 * File input component.
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function FileInput(props: Props): ReactElement {
  const { name, types, className, children, disabled, multiple, required, onSelect, onError } = props;
  const inputRef = useRef<HTMLInputElement>(null);

  const [fileList, setFileList] = useState<FileList | null>(null);
  const [errors, setErrors] = useState<FileInputError[]>([]);

  const accept = useMemo(() => {
    if (types === undefined) {
      return '';
    }
    return types.map((type: string) => type.toLowerCase()).join(',');
  }, [types]);

  useEffect(() => {
    if (!onSelect) {
      return;
    }

    if (fileList === null) {
      return;
    }

    const files: File[] = [];
    const errors: FileInputError[] = [];
    for (let i = 0; i < fileList.length; i++) {
      const file = fileList.item(i);
      if (file === null) {
        continue;
      }

      const supported = (types || []).map((type: string) => type.toLowerCase()).includes(file.type.toLowerCase());
      if (!supported) {
        errors.push(new FileInputTypeError(`File type (${file.type}) of "${file.name}" is not supported.`));
        continue;
      }

      const minSize = props.minSize || Number.MIN_SAFE_INTEGER;
      const maxSize = props.maxSize || Number.MAX_SAFE_INTEGER;
      if (file.size < minSize || file.size > maxSize) {
        errors.push(new FileInputSizeError(`File size of "${file.name}" is not in range [${minSize}, ${maxSize}].`));
        continue;
      }
      files.push(file);
    }

    setErrors(errors);

    if (files.length === 0 || errors.length !== 0) {
      return;
    }

    if (multiple) {
      onSelect(files);
    } else {
      onSelect(files[0]);
    }
  }, [onSelect, fileList]);

  useEffect(() => {
    if (!onError) {
      return;
    }

    if (errors.length !== 0) {
      onError(errors[0]);
    }
  }, [onError, errors]);

  const onLabelClick = useCallback((event: MouseEvent<HTMLLabelElement>) => {
    event.stopPropagation();

    const input = inputRef.current;
    if (input !== null) {
      input.value = '';
      input.click();
    }
  }, []);

  const onFileDrop = useCallback((event: DragEvent<HTMLLabelElement>) => {
    event.preventDefault();

    setFileList(event.dataTransfer.files);
    setErrors([]);
  }, []);

  const onFileChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    event.preventDefault();

    setFileList(event.target.files);
    setErrors([]);
  }, []);

  return (
    <label
      className={`flex flex-row justify-center items-center cursor-pointer rounded border border-dashed border-gray-400 ${
        className || ''
      }`}
      htmlFor={name}
      onClick={onLabelClick}
      onDragOver={(e) => e.preventDefault()}
      onDrop={onFileDrop}
    >
      <input
        ref={inputRef}
        className="hidden"
        name={name}
        type="file"
        accept={accept}
        disabled={disabled}
        required={required}
        multiple={multiple}
        onChange={onFileChange}
      />
      {children}
    </label>
  );
}

export default FileInput;
