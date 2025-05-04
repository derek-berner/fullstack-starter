import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Messages from './Messages';
import '@testing-library/jest-dom';

// Mock the fetch function
const mockFetch = jest.fn();
global.fetch = mockFetch;

const mockMessages = {
  messages: [
    {
      id: '1',
      message: 'Hello, this is a test message!',
      user_id: 'anonymous',
      created_at: '2024-01-01T00:00:00Z',
    },
    {
      id: '2',
      message: 'Another test message',
      user_id: 'anonymous',
      created_at: '2024-01-01T00:01:00Z',
    },
  ],
  total: 10,
  page: 1,
  per_page: 5,
  total_pages: 2,
};

describe('Messages Component', () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
          gcTime: 0,
        },
      },
    });
    mockFetch.mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockMessages),
      })
    );
  });

  afterEach(() => {
    queryClient.clear();
    jest.clearAllMocks();
  });

  const renderMessages = () => {
    return render(
      <QueryClientProvider client={queryClient}>
        <Messages />
      </QueryClientProvider>
    );
  };

  test('should display messages container', async () => {
    renderMessages();
    const container = await screen.findByTestId('messages-container');
    expect(container).toBeInTheDocument();
  });

  test('should display messages', async () => {
    renderMessages();
    const messages = await screen.findAllByTestId(/^message-[0-9]+$/);
    expect(messages).toHaveLength(2);
  });

  test('should display message content', async () => {
    renderMessages();
    const messageContent = await screen.findByTestId('message-content-1');
    expect(messageContent).toHaveTextContent('Hello, this is a test message!');
  });

  test('should display user information', async () => {
    renderMessages();
    const userId = await screen.findByTestId('message-user-1');
    expect(userId).toHaveTextContent('anonymous');
  });

  test('should display pagination controls', async () => {
    renderMessages();
    const pagination = await screen.findByTestId('pagination');
    expect(pagination).toBeInTheDocument();

    const prevButton = screen.getByTestId('prev-page');
    const nextButton = screen.getByTestId('next-page');
    const pageInfo = screen.getByTestId('page-info');

    expect(prevButton).toBeInTheDocument();
    expect(nextButton).toBeInTheDocument();
    expect(pageInfo).toHaveTextContent('Page 1 of 2');
  });

  test('should handle pagination', async () => {
    renderMessages();
    const nextButton = await screen.findByTestId('next-page');
    
    // Mock the response for page 2
    mockFetch.mockImplementationOnce(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          ...mockMessages,
          page: 2,
        }),
      })
    );

    fireEvent.click(nextButton);

    await waitFor(() => {
      const pageInfo = screen.getByTestId('page-info');
      expect(pageInfo).toHaveTextContent('Page 2 of 2');
    });
  });
}); 