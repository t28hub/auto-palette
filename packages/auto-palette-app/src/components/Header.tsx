import { ReactElement } from 'react';
import { FaGithub } from 'react-icons/fa';

/**
 * Component properties for Header.
 */
interface Props {
  readonly className?: string;
}

/**
 * Header component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function Header(props: Props): ReactElement {
  const { className } = props;

  return (
    <header className={`flex justify-center w-full h-14 ${className || ''}`}>
      <div className="flex flex-row justify-start items-center w-full max-w-3xl h-full gap-4">
        <h1 className="flex-shrink-0 text-lg text-slate-50 font-semibold select-none">Auto Palette</h1>
        <nav className="flex-1 flex flex-row justify-end">
          <a
            className="py-2 hover:opacity-60 transition-opacity"
            aria-label="Visit GitHub repository"
            target="_blank"
            rel="noopener noreferrer"
            href="https://github.com/t28hub/auto-palette"
          >
            <FaGithub className="w-6 h-6 text-slate-50" />
          </a>
        </nav>
      </div>
    </header>
  );
}

export default Header;
