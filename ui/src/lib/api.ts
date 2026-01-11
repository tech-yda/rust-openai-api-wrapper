// API types matching the Rust backend

export interface Session {
  id: string
  system_prompt: string | null
  created_at: string
}

export interface Message {
  id: string
  session_id: string
  role: 'user' | 'assistant'
  content: string
  created_at: string
}

export interface SessionWithMessages {
  session: Session
  messages: Message[]
}

export interface CreateSessionRequest {
  system_prompt?: string
}

export interface CreateSessionResponse {
  id: string
  system_prompt: string | null
  created_at: string
}

export interface SessionChatRequest {
  message: string
}

export interface SessionChatResponse {
  response: string
  model: string
  session_id: string
  message_count: number
}

// API client
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080'

export const api = {
  // Sessions
  async createSession(request: CreateSessionRequest): Promise<CreateSessionResponse> {
    const res = await fetch(`${API_BASE_URL}/sessions`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
    })
    if (!res.ok) throw new Error('Failed to create session')
    return res.json()
  },

  async getSession(id: string): Promise<SessionWithMessages> {
    const res = await fetch(`${API_BASE_URL}/sessions/${id}`)
    if (!res.ok) throw new Error('Failed to get session')
    return res.json()
  },

  async deleteSession(id: string): Promise<void> {
    const res = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'DELETE',
    })
    if (!res.ok) throw new Error('Failed to delete session')
  },

  async sendMessage(sessionId: string, message: string): Promise<SessionChatResponse> {
    const res = await fetch(`${API_BASE_URL}/sessions/${sessionId}/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message }),
    })
    if (!res.ok) throw new Error('Failed to send message')
    return res.json()
  },

  // Health check
  async healthCheck(): Promise<{ status: string; version: string }> {
    const res = await fetch(`${API_BASE_URL}/health`)
    if (!res.ok) throw new Error('API is not healthy')
    return res.json()
  },
}
