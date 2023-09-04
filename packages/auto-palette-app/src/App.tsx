import { ReactElement, useEffect } from 'react';

import { Footer, Header, ImageViewer, Sidebar, Toolbar } from './components';
import { useAppDispatch, useAppSelector, useImageData } from './hooks';
import { extractPalette, imageSelector } from './store';

function App(): ReactElement {
  const dispatch = useAppDispatch();
  const imageState = useAppSelector(imageSelector);
  const { imageData } = useImageData(imageState.url);

  useEffect(() => {
    if (!imageData) {
      return;
    }

    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    dispatch(extractPalette({ imageData }));
  }, [dispatch, imageData]);

  return (
    <div className="flex flex-col w-full min-h-screen overflow-hidden">
      <Header className="flex-shrink-0 bg-gray-900" />
      <main className="flex-1 flex flex-row relative">
        <ImageViewer className="flex-1" />
        <Sidebar className="flex-shrink-0 absolute top-0 left-0 z-10 m-4" />
        <Toolbar className="flex-shrink-0 absolute top-0 right-0 z-10 m-4" />
      </main>
      <Footer className="flex-shrink-0 bg-gray-900" />
    </div>
  );
}

export default App;
