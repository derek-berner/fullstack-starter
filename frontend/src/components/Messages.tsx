import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { format } from 'date-fns';
import './Messages.css';

interface Message {
  id: string;
  content: string;
  author: string;
  created_at: string;
}

interface PaginatedMessages {
  messages: Message[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

const fetchMessages = async (page: number, perPage: number): Promise<PaginatedMessages> => {
  const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8000';
  const response = await fetch(`${apiUrl}/messages?page=${page}&per_page=${perPage}`);
  if (!response.ok) {
    throw new Error('Failed to fetch messages');
  }
  return response.json();
};

const Messages: React.FC = () => {
  const [page, setPage] = useState(1);
  const perPage = 5;

  const { data, isLoading, error } = useQuery({
    queryKey: ['messages', page, perPage],
    queryFn: () => fetchMessages(page, perPage),
  });

  if (isLoading) return <div>Loading messages...</div>;
  if (error) return <div>Error loading messages: {(error as Error).message}</div>;

  return (
    <div className="messages-container" data-testid="messages-container">
      <h2>Messages</h2>
      <div className="messages-list">
        {data?.messages.map((message) => (
          <div key={message.id} className="message" data-testid={`message-${message.id}`}>
            <div className="message-header">
              <span className="author" data-testid={`message-author-${message.id}`}>
                {message.author}
              </span>
              <span className="timestamp" data-testid={`message-time-${message.id}`}>
                {format(new Date(message.created_at), 'MMM d, yyyy HH:mm')}
              </span>
            </div>
            <div className="message-content" data-testid={`message-content-${message.id}`}>
              {message.content}
            </div>
          </div>
        ))}
      </div>
      <div className="pagination" data-testid="pagination">
        <button
          onClick={() => setPage((p) => Math.max(1, p - 1))}
          disabled={page === 1}
          data-testid="prev-page"
        >
          Previous
        </button>
        <span data-testid="page-info">
          Page {page} of {data?.total_pages || 1}
        </span>
        <button
          onClick={() => setPage((p) => Math.min(data?.total_pages || 1, p + 1))}
          disabled={page === data?.total_pages}
          data-testid="next-page"
        >
          Next
        </button>
      </div>
    </div>
  );
};

export default Messages; 