import { ReactElement } from 'react';

/**
 * Component properties.
 */
type Props = {
  readonly className?: string;
};

/**
 * Footer component.
 *
 * @constructor
 * @param props - Component properties.
 * @return {ReactElement}
 */
function Footer(props: Props): ReactElement {
  const { className } = props;
  const year = new Date().getFullYear();

  return (
    <footer className={`flex justify-center items-center w-full h-14 p-4 ${className || ''}`}>
      <span className="text-sm text-slate-50 font-normal">&copy; {year} Tatsuya Maki. All Rights Reserved.</span>
    </footer>
  );
}

export default Footer;
