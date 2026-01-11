import { createFileRoute } from '@tanstack/react-router'
import { useState, useEffect, useCallback } from 'react'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  ChatMessage,
  ChatInput,
  SessionList,
  SystemPromptDialog,
} from '@/components/chat'
import { api, type Session, type Message } from '@/lib/api'

export const Route = createFileRoute('/')({ component: ChatPage })

function ChatPage() {
  const [sessions, setSessions] = useState<Session[]>([])
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null)
  const [messages, setMessages] = useState<Message[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [showNewSessionDialog, setShowNewSessionDialog] = useState(false)

  // Load session data when session changes
  useEffect(() => {
    if (currentSessionId) {
      api.getSession(currentSessionId).then((data) => {
        setMessages(data.messages)
      })
    } else {
      setMessages([])
    }
  }, [currentSessionId])

  const handleNewSession = useCallback(async (systemPrompt?: string) => {
    try {
      const session = await api.createSession({ system_prompt: systemPrompt })
      setSessions((prev) => [
        { id: session.id, system_prompt: session.system_prompt, created_at: session.created_at },
        ...prev,
      ])
      setCurrentSessionId(session.id)
      setMessages([])
    } catch (error) {
      console.error('Failed to create session:', error)
    }
  }, [])

  const handleDeleteSession = useCallback(
    async (id: string) => {
      try {
        await api.deleteSession(id)
        setSessions((prev) => prev.filter((s) => s.id !== id))
        if (currentSessionId === id) {
          setCurrentSessionId(null)
          setMessages([])
        }
      } catch (error) {
        console.error('Failed to delete session:', error)
      }
    },
    [currentSessionId]
  )

  const handleSendMessage = useCallback(
    async (content: string) => {
      if (!currentSessionId) {
        // Create a new session first
        const session = await api.createSession({})
        setSessions((prev) => [
          { id: session.id, system_prompt: session.system_prompt, created_at: session.created_at },
          ...prev,
        ])
        setCurrentSessionId(session.id)

        // Send message to new session
        setIsLoading(true)
        try {
          const userMessage: Message = {
            id: crypto.randomUUID(),
            session_id: session.id,
            role: 'user',
            content,
            created_at: new Date().toISOString(),
          }
          setMessages([userMessage])

          const response = await api.sendMessage(session.id, content)
          const assistantMessage: Message = {
            id: crypto.randomUUID(),
            session_id: session.id,
            role: 'assistant',
            content: response.response,
            created_at: new Date().toISOString(),
          }
          setMessages((prev) => [...prev, assistantMessage])
        } finally {
          setIsLoading(false)
        }
        return
      }

      setIsLoading(true)
      try {
        // Optimistically add user message
        const userMessage: Message = {
          id: crypto.randomUUID(),
          session_id: currentSessionId,
          role: 'user',
          content,
          created_at: new Date().toISOString(),
        }
        setMessages((prev) => [...prev, userMessage])

        const response = await api.sendMessage(currentSessionId, content)
        const assistantMessage: Message = {
          id: crypto.randomUUID(),
          session_id: currentSessionId,
          role: 'assistant',
          content: response.response,
          created_at: new Date().toISOString(),
        }
        setMessages((prev) => [...prev, assistantMessage])
      } catch (error) {
        console.error('Failed to send message:', error)
        // Remove the optimistic user message on error
        setMessages((prev) => prev.slice(0, -1))
      } finally {
        setIsLoading(false)
      }
    },
    [currentSessionId]
  )

  return (
    <div className="flex h-[calc(100vh-64px)]">
      {/* Sidebar */}
      <div className="w-64 border-r shrink-0">
        <SessionList
          sessions={sessions}
          currentSessionId={currentSessionId ?? undefined}
          onSelectSession={setCurrentSessionId}
          onNewSession={() => setShowNewSessionDialog(true)}
          onDeleteSession={handleDeleteSession}
        />
      </div>

      {/* Main chat area */}
      <div className="flex-1 flex flex-col">
        {currentSessionId ? (
          <>
            <ScrollArea className="flex-1">
              <div className="divide-y">
                {messages.map((message) => (
                  <ChatMessage key={message.id} message={message} />
                ))}
                {isLoading && (
                  <div className="flex gap-3 p-4 bg-background animate-pulse">
                    <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-muted" />
                    <div className="flex-1 space-y-2">
                      <div className="h-4 w-20 bg-muted rounded" />
                      <div className="h-4 w-full bg-muted rounded" />
                    </div>
                  </div>
                )}
              </div>
            </ScrollArea>
            <ChatInput onSend={handleSendMessage} isLoading={isLoading} />
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center">
            <div className="text-center space-y-4">
              <h2 className="text-2xl font-semibold">Welcome to Chat</h2>
              <p className="text-muted-foreground">
                Start a new conversation or select an existing session
              </p>
              <ChatInput
                onSend={handleSendMessage}
                isLoading={isLoading}
              />
            </div>
          </div>
        )}
      </div>

      <SystemPromptDialog
        open={showNewSessionDialog}
        onOpenChange={setShowNewSessionDialog}
        onSubmit={handleNewSession}
      />
    </div>
  )
}
