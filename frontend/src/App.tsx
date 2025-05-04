import React from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Messages from './components/Messages';
import './App.css';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <header className="App-header">
          <h1>Game Application</h1>
        </header>
        <main>
          <Messages />
        </main>
      </div>
    </QueryClientProvider>
  );
}

export default App;
