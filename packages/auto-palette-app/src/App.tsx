import { ReactElement } from 'react';

import { FileInput, Footer, Header } from './components';

function App(): ReactElement {
  return (
    <div className="flex flex-col w-full h-screen overflow-hidden">
      <Header className="flex-shrink-0 bg-gray-900" />
      <main className="flex-1 flex flex-row relative">
        <div className="flex-shrink-0 flex items-stretch justify-center w-60 h-full absolute top-0 left-0 z-10">
          <div className="flex-1 p-4">
            <FileInput />
          </div>
        </div>
        <div className="flex-1 flex items-center justify-center p-4 bg-gray-100"></div>
        <div className="flex-shrink-0 w-60 h-full absolute top-0 right-0 overflow-x-hidden overflow-y-auto z-10">
          <ul className="p-4">
            <li className="p-4 border-b">Item 1</li>
            <li className="p-4 border-b">Item 2</li>
            <li className="p-4 border-b">Item 3</li>
            <li className="p-4 border-b">Item 4</li>
            <li className="p-4 border-b">Item 5</li>
            <li className="p-4 border-b">Item 6</li>
            <li className="p-4 border-b">Item 7</li>
            <li className="p-4 border-b">Item 8</li>
            <li className="p-4 border-b">Item 9</li>
            <li className="p-4 border-b">Item 10</li>
            <li className="p-4 border-b">Item 11</li>
            <li className="p-4 border-b">Item 12</li>
          </ul>
        </div>
      </main>
      <Footer className="flex-shrink-0 bg-gray-900" />
    </div>
  );
}

export default App;
