'use client';

import { useState } from 'react';
import axios from 'axios';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Card } from '@/components/ui/card';

export default function Home() {
  const [prompt, setPrompt] = useState('');
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    setLoading(true);
    try {
      const res = await axios.post('/api/chat', { prompt });
      setResponse(res.data.response);
    } catch (error) {
      console.error('API error:', error);
      setResponse('Oops! Something went wrong.');
    }
    setLoading(false);
  };

  return (
    <main className="flex flex-col items-center justify-center min-h-screen p-4 bg-gray-100">
      <Card className="w-full max-w-2xl p-6 shadow-lg space-y-4">
        <h1 className="text-3xl font-bold text-center">Welcome to Qompass AI</h1>

        <Textarea
          placeholder="Type your question here..."
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          rows={4}
        />

        <Button onClick={handleSubmit} disabled={loading} className="w-full">
          {loading ? 'Thinking...' : 'Submit'}
        </Button>

        {response && (
          <Card className="mt-4 p-4 bg-white shadow-inner whitespace-pre-line">
            {response}
          </Card>
        )}
      </Card>
    </main>
  );
}

